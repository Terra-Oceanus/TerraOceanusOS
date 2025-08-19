//! Storage

mod error;
pub mod nvme;

pub use error::Error;

pub fn init() -> Result<(), crate::Error> {
    nvme::init()?;
    Ok(())
}
