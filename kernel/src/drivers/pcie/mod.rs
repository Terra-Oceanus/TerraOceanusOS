//! Peripheral Component Interconnect Express

use crate::{acpi::mcfg, mem::Memory};

pub mod capabilities;
pub mod error;
mod type0;
mod type1;

pub use error::Error;
pub use type0::Type0;

#[repr(C)]
pub struct Header {
    /// - 0x8086: Intel Corporation
    /// - 0xFFFF: No Function
    vendor_id: u16,

    device_id: u16,

    /// - Bit 0: I/O Space Enable
    /// - Bit 1: Memory Space Enable
    /// - Bit 2: Bus Master Enable
    /// - Bits 3 ..= 5: 0
    /// - Bit 6: Parity Error Response
    /// - Bit 7: 0
    /// - Bit 8: SERR# Enable
    /// - Bit 9: 0
    /// - Bit 10: Interrupt Disable
    /// - Bits 11 ..= 15: Reserved
    command: u16,

    /// - Bit 0: Immediate Readiness
    /// - Bits 1 ..= 2: Reserved
    /// - Bit 3: Interrupt Status
    /// - Bit 4: 1
    /// - Bit 5: 0
    /// - Bit 6: Reserved
    /// - Bit 7: 0
    /// - Bit 8: Master Data Parity Error
    /// - Bits 9 ..= 10: 0
    /// - Bit 11: Signaled Target Abort
    /// - Bit 12: Received Target Abort
    /// - Bit 13: Received Master Abort
    /// - Bit 14: Signaled System Error
    /// - Bit 15: Detected Parity Error
    status: u16,

    revision_id: u8,

    /// - Bits 0 ..= 7: Programming Interface
    /// - Bits 8 ..= 15: Sub-Class Code
    /// - Bits 16 ..= 23: Base Class Code
    class_code: [u8; 3],

    reserved: u16,

    /// - Bits 0 ..= 6: Header Layout
    /// - Bit 7: Multi-Function Device
    ///   - 0: Single Function
    ///   - 1: Multiple Functions
    header_type: u8,

    /// Built-in Self Test
    /// - Bits 0 ..= 3: Completion Code
    /// - Bits 4 ..= 5: Reserved
    /// - Bit 6: Start BIST
    /// - Bit 7: BIST Capable
    bist: u8,
}
impl Memory for Header {}
impl Header {
    pub fn is_present(&self) -> bool {
        self.vendor_id != 0xFFFF
    }

    pub fn is_multi_function(&self) -> bool {
        (self.header_type & 0b1000_0000) != 0
    }

    pub fn handle(&self) -> Result<(), crate::Error> {
        match self.header_type & 0b111_1111 {
            0 => Ok(Type0::get_ref(self as *const _ as usize).handle()),
            1 => Ok(()),
            _ => Err(Error::InvalidHeaderType.into()),
        }
    }

    pub fn vendor_id(&self) -> u16 {
        self.vendor_id
    }

    pub fn device_id(&self) -> u16 {
        self.device_id
    }

    pub fn set_memory_space(&mut self, enable: bool) {
        if enable {
            self.command |= 1 << 1;
        } else {
            self.command &= !(1 << 1);
        }
    }

    pub fn set_bus_master(&mut self, enable: bool) {
        if enable {
            self.command |= 1 << 2;
        } else {
            self.command &= !(1 << 2);
        }
    }

    pub fn set_interrupt(&mut self, enable: bool) {
        if enable {
            self.command &= !(1 << 10);
        } else {
            self.command |= 1 << 10;
        }
    }
}

/// Base Address Register
/// - Memory Space
///   - Bit 0: 0
///   - Bits 1 ..= 2: Type
///     - 0b00: 32-bits
///     - 0b01: Reserved
///     - 0b10: 64-bits
///     - 0b11: Reserved
///   - Bit 3: Undefined
///   - Bits 4 ..= 31: 16-Byte Aligned Base Address
/// - I/O Space
///   - Bit 0: 1
///   - Bit 1: Reserved
///   - Bits 2 ..= 31: 4-Byte Aligned Base Address
pub struct BAR(u32);
impl BAR {
    fn is_memory(&self) -> bool {
        self.0 & 0b1 == 0
    }

    fn is_64bit(&self) -> bool {
        (self.0 >> 1) & 0b11 == 0b10
    }

    fn addr(&self) -> usize {
        (if self.is_memory() {
            self.0 >> 4
        } else {
            self.0 >> 2
        }) as usize
    }
}

pub fn init() -> Result<(), crate::Error> {
    mcfg::init()
}
