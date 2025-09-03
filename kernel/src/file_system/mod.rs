//! File System

use crate::drivers::storage;

mod error;
mod fat32;

pub use error::Error;

pub fn handle(start: u64, end: u64) -> Result<(), crate::Error> {
    let lba0 = storage::read(start, 1, 0)?;
    if !fat32::validate(lba0) {
        return Err(Error::InvalidFileSystem(start as usize, end as usize).into());
    }

    Ok(())
}
