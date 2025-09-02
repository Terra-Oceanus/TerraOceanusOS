//! Partition

mod error;
mod gpt;
mod mbr;

pub use error::Error;

pub fn validate() -> Result<(), crate::Error> {
    mbr::validate()?;
    gpt::validate()?;
    Ok(())
}
