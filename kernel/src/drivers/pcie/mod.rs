//! Peripheral Component Interconnect Express

use crate::{acpi::mcfg, traits::FromAddr};

pub mod error;
mod type0;
mod type1;
mod type2;

pub use error::Error;

#[repr(C, packed)]
pub struct Header {
    vendor_id: u16,
    device_id: u16,

    /// - Bit 0: I/O Space (R/W)
    /// - Bit 1: Memory Space (R/W)
    /// - Bit 2: Bus Master (R/W)
    /// - Bit 3: Special Cycles (RO)
    /// - Bit 4: Memory Write and Invalidate Enable (RO)
    /// - Bit 5: VGA Palette Snoop (RO)
    /// - Bit 6: Parity Error Response (R/W)
    /// - Bit 7: Reserved (RO)
    /// - Bit 8: SERR# Enable (R/W)
    /// - Bit 9: Fast Back-to-Back Enable (RO)
    /// - Bit 10: Interrupt Disable (R/W)
    /// - Bits 11 ..= 15: Reserved
    command: u16,

    /// - Bits 0 ..= 2: Reserved
    /// - Bit 3: Interrupt Status (RO)
    /// - Bit 4: Capabilities List (RO)
    /// - Bit 5: 66 MHz Capable (RO)
    /// - Bit 6: Reserved
    /// - Bit 7: Fast Back-to-Back Capable	 (RO)
    /// - Bit 8: Master Data Parity Error (R/W1C)
    /// - Bits 9 ..= 10: DEVSEL Timing (RO)
    /// - Bit 11: Signaled Target Abort (R/W1C)
    /// - Bit 12: Received Target Abort (R/W1C)
    /// - Bit 13: Received Master Abort (R/W1C)
    /// - Bit 14: Signaled System Error (R/W1C)
    /// - Bit 15: Detected Parity Error (R/W1C)
    status: u16,

    revision_id: u8,

    /// Programming Interface
    prog_if: u8,

    subclass: u8,
    class: u8,

    cache_line_size: u8,

    latency_timer: u8,

    /// - Bits 0 ..= 6: Header Type
    /// - Bit 7: MF
    /// - - 0: Single Function
    /// - - 1: Multiple Functions
    header_type: u8,

    built_in_self_test: u8,
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
            0 => type0::handle(self as *const Self as u64),
            1 => Ok(()),
            2 => Ok(()),
            _ => Err(Error::InvalidHeaderType),
        }
    }
}

pub fn init() -> Result<(), crate::Error> {
    mcfg::init()?;
    Ok(())
}
