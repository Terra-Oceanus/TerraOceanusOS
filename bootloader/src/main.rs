#![no_main]
#![no_std]

use core::{
    arch::asm,
    ptr::{self, write_bytes},
    slice::from_raw_parts_mut,
};
use uefi::{
    CStr16, Event, Status,
    boot::{
        AllocateType, MemoryType, OpenProtocolAttributes, OpenProtocolParams, SearchType,
        allocate_pages, exit_boot_services, get_handle_for_protocol, image_handle,
        locate_handle_buffer, open_protocol, open_protocol_exclusive, wait_for_event,
    },
    entry, guid,
    mem::memory_map::MemoryMap,
    proto::{
        console::gop::{GraphicsOutput, PixelFormat},
        media::{
            file::{File, FileAttribute, FileMode, FileType, RegularFile},
            fs::SimpleFileSystem,
        },
    },
    table::system_table_raw,
};
use xmas_elf::{ElfFile, program::Type};

fn find_kernel() -> Result<RegularFile, Status> {
    const NAME: &str = "kernel";
    let mut buffer = [0u16; NAME.len() + 1];
    let name = CStr16::from_str_with_buf(NAME, &mut buffer).map_err(|_e| Status::LOAD_ERROR)?;

    for &handle in locate_handle_buffer(SearchType::from_proto::<SimpleFileSystem>())
        .map_err(|e| e.status())?
        .iter()
    {
        match open_protocol_exclusive::<SimpleFileSystem>(handle)
            .map_err(|e| e.status())?
            .open_volume()
            .map_err(|e| e.status())?
            .open(name, FileMode::Read, FileAttribute::empty())
        {
            Ok(h) => match h.into_type().map_err(|e| e.status())? {
                FileType::Regular(f) => return Ok(f),
                FileType::Dir(_) => continue,
            },
            Err(_) => continue,
        }
    }
    Err(Status::LOAD_ERROR)
}

fn load_kernel() -> Result<usize, Status> {
    let mut kernel = find_kernel()?;

    const HEADER_SIZE: usize = 64;
    const PROGRAM_HEADER_COUNT: usize = 3;
    const PROGRAM_HEADER_SIZE: usize = 56;
    let mut buffer = [0u8; HEADER_SIZE + PROGRAM_HEADER_COUNT * PROGRAM_HEADER_SIZE];
    let read_size = kernel.read(&mut buffer).map_err(|e| e.status())?;
    let elf = ElfFile::new(&buffer).map_err(|_e| Status::LOAD_ERROR)?;

    // Check
    let ph_count = elf.header.pt2.ph_count() as usize;
    let ph_size = elf.header.pt2.ph_entry_size() as usize;
    if PROGRAM_HEADER_COUNT != ph_count
        || PROGRAM_HEADER_SIZE != ph_size
        || read_size != HEADER_SIZE + ph_count * ph_size
    {
        return Err(Status::LOAD_ERROR);
    }

    for ph in elf.program_iter() {
        if ph.get_type().map_err(|_e| Status::LOAD_ERROR)? != Type::Load {
            continue;
        }

        let offset = ph.offset();
        let physical_addr = ph.physical_addr();
        let file_size = ph.file_size() as usize;
        let mem_size = ph.mem_size() as usize;

        const PAGE_SIZE: usize = 0x1000;
        let pages = allocate_pages(
            AllocateType::Address(physical_addr),
            MemoryType::LOADER_DATA,
            (mem_size + PAGE_SIZE - 1) / PAGE_SIZE,
        )
        .map_err(|e| e.status())?
        .as_ptr();
        if pages as u64 != physical_addr {
            return Err(Status::LOAD_ERROR);
        }

        let mut buffer = unsafe {
            write_bytes(pages, 0, mem_size);
            from_raw_parts_mut(pages, file_size)
        };
        kernel.set_position(offset).map_err(|e| e.status())?;
        let read_size = kernel.read(&mut buffer).map_err(|e| e.status())?;
        if read_size < file_size {
            return Err(Status::LOAD_ERROR);
        }
    }
    Ok(elf.header.pt2.entry_point() as usize)
}

fn get_graphics_output_config() -> Result<(usize, usize, usize, usize), Status> {
    let handle = get_handle_for_protocol::<GraphicsOutput>().map_err(|e| e.status())?;
    let mut protocol = unsafe {
        open_protocol::<GraphicsOutput>(
            OpenProtocolParams {
                handle,
                agent: image_handle(),
                controller: None,
            },
            OpenProtocolAttributes::GetProtocol,
        )
    }
    .map_err(|e| e.status())?;

    let mode_info = protocol.current_mode_info();
    if !matches!(
        mode_info.pixel_format(),
        PixelFormat::Rgb | PixelFormat::Bgr
    ) {
        return Err(Status::LOAD_ERROR);
    }
    let (width, height) = mode_info.resolution();
    let stride = mode_info.stride();

    let mut frame_buffer = protocol.frame_buffer();
    let frame_buffer_base = frame_buffer.as_mut_ptr() as usize;

    Ok((frame_buffer_base, width, height, stride))
}

fn get_rsdp_addr() -> Result<usize, Status> {
    let system_table = unsafe { system_table_raw().ok_or(Status::LOAD_ERROR)?.as_ref() };
    for i in 0..system_table.number_of_configuration_table_entries {
        let table = unsafe {
            system_table
                .configuration_table
                .add(i)
                .as_ref()
                .ok_or(Status::LOAD_ERROR)?
        };
        if table.vendor_guid == guid!("8868e871-e4f1-11d3-bc22-0080c73c8881") {
            return Ok(table.vendor_table as usize);
        }
    }
    Err(Status::LOAD_ERROR)
}

fn wait_for_key_press() -> Result<(), Status> {
    unsafe {
        let stdin = system_table_raw().ok_or(Status::LOAD_ERROR)?.as_ref().stdin;
        let _index = wait_for_event(&mut [
            Event::from_ptr((&*stdin).wait_for_key).ok_or(Status::LOAD_ERROR)?
        ])
        .map_err(|e| e.status())?;
        let _status = ((&*stdin).read_key_stroke)(stdin, ptr::null_mut());
    }
    Ok(())
}

#[entry]
fn main() -> Status {
    match uefi::helpers::init() {
        Ok(_) => {}
        Err(e) => return e.status(),
    };

    let entry = match load_kernel() {
        Ok(addr) => addr,
        Err(e) => return e,
    };
    log::info!("Kernel entry: {:#x}", entry);

    let (frame_buffer_base, width, height, stride) = match get_graphics_output_config() {
        Ok(config) => config,
        Err(e) => return e,
    };
    log::info!("Frame buffer base: {:#x}", frame_buffer_base);
    log::info!("Screen width: {width}, height: {height}, stride: {stride}");

    let rsdp_addr = match get_rsdp_addr() {
        Ok(p) => p,
        Err(e) => return e,
    };
    log::info!("RSDP address: {:#x}", rsdp_addr);

    log::info!("Press any key to continue......");
    match wait_for_key_press() {
        Ok(_) => {}
        Err(e) => return e,
    };

    unsafe {
        let memory_map_owned = exit_boot_services(None);
        let memory_map_meta = memory_map_owned.meta();
        asm!(
            "jmp {}",
            in(reg) entry,
            in("r8") frame_buffer_base,
            in("r9") width,
            in("r10") height,
            in("r11") stride,
            in("r12") rsdp_addr,
            in("r13") memory_map_owned.buffer().as_ptr() as usize,
            in("r14") memory_map_meta.desc_size,
            in("r15") memory_map_meta.entry_count(),
            options(noreturn),
        );
    }
}
