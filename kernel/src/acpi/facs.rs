//! Firmware ACPI Control Structure

use crate::{Error, Output, init_check, init_end, init_start};

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
        init_start!();
        self.header.init(*SIGNATURE)?;
        init_end!();
        Ok(())
    }
}

pub fn init() -> Result<(), Error> {
    unsafe {
        init_check!(ADDR);
        FACS::get_ref(ADDR).init()
    }
}
