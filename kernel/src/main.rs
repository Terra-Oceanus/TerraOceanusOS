#![no_std]
#![no_main]

use core::{arch::asm, panic::PanicInfo};

mod acpi;
mod arch;
mod drivers;
mod error;
mod fs;
mod io;
mod math;
mod mem;
mod types;

use arch::x86_64;
use error::Error;
use io::text::{Output, screen};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(loc) = info.location() {
        "\nfile: ".out();
        loc.file().out();
        " line: ".out();
        (loc.line() as usize).out();
        " column: ".out();
        (loc.column() as usize).out();
    }
    if let Some(msg) = info.message().as_str() {
        " msg: ".out();
        msg.out();
    }
    ".\n".out();

    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let mut frame_buffer_base: usize;
    let mut width: usize;
    let mut height: usize;
    let mut stride: usize;
    let mut rsdp_addr: usize;
    let mut memory_map_entry: usize;
    let mut memory_descriptor_size: usize;
    let mut memory_descriptor_count: usize;
    unsafe {
        asm!(
            "",
            lateout("r8") frame_buffer_base,
            lateout("r9") width,
            lateout("r10") height,
            lateout("r11") stride,
            lateout("r12") rsdp_addr,
            lateout("r13") memory_map_entry,
            lateout("r14") memory_descriptor_size,
            lateout("r15") memory_descriptor_count,
        )
    };
    match init(
        frame_buffer_base,
        width,
        height,
        stride,
        rsdp_addr,
        memory_map_entry,
        memory_descriptor_size,
        memory_descriptor_count,
    ) {
        Ok(_) => screen::clear(),
        Err(e) => {
            e.out();
            loop {}
        }
    }

    unsafe {
        asm!(
            "mov ss, ax",
            "mov ds, ax",
            "mov es, ax",
            "mov fs, ax",
            "mov gs, ax",

            "push {0:r}",
            "lea rax, [rip + 9f]",
            "push rax",
            "retfq",
            "9:",

            "sti",
            in(reg) x86_64::gdt::SegmentSelector::KernelCode as u16,
            in("ax") x86_64::gdt::SegmentSelector::KernelData as u16,
        )
    };

    loop {}
}

fn init(
    frame_buffer_base: usize,
    screen_width: usize,
    screen_height: usize,
    screen_stride: usize,
    rsdp_addr: usize,
    memory_map_entry: usize,
    memory_descriptor_size: usize,
    memory_descriptor_count: usize,
) -> Result<(), Error> {
    io::init(
        frame_buffer_base,
        screen_width,
        screen_height,
        screen_stride,
    );
    acpi::init(rsdp_addr)?;
    x86_64::init()?;
    mem::init(
        memory_map_entry,
        memory_descriptor_size,
        memory_descriptor_count,
    )?;
    drivers::init()
}
