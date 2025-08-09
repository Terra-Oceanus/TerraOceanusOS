//! Drivers

use crate::error::Error;

mod pcie;
mod storage;

pub fn init() -> Result<(), Error> {
    pcie::init()
}