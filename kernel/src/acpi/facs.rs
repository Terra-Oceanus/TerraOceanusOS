//! Firmware ACPI Control Structure

use crate::Memory;

use super::{Error, Header};

pub const SIGNATURE: &[u8; 4] = b"FACS";

static mut ADDR: u64 = 0;

pub fn set_config(addr: u64) {
    unsafe { ADDR = addr }
}

#[repr(C, packed)]
struct FACS {
    header: Header,
}
impl Memory for FACS {}
impl FACS {
    fn init(&self) -> Result<(), Error> {
        self.header.init(*SIGNATURE)
    }
}

pub fn init() -> Result<(), Error> {
    unsafe { FACS::get_ref(ADDR).init() }
}
