//! File System

mod error;
mod fat32;

pub use error::Error;

pub fn handle(start: u64, end: u64) -> Result<(), crate::Error> {
    use crate::drivers::storage::read;

    let lba0 = read(start, 1, 0)?;
    for handler in [fat32::handle] {
        if handler(start, lba0)? {
            return Ok(());
        }
    }
    Err(Error::InvalidFileSystem(start as usize, end as usize).into())
}
