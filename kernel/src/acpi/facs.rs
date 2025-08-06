//! Firmware ACPI Control Structure

use crate::error::Error;

use super::{FromAddr, Header};

pub const SIGNATURE: &[u8; 4] = b"FACS";

static mut ADDR: u64 = 0;

pub fn set_config(addr: u64) {
    unsafe { ADDR = addr }
}

#[repr(C, packed)]
struct FACS {
    header: Header,
}
impl FACS {
    fn init(&self) -> Result<(), Error> {
        self.header.init(*SIGNATURE)?;
        Ok(())
    }
}

pub fn init() -> Result<(), Error> {
    unsafe { FACS::get_ref(ADDR).init() }
}
