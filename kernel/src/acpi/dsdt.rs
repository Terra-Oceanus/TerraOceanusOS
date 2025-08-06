//! Differentiated System Description Table

use crate::error::Error;

use super::{FromAddr, Header};

pub const SIGNATURE: &[u8; 4] = b"DSDT";

static mut ADDR: u64 = 0;

pub fn set_config(addr: u64) {
    unsafe { ADDR = addr }
}

#[repr(C, packed)]
struct DSDT {
    header: Header,

    definition_block: [u8; 0],
}
impl DSDT {
    fn init(&self) -> Result<(), Error> {
        self.header.init(*SIGNATURE)?;
        Ok(())
    }
}

pub fn init() -> Result<(), Error> {
    unsafe { DSDT::get_ref(ADDR).init() }
}
