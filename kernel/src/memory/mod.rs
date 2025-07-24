//! Memory

use crate::{Output, init_end, init_start};

mod physical;

pub fn init(entry: usize, descriptor_size: usize, descriptor_count: usize) {
    init_start!();
    physical::init(entry, descriptor_size, descriptor_count);
    init_end!();
}
