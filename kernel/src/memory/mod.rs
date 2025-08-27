//! Memory

mod error;
mod physical;

pub use error::Error;

const PAGE_SIZE: usize = 0x1000;

pub fn init(entry: usize, descriptor_size: usize, descriptor_count: usize) -> Result<(), Error> {
    physical::init(entry, descriptor_size, descriptor_count)
}
