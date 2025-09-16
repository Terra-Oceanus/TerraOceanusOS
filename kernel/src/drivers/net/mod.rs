//! Network

use crate::{drivers::pcie, mem::Memory};

mod error;
mod intel_corporation;

pub use error::Error;

static mut PCIE_ADDR: usize = 0;

pub fn set_config(addr: usize) {
    unsafe { PCIE_ADDR = addr };
}

pub fn init() -> Result<(), Error> {
    if unsafe { PCIE_ADDR } == 0 {
        return Err(Error::InvalidAddress("PCIe"));
    }
    let pcie = pcie::Type0::get_mut(unsafe { PCIE_ADDR });
    match pcie.header.vendor_id() {
        0x8086 => intel_corporation::init(pcie),
        _ => {}
    }
    Ok(())
}
