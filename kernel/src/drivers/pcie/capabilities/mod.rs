//! Capability

use crate::traits::FromAddr;

mod capability;
pub mod extended;
pub mod msi_x;

#[repr(C)]
pub struct Header {
    capability_id: u8,

    next_capability_pointer: u8,
}
impl FromAddr for Header {}
impl Header {
    pub fn id(&self) -> u8 {
        self.capability_id
    }

    pub fn next(&self) -> u64 {
        self.next_capability_pointer as u64
    }
}
