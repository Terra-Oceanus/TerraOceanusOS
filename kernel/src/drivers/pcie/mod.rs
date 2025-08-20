//! Peripheral Component Interconnect Express

use crate::{acpi::mcfg, traits::FromAddr};

mod capabilities;
pub mod error;
mod type0;
mod type1;

pub use error::Error;

#[repr(C, packed)]
pub struct Header {
    /// - 0xFFFF for no Function
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
impl FromAddr for Header {}
impl Header {
    pub fn is_present(&self) -> bool {
        self.vendor_id != 0xFFFF
    }

    pub fn is_multi_function(&self) -> bool {
        (self.header_type & 0b10000000) != 0
    }

    pub fn handle(&self) -> Result<(), Error> {
        match self.header_type & 0b01111111 {
            0 => Ok(type0::handle(self as *const Self as u64)),
            1 => Ok(()),
            _ => Err(Error::InvalidHeaderType),
        }
    }
}

pub fn init() -> Result<(), crate::Error> {
    mcfg::init()
}
