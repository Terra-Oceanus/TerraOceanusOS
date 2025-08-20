//! Capability

use crate::traits::FromAddr;

mod capability;
pub mod extended;

#[repr(C)]
struct Header {
    capability_id: u8,

    next_capability_pointer: u8,
}
impl FromAddr for Header {}
