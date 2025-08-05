//! Local

use core::ptr::{read_volatile, write_volatile};

use crate::error::Error;

mod timer;

static mut ADDR: u32 = 0;

#[repr(u16)]
enum Local {
    /// Read/Write
    /// Local APIC ID Register
    /// - Bits 0 ..= 23: Reserved
    /// - Bits 24 ..= 31: ID
    ID = 0x20,

    /// Read-only
    /// Local APIC Version Register
    /// - Bits 0 ..= 7: Version
    /// - Bits 8 ..= 15: Reserved
    /// - Bits 16 ..= 23: Max LVT Entry
    /// - Bit 24: Support for EOI-broadcast suppression
    /// - Bits 25 ..= 31: Reserved
    Version = 0x30,

    /// Read/Write
    /// Task Priority Register
    TPR = 0x80,

    /// Read-only
    /// Arbitration Priority Register
    APR = 0x90,

    /// Read-only
    /// Processor Priority Register
    PPR = 0xA0,

    /// Write-only
    /// End of Interrupt Register
    EOI = 0xB0,

    /// Read-only
    /// Remote Read Register
    RRD = 0xC0,

    /// Read/Write
    /// Logical Destination Register
    LDR = 0xD0,

    /// Read/Write
    /// Destination Format Register
    DFR = 0xE0,

    /// Read/Write
    /// Spurious Interrupt Vector Register
    /// - Bits 0 ..= 7: Spurious Interrupt Vector
    /// - Bit 8: APIC Software Enable/Disable
    ///   - 0: APIC Disabled
    ///   - 1: APIC Enabled
    /// - Bit 9: Focus Processor Checking
    ///   - 0: Enabled
    ///   - 1: Disabled
    /// - Bits 10 ..= 11: Reserved
    /// - Bit 12: EOI-Broadcast Suppression
    ///   - 0: Disabled
    ///   - 1: Enabled
    /// - Bits 13 ..= 31: Reserved
    SIVR = 0xF0,

    /// Read-only
    /// In-Service Register
    ISR0 = 0x100,
    ISR1 = 0x110,
    ISR2 = 0x120,
    ISR3 = 0x130,
    ISR4 = 0x140,
    ISR5 = 0x150,
    ISR6 = 0x160,
    ISR7 = 0x170,

    /// Read-only
    /// Trigger Mode Register
    TMR0 = 0x180,
    TMR1 = 0x190,
    TMR2 = 0x1A0,
    TMR3 = 0x1B0,
    TMR4 = 0x1C0,
    TMR5 = 0x1D0,
    TMR6 = 0x1E0,
    TMR7 = 0x1F0,

    /// Read-only
    /// Interrupt Request Register
    IRR0 = 0x200,
    IRR1 = 0x210,
    IRR2 = 0x220,
    IRR3 = 0x230,
    IRR4 = 0x240,
    IRR5 = 0x250,
    IRR6 = 0x260,
    IRR7 = 0x270,

    /// Write/Read
    /// Error Status Register
    ESR = 0x280,

    /// Read/Write
    /// Corrected Machine Check Interrupt Register
    /// - Bits 0 ..= 7: Interrupt Vector
    /// - Bits 8 ..= 10: Delivery Mode
    ///   - 000: Fixed
    ///   - 010: SMI
    ///   - 100: NMI
    ///   - 111: ExtINT
    ///   - 101: INIT
    /// - Bit 11: Reserved
    /// - Bit 12: Delivery Status
    ///   - 0: Idle
    ///   - 1: Send Pending
    /// - Bits 13 ..= 15: Reserved
    /// - Bit 16: Mask
    ///   - 0: Not Masked
    ///   - 1: Masked
    /// - Bits 17 ..= 31: Reserved
    CMCIR = 0x2F0,

    /// Read/Write
    /// Interrupt Command Register
    ICR0 = 0x300,
    ICR1 = 0x310,

    /// Read/Write
    /// Timer Register
    /// - Bits 0 ..= 7: Interrupt Vector
    /// - Bits 8 ..= 11: Reserved
    /// - Bit 12: Delivery Status
    ///   - 0: Idle
    ///   - 1: Send Pending
    /// - Bits 13 ..= 15: Reserved
    /// - Bit 16: Mask
    ///   - 0: Not Masked
    ///   - 1: Masked
    /// - Bits 17 ..= 18: Timer Mode
    ///   - 00: One-shot
    ///   - 01: Periodic
    ///   - 10: TSC-Deadline
    /// - Bits 19 ..= 31: Reserved
    Timer = 0x320,

