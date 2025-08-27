//! Capability

use crate::Memory;

mod capability;
pub mod extended;
mod msi_x;

pub use msi_x::MSIX;

#[repr(C)]
pub struct Header {
    capability_id: u8,

    next_capability_pointer: u8,
}
impl Memory for Header {}
impl Header {
    pub fn id(&self) -> u8 {
        self.capability_id
    }

    pub fn next(&self) -> usize {
        self.next_capability_pointer as usize
    }
}
