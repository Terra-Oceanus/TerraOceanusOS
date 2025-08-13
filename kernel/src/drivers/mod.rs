//! Drivers

mod error;
pub mod pcie;
mod storage;

pub use error::Error;

pub fn init() -> Result<(), crate::Error> {
    pcie::init()?;
    storage::init();
    Ok(())
}
