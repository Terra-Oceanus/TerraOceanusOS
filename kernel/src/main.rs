#![no_std]
#![no_main]

use core::{arch::asm, panic::PanicInfo};

mod acpi;
mod arch;
mod error;
mod io;
mod macros;

use arch::x86_64;
use error::Error;
use io::text::Output;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(loc) = info.location() {
        "\nfile: ".output();
        loc.file().output();
        " line: ".output();
        (loc.line() as usize).output();
        " column: ".output();
        (loc.column() as usize).output();
    }
    if let Some(msg) = info.message().as_str() {
        " msg: ".output();
        msg.output();
    }
    ".\n".output();

    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let mut frame_buffer_base: u64;
    let mut width: usize;
    let mut height: usize;
    let mut stride: usize;
    let mut rsdp_addr: u64;
    unsafe {
        asm!(
            "",
            lateout("r8") frame_buffer_base,
            lateout("r9") width,
            lateout("r10") height,
            lateout("r11") stride,
            lateout("r12") rsdp_addr,
        )
    };
    init(frame_buffer_base, width, height, stride, rsdp_addr).unwrap_or_else(|e| e.output());

    "Switching segment...".output();
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
            in(reg) x86_64::gdt::SegmentSelector::KernelCode as u16,
            in("ax") x86_64::gdt::SegmentSelector::KernelData as u16,
        )
    };
    "finished.\n".output();

    "Setting Interrupt Flag...".output();
    unsafe { asm!("sti") };
    "finished.\n".output();

    loop {}
}

fn init(
    frame_buffer_base: u64,
    screen_width: usize,
    screen_height: usize,
    screen_stride: usize,
    rsdp_addr: u64,
) -> Result<(), Error> {
    io::init(
        frame_buffer_base,
        screen_width,
        screen_height,
        screen_stride,
    );
    acpi::init(rsdp_addr)?;
    x86_64::init()?;
    init_end!();
    Ok(())
}
