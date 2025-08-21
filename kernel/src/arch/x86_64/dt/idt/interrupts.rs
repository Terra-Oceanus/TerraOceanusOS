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
    "#DE".output();

    loop {}
}

pub fn debug_exception() {
    "#DB".output();

    loop {}
}

pub fn nmi_interrupt() {
    "NMI".output();

    loop {}
}

pub fn breakpoint() {
    "#BP".output();

    loop {}
}

pub fn overflow() {
    "#OF".output();

    loop {}
}

pub fn bound_range_exceeded() {
    "#BR".output();

    loop {}
}

pub fn invalid_opcode() {
    "#UD".output();

    loop {}
}

pub fn device_not_available() {
    "#NM".output();

    loop {}
}

pub fn double_fault() {
    "#DF".output();

    loop {}
}

pub fn invalid_tss() {
    "#TS".output();

    loop {}
}

pub fn segment_not_present() {
    "#NP".output();

    loop {}
}

pub fn stack_segment_fault() {
    "#SS".output();

    loop {}
}

pub fn general_protection(frame: InterruptFrame, error_code: u64) {
    "\nFault: General Protection Exception ".output();
    if error_code != 0 {
        "with error code(".output();
        error_code.output();
        ") ".output();
    }
    "at ".output();
    frame.cs.output();
    ":".output();
    frame.rip.output();
    ".\n".output();

    loop {}
}

pub fn page_fault() {
    "#PF".output();

    loop {}
}

pub fn x87_fpu_floating_point_error() {
    "#MF".output();

    loop {}
}

pub fn alignment_check() {
    "#AC".output();

    loop {}
}

pub fn machine_check() {
    "#MC".output();

    loop {}
}

pub fn simd_floating_point_exception() {
    "#XM".output();

    loop {}
}

pub fn virtualization_exception() {
    "#VE".output();

    loop {}
}

pub fn control_protection_exception() {
    "#CP".output();

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
