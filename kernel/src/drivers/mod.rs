//! Drivers

mod error;
pub mod pcie;
pub mod storage;

pub use error::Error;

pub fn init() -> Result<(), crate::Error> {
    pcie::init()?;
    storage::init()
}
