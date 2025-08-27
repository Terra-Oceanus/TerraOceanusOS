//! Extended

use crate::Memory;

#[repr(C)]
pub struct Header {
    extended_capability_id: u16,

    /// - Bits 0 ..= 3: Capability Version
    /// - Bits 4 ..= 15: Next Capability Offset
    info: u16,
}
impl Memory for Header {}
impl Header {
    pub fn id(&self) -> u16 {
        self.extended_capability_id
    }

    pub fn version(&self) -> u8 {
        (self.info & 0xF) as u8
    }

    pub fn next(&self) -> u64 {
        (self.info >> 4) as u64
    }
}
