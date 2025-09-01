//! Partition

mod error;
mod gpt;
mod mbr;

pub use error::Error;

pub fn validate() -> Result<(), crate::Error> {
    mbr::validate(super::read(0, 1)?)?;
    gpt::validate(super::read(1, 1)?)?;
    Ok(())
}