    /// Read/Write
    /// Thermal Sensor Register
    /// - Bits 0 ..= 7: Interrupt Vector
    /// - Bits 8 ..= 10: Delivery Mode
    ///   - 000: Fixed
    ///   - 010: SMI
    ///   - 100: NMI
    ///   - 111: ExtINT
    ///   - 101: INIT
    /// - Bit 11: Reserved
    /// - Bit 12: Delivery Status
    ///   - 0: Idle
    ///   - 1: Send Pending
    /// - Bits 13 ..= 15: Reserved
    /// - Bit 16: Mask
    ///   - 0: Not Masked
    ///   - 1: Masked
    /// - Bits 17 ..= 31: Reserved
    TSR = 0x330,

    /// Read/Write
    /// Performance Monitoring Counters Register
    /// - Bits 0 ..= 7: Interrupt Vector
    /// - Bits 8 ..= 10: Delivery Mode
    ///   - 000: Fixed
    ///   - 010: SMI
    ///   - 100: NMI
    ///   - 111: ExtINT
    ///   - 101: INIT
    /// - Bit 11: Reserved
    /// - Bit 12: Delivery Status
    ///   - 0: Idle
    ///   - 1: Send Pending
    /// - Bits 13 ..= 15: Reserved
    /// - Bit 16: Mask
    ///   - 0: Not Masked
    ///   - 1: Masked
    /// - Bits 17 ..= 31: Reserved
    PMCR = 0x340,

    /// Read/Write
    /// LINT0 Register
    /// - Bits 0 ..= 7: Interrupt Vector
    /// - Bits 8 ..= 10: Delivery Mode
    ///   - 000: Fixed
    ///   - 010: SMI
    ///   - 100: NMI
    ///   - 111: ExtINT
    ///   - 101: INIT
    /// - Bit 11: Reserved
    /// - Bit 12: Delivery Status
    ///   - 0: Idle
    ///   - 1: Send Pending
    /// - Bit 13: Interrupt Input Pin Polarity
    /// - Bit 14: Remote IRR
    /// - Bit 15: Trigger Mode
    ///   - 0: Edge
    ///   - 1: Level
    /// - Bit 16: Mask
    ///   - 0: Not Masked
    ///   - 1: Masked
    /// - Bits 17 ..= 31: Reserved
    LINT0 = 0x350,

    /// Read/Write
    /// LINT1 Register
    /// - Bits 0 ..= 7: Interrupt Vector
    /// - Bits 8 ..= 10: Delivery Mode
    ///   - 000: Fixed
    ///   - 010: SMI
    ///   - 100: NMI
    ///   - 111: ExtINT
    ///   - 101: INIT
    /// - Bit 11: Reserved
    /// - Bit 12: Delivery Status
    ///   - 0: Idle
    ///   - 1: Send Pending
    /// - Bit 13: Interrupt Input Pin Polarity
    /// - Bit 14: Remote IRR
    /// - Bit 15: Trigger Mode
    ///   - 0: Edge
    ///   - 1: Level
    /// - Bit 16: Mask
    ///   - 0: Not Masked
    ///   - 1: Masked
    /// - Bits 17 ..= 31: Reserved
    LINT1 = 0x360,

    /// Read/Write
    /// Error Register
    /// - Bits 0 ..= 11: Reserved
    /// - Bit 12: Delivery Status
    ///   - 0: Idle
    ///   - 1: Send Pending
    /// - Bits 13 ..= 15: Reserved
    /// - Bit 16: Mask
    ///   - 0: Not Masked
    ///   - 1: Masked
    /// - Bits 17 ..= 31: Reserved
    Error = 0x370,

    /// Read/Write
    /// Timer Initial Count Register
    TICR = 0x380,

    /// Read Only
    /// Timer Current Count Register
    TCCR = 0x390,

    /// Read/Write
    /// Timer Divide Configuration Register
    /// - Bits 0 ..= 1 & 3: Divide Value
    ///   - 000: Divide by 2
    ///   - 001: Divide by 4
    ///   - 010: Divide by 8
    ///   - 011: Divide by 16
    ///   - 100: Divide by 32
    ///   - 101: Divide by 64
    ///   - 110: Divide by 128
    ///   - 111: Divide by 1
    /// - Bit 2: 0
    /// - Bits 4 ..= 31: Reserved
    TDCR = 0x3E0,
}

#[inline(always)]
fn read(reg: Local) -> u32 {
    unsafe { read_volatile((ADDR + reg as u32) as *const u32) }
}

#[inline(always)]
fn write(reg: Local, value: u32) {
    unsafe { write_volatile((ADDR + reg as u32) as *mut u32, value) }
}

pub fn init(addr: u32) -> Result<(), Error> {
    unsafe { ADDR = addr };
    let mut sivr = read(Local::SIVR);
    if (sivr >> 8) & 1 == 0 {
        sivr |= 1 << 8;
        write(Local::SIVR, sivr);
    };
    timer::init();
    Ok(())
}

pub fn id() -> u32 {
    read(Local::ID)
}

pub fn eoi() {
    write(Local::EOI, 0);
}
