//! Drivers

use crate::error::Error;

pub mod pcie;
mod storage;

pub fn init() -> Result<(), Error> {
    pcie::init()
}
