//! Interrupts

use crate::{
    Output,
    io::{
        port,
        text::{Cursor, keyboard},
    },
};

use super::super::super::apic::lapic::eoi;

#[repr(u8)]
pub enum Interrupt {
    /// #DE
    DivideError,

    /// #DB
    DebugException,

    NMIInterrupt,

    /// #BP
    Breakpoint,

    /// #OF
    Overflow,

    /// #BR
    BOUNDRangeExceeded,

    /// #UD
    ///
    /// Undefined Opcode
    InvalidOpcode,

    /// #NM
    ///
    /// No Math Coprocessor
    DeviceNotAvailable,

    /// #DF
    DoubleFault,

    /// Reserved
    CoprocessorSegmentOverrun,

    /// #TS
    InvalidTSS,

    /// #NP
    SegmentNotPresent,

    /// #SS
    StackSegmentFault,

    /// #GP
    GeneralProtection,

    /// #PF
    PageFault,

    /// #MF
    ///
    /// Math Fault
    X87FPUFloatingPointError = 16,

    /// #AC
    AlignmentCheck,

    /// #MC
    MachineCheck,

    /// #XM
    SIMDFloatingPointException,

    /// #VE
    VirtualizationException,

    /// #CP
    ControlProtectionException,

    Timer = 32,
    Keyboard,
    NVMe,
}

#[repr(C)]
pub struct InterruptFrame {
    rip: u64,
    cs: u64,
    eflags: u64,
    rsp: u64,
    ss: u64,
}

pub fn divide_error() {
    "#DE".out();

    loop {}
}

pub fn debug_exception() {
    "#DB".out();

    loop {}
}

pub fn nmi_interrupt() {
    "NMI".out();

    loop {}
}

pub fn breakpoint() {
    "#BP".out();

    loop {}
}

pub fn overflow() {
    "#OF".out();

    loop {}
}

pub fn bound_range_exceeded() {
    "#BR".out();

    loop {}
}

pub fn invalid_opcode() {
    "#UD".out();

    loop {}
}

pub fn device_not_available() {
    "#NM".out();

    loop {}
}

pub fn double_fault() {
    "#DF".out();

    loop {}
}

pub fn invalid_tss() {
    "#TS".out();

    loop {}
}

pub fn segment_not_present() {
    "#NP".out();

    loop {}
}

pub fn stack_segment_fault() {
    "#SS".out();

    loop {}
}

pub fn general_protection(frame: InterruptFrame, error_code: u64) {
    "\nFault: General Protection Exception ".out();
    if error_code != 0 {
        "with error code(".out();
        error_code.out();
        ") ".out();
    }
    "at ".out();
    frame.cs.out();
    ":".out();
    frame.rip.out();
    ".\n".out();

    loop {}
}

pub fn page_fault() {
    "#PF".out();

    loop {}
}

pub fn x87_fpu_floating_point_error() {
    "#MF".out();

    loop {}
}

pub fn alignment_check() {
    "#AC".out();

    loop {}
}

pub fn machine_check() {
    "#MC".out();

    loop {}
}

pub fn simd_floating_point_exception() {
    "#XM".out();

    loop {}
}

pub fn virtualization_exception() {
    "#VE".out();

    loop {}
}

pub fn control_protection_exception() {
    "#CP".out();

    loop {}
}

static mut TICK: usize = 0;
pub fn timer() {
    unsafe {
        TICK += 1;
        if TICK == 50 {
            Cursor::show();
        } else if TICK == 100 {
            Cursor::read_cache();
            TICK = 0;
        }
    }
    eoi();
}

pub fn keyboard() {
    keyboard::input(port::in_byte(port::PS2_DATA));
    eoi();
}

pub fn nvme() {
    eoi();
}
