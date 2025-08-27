//! Differentiated System Description Table

use crate::Memory;

use super::{Error, Header};

pub const SIGNATURE: &[u8; 4] = b"DSDT";

static mut ADDR: usize = 0;

pub fn set_config(addr: usize) {
    unsafe { ADDR = addr }
}

#[repr(C, packed)]
struct DSDT {
    header: Header,

    definition_block: [u8; 0],
}
impl Memory for DSDT {}
impl DSDT {
    fn init(&self) -> Result<(), Error> {
        self.header.init(*SIGNATURE)
    }
}

pub fn init() -> Result<(), Error> {
    unsafe { DSDT::get_ref(ADDR).init() }
}
