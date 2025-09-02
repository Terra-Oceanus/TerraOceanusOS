//! Storage

mod error;
pub mod nvme;
mod partition;

pub use error::Error;

pub fn init() -> Result<(), crate::Error> {
    nvme::init()?;
    partition::validate()
}

pub fn read(start: u64, size: usize) -> Result<usize, crate::Error> {
    nvme::read(start, size)
}
