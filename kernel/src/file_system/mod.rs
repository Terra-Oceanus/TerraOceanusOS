//! File System

mod error;
mod fat32;

pub use error::Error;

pub fn handle(start: u64, end: u64) -> Result<(), crate::Error> {
    if fat32::handle(start)? {}
    Err(Error::InvalidFileSystem(start as usize, end as usize).into())
}
