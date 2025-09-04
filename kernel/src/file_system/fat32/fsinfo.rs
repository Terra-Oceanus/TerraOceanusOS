//! FSInfo

use crate::memory::Memory;

#[repr(C)]
pub struct FSI {
    /// - 0x41615252
    lead_sig: u32,

    reserved0: [u8; 480],

    /// - 0x61417272
    struc_sig: u32,

    /// - 0xFFFFFFFF: Reserved
    nxt_free: u32,

    reserved1: [u8; 12],

    /// 0xAA550000
    trail_sig: u32,
}
impl Memory for FSI {}
impl FSI {
    pub fn validate(&self) -> bool {
        self.lead_sig == 0x41615252
            && self.reserved0 == [0; 480]
            && self.struc_sig == 0x51417272
            && self.reserved1 == [0; 12]
            && self.trail_sig == 0xAA550000
    }
}
