//! Peripheral Component Interconnect Express

use crate::{acpi::mcfg, error::Error};

pub fn init() -> Result<(), Error> {
    mcfg::init()
}
