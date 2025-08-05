//! Differentiated System Description Table

use crate::{Error, Output, init_check, init_end, init_start};

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
        init_start!();
        self.header.init(*SIGNATURE)?;
        init_end!();
        Ok(())
    }
}

pub fn init() -> Result<(), Error> {
    unsafe {
        init_check!(ADDR);
        DSDT::get_ref(ADDR).init()
    }
}
