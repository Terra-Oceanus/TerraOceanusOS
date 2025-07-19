//! Interrupt Descriptor Table

use core::{
    arch::{asm, naked_asm},
    ptr::addr_of,
};

use crate::{Output, init_end, init_message, init_start};

use super::{Descriptor, gdt};

mod interrupts;

pub use interrupts::Interrupt;

macro_rules! interrupt {
    ($name:ident) => {{
        #[unsafe(naked)]
        unsafe extern "C" fn wrapper() {
            naked_asm!(
                "push r15",
                "push r14",
                "push r13",
                "push r12",
                "push r11",
                "push r10",
                "push r9",
                "push r8",
                "push rbp",
                "push rdi",
                "push rsi",
                "push rdx",
                "push rcx",
                "push rbx",
                "push rax",
                "mov rdi, rsp",
                "add rdi, 8 * 15",
                "call {}",
                "pop rax",
                "pop rbx",
                "pop rcx",
                "pop rdx",
                "pop rsi",
                "pop rdi",
                "pop rbp",
                "pop r8",
                "pop r9",
                "pop r10",
                "pop r11",
                "pop r12",
                "pop r13",
                "pop r14",
                "pop r15",
                "iretq",
                sym interrupts::$name,
            );
        }
        wrapper
    }}
}

macro_rules! exception {
    ($name:ident) => {{
        #[unsafe(naked)]
        unsafe extern "C" fn wrapper() {
            naked_asm!(
                "push r15",
                "push r14",
                "push r13",
                "push r12",
                "push r11",
                "push r10",
                "push r9",
                "push r8",
                "push rbp",
                "push rdi",
                "push rsi",
                "push rdx",
                "push rcx",
                "push rbx",
                "push rax",
                "mov rsi, [rsp + 8 * 15]",
                "mov rdi, rsp",
                "add rdi, 8 * 16",
                "sub rsp, 8",
                "call {}",
                "add rsp, 8",
                "pop rax",
                "pop rbx",
                "pop rcx",
                "pop rdx",
                "pop rsi",
                "pop rdi",
                "pop rbp",
                "pop r8",
                "pop r9",
                "pop r10",
                "pop r11",
                "pop r12",
                "pop r13",
                "pop r14",
                "pop r15",
                "add rsp, 8",
                "iretq",
                sym interrupts::$name,
            );
        }
        wrapper
    }}
}

static mut IDT: [GateDescriptor; 256] = [GateDescriptor::null(); 256];

#[repr(C)]
#[derive(Clone, Copy)]
struct GateDescriptor {
    /// Offset 0 ..= 15
    offset_low: u16,

    segment_selector: u16,

    /// Interrupt Stack Table
    ist: u8,

    /// - Bits 0 ..= 3: Type
    /// - Bit 4: 0
    /// - Bits 5 ..= 6: DPL for Descriptor Privilege Level
    /// - Bit 7: P for Segment Present flag
    type_attributes: u8,

    /// Offset 16 ..= 31
    offset_middle: u16,

    /// Offset 32 ..= 63
    offset_high: u32,

    reserved: u32,
}
impl GateDescriptor {
    const fn null() -> Self {
        Self {
            offset_low: 0,
            segment_selector: 0,
            ist: 0,
            type_attributes: 0,
            offset_middle: 0,
            offset_high: 0,
            reserved: 0,
        }
    }

    fn interrupt(addr: u64) -> Self {
        Self {
            offset_low: addr as u16,
            segment_selector: gdt::SegmentSelector::KernelCode as u16,
            ist: 0,
            type_attributes: 0b10001110,
            offset_middle: (addr >> 16) as u16,
            offset_high: (addr >> 32) as u32,
            reserved: 0,
        }
    }

    fn trap(addr: u64) -> Self {
        Self {
            offset_low: addr as u16,
            segment_selector: gdt::SegmentSelector::KernelCode as u16,
            ist: 0,
            type_attributes: 0b10001111,
            offset_middle: (addr >> 16) as u16,
            offset_high: (addr >> 32) as u32,
            reserved: 0,
        }
    }
}

pub fn init() {
    init_start!();
    unsafe {
        IDT[Interrupt::DivideError as usize] =
            GateDescriptor::interrupt(interrupt!(divide_error) as u64);
        IDT[Interrupt::DebugException as usize] =
            GateDescriptor::interrupt(interrupt!(debug_exception) as u64);
        IDT[Interrupt::NMIInterrupt as usize] =
            GateDescriptor::interrupt(interrupt!(nmi_interrupt) as u64);
        IDT[Interrupt::Breakpoint as usize] =
            GateDescriptor::interrupt(interrupt!(breakpoint) as u64);
        IDT[Interrupt::Overflow as usize] = GateDescriptor::interrupt(interrupt!(overflow) as u64);
        IDT[Interrupt::BOUNDRangeExceeded as usize] =
            GateDescriptor::interrupt(interrupt!(bound_range_exceeded) as u64);
        IDT[Interrupt::InvalidOpcode as usize] =
            GateDescriptor::interrupt(interrupt!(invalid_opcode) as u64);
        IDT[Interrupt::DeviceNotAvailable as usize] =
            GateDescriptor::interrupt(interrupt!(device_not_available) as u64);
        IDT[Interrupt::DoubleFault as usize] =
            GateDescriptor::interrupt(interrupt!(double_fault) as u64);
        IDT[Interrupt::InvalidTSS as usize] =
            GateDescriptor::interrupt(interrupt!(invalid_tss) as u64);
        IDT[Interrupt::SegmentNotPresent as usize] =
            GateDescriptor::interrupt(interrupt!(segment_not_present) as u64);
        IDT[Interrupt::StackSegmentFault as usize] =
            GateDescriptor::interrupt(interrupt!(stack_segment_fault) as u64);
        IDT[Interrupt::GeneralProtection as usize] =
            GateDescriptor::interrupt(exception!(general_protection) as u64);
        IDT[Interrupt::PageFault as usize] =
            GateDescriptor::interrupt(interrupt!(page_fault) as u64);
        IDT[Interrupt::X87FPUFloatingPointError as usize] =
            GateDescriptor::interrupt(interrupt!(x87_fpu_floating_point_error) as u64);
        IDT[Interrupt::AlignmentCheck as usize] =
            GateDescriptor::interrupt(interrupt!(alignment_check) as u64);
        IDT[Interrupt::MachineCheck as usize] =
            GateDescriptor::interrupt(interrupt!(machine_check) as u64);
        IDT[Interrupt::SIMDFloatingPointException as usize] =
            GateDescriptor::interrupt(interrupt!(simd_floating_point_exception) as u64);
        IDT[Interrupt::VirtualizationException as usize] =
            GateDescriptor::interrupt(interrupt!(virtualization_exception) as u64);
        IDT[Interrupt::ControlProtectionException as usize] =
            GateDescriptor::interrupt(interrupt!(control_protection_exception) as u64);
        IDT[Interrupt::Timer as usize] = GateDescriptor::interrupt(interrupt!(timer) as u64);
        IDT[Interrupt::Keyboard as usize] = GateDescriptor::interrupt(interrupt!(keyboard) as u64);
    }
    init_message!(true, false, "Loading IDT...");
    unsafe {
        asm!(
            "lidt [{}]",
            in(reg) &Descriptor::new::<[GateDescriptor; 256]>(addr_of!(IDT) as u64),
        )
    };
    init_message!(false, true, "finished");
    init_end!();
}
