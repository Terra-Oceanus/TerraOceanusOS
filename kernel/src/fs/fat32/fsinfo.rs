//! FSInfo

use crate::mem::Memory;

#[repr(C, packed)]
pub struct FSI {
    /// - 0x41615252
    lead_sig: u32,

    reserved0: [u8; 480],

    /// - 0x61417272
    struc_sig: u32,

    /// Last free cluster
    /// - 0xFFFFFFFF: Reserved
    free_count: u32,

    /// Next free cluster
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
            && self.struc_sig == 0x61417272
            && self.trail_sig == 0xAA550000
            && self.reserved0 == [0; 480]
            && self.reserved1 == [0; 12]
    }
}
