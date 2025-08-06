//! Memory

use crate::error::Error;

mod physical;

const PAGE_SIZE: u64 = 0x1000;

pub fn init(entry: usize, descriptor_size: usize, descriptor_count: usize) -> Result<(), Error> {
    physical::init(entry, descriptor_size, descriptor_count)
}
