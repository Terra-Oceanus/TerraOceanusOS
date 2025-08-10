//! I/O

use core::ptr::{read_volatile, write_volatile};

use super::{super::idt::Interrupt, lapic};

mod error;

pub use error::Error;

const MAX_IO_APIC_COUNT: usize = 9;
static mut IOAPICS: [Config; MAX_IO_APIC_COUNT] = [Config::null(); MAX_IO_APIC_COUNT];
static mut COUNT: usize = 0;

#[repr(u8)]
enum MemoryMappedRegister {
    /// Read/Write
    /// Index
    /// - Bits 0 ..= 7: Register Address
    /// - Bits 8 ..= 31: Reserved
    RegisterSelect,

    /// Read/Write
    /// Data
    /// - Bits 0 ..= 31: Register Data
    Window = 0x10,
}

#[repr(u8)]
enum Register {
    /// Read/Write
    /// IOAPIC ID
    /// - Bits 0 ..= 23: Reserved
    /// - Bits 24 ..= 27: ID
    /// - Bits 28 ..= 31: Reserved
    ID,

    /// Read-only
    /// IOAPIC Version
    /// - Bits 0 ..= 7: Version
    /// - Bits 8 ..= 15: Reserved
    /// - Bits 16 ..= 23: Maximum Redirection Entry
    /// - Bits 24 ..= 31: Reserved
    Version,

    /// Read-only
    /// IOAPIC Arbitration ID
    /// - Bits 0 ..= 23: Reserved
    /// - Bits 24 ..= 27: Arbitration ID
    /// - Bits 28 ..= 31: Reserved
    ArbitrationID,

    /// Read/Write
    /// Redirection Table
    /// - Bits 0 ..= 7: Interrupt Vector (R/W)
    /// - Bits 8 ..= 10: Delivery Mode (R/W)
    ///   - 000: Fixed
    ///   - 001: Lowest Priority
    ///   - 010: SMI
    ///   - 011: Reserved
    ///   - 100: NMI
    ///   - 101: INIT
    ///   - 110: Reserved
    ///   - 111: ExtINT
    /// - Bit 11: Destination Mode (R/W)
    ///   - 0: Physical Mode
    ///   - 1: Logical Mode
    /// - Bit 12: Delivery Status (RO)
    ///   - 0: IDLE
    ///   - 1: Send Pending
    /// - Bit 13: Interrupt Input Pin Polarity (R/W)
    ///   - 0: High active
    ///   - 1: Low active
    /// - Bit 14: Remote IRR (RO)
    /// - Bit 15: Trigger Mode (R/W)
    ///   - 0: Edge sensitive
    ///   - 1: Level sensitive
    /// - Bit 16: Interrupt Mask (R/W)
    ///   - 0: Not masked
    ///   - 1: Masked
    /// - Bits 17 ..= 55: Reserved
    /// - Bits 56 ..= 63: Destination Field (R/W)
    ///   - Physical: APIC ID
    ///   - Logical: Set of processors
    RedirectionTableEntry = 0x10,
}

#[derive(Clone, Copy)]
pub struct Config {
    addr: u32,
    base: u32,
}
impl Config {
    const fn null() -> Self {
        Self { addr: 0, base: 0 }
    }

    fn init(&self, index: u32, vector: u32) {
        let addr = Register::RedirectionTableEntry as u32 + index * 2;
        self.write(addr, ((self.read(addr) & !0xFF) | vector) & !(1 << 16));
        self.write(addr + 1, lapic::id());
    }

    fn read(&self, index: u32) -> u32 {
        unsafe {
            write_volatile(
                (self.addr + MemoryMappedRegister::RegisterSelect as u32) as *mut u32,
                index,
            );
            read_volatile((self.addr + MemoryMappedRegister::Window as u32) as *const u32)
        }
    }

    fn write(&self, index: u32, data: u32) {
        unsafe {
            write_volatile(
                (self.addr + MemoryMappedRegister::RegisterSelect as u32) as *mut u32,
                index,
            );
            write_volatile(
                (self.addr + MemoryMappedRegister::Window as u32) as *mut u32,
                data,
            );
        }
    }
}

pub fn append(addr: u32, base: u32) -> Result<(), Error> {
    unsafe {
        if COUNT == MAX_IO_APIC_COUNT {
            return Err(Error::MaxCountReached);
        }
        IOAPICS[COUNT] = Config { addr, base };
        COUNT += 1;
    }
    Ok(())
}

pub fn handle_override(src: u8, dst: u32, polarity: u8, trigger_mode: u8) -> Result<(), Error> {
    for i in (0..unsafe { COUNT }).rev() {
        let ioapic = &mut unsafe { IOAPICS }[i];
        if dst < ioapic.base {
            continue;
        }
        if dst >= ioapic.base + (ioapic.read(Register::Version as u32) >> 16) & 0xFF {
            return Err(Error::InvalidGSIIndex);
        }

        let index_reg = (ioapic.addr + MemoryMappedRegister::RegisterSelect as u32) as *mut u32;
        let data_reg = (ioapic.addr + MemoryMappedRegister::Window as u32) as *mut u32;
        let addr = Register::RedirectionTableEntry as u32 + (dst - ioapic.base) * 2;
        return Ok(());
    }
    Err(Error::InvalidGSIIndex)
}

pub fn init() {
    for i in 0..unsafe { COUNT } {
        let ioapic = &mut unsafe { IOAPICS }[i];
        for j in 0..((ioapic.read(Register::Version as u32) >> 16) & 0xFF) {
            match ioapic.base + j {
                1 => ioapic.init(j, Interrupt::Keyboard as u32),
                _ => {}
            }
        }
    }
}
