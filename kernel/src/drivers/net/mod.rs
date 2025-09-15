//! Network

use crate::{drivers::pcie, mem::Memory};

static mut PCIE_ADDR: usize = 0;

pub fn set_config(addr: usize) {
    unsafe { PCIE_ADDR = addr };
}

pub fn init() -> Result<(), super::Error> {
    let pcie = pcie::Type0::get_ref(unsafe { PCIE_ADDR });
    match pcie.header.vendor_id() {
        0x8086 => {}
        _ => {}
    }
    Ok(())
}
