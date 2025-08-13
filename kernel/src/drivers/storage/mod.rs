//! Storage

mod error;
pub mod nvme;

pub use error::Error;

pub fn init() -> Result<(), Error> {
    nvme::init()?;
    Ok(())
}
