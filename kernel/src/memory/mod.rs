//! Memory

use crate::{Output, error::Error, init_end, init_start};

mod physical;

const PAGE_SIZE: u64 = 0x1000;

pub fn init(entry: usize, descriptor_size: usize, descriptor_count: usize) -> Result<(), Error> {
    init_start!();
    physical::init(entry, descriptor_size, descriptor_count)?;
    init_end!();
    Ok(())
}
