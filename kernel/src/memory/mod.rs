//! Memory

use crate::{Output, init_end, init_start};

#[repr(C)]
pub struct Descriptor {
    type_: u32,

    phys_start: u64,
    virt_start: u64,

    page_count: u64,

    attributes: u64,
}

pub fn init(entry: usize, descriptor_size: usize, descriptor_count: usize) {
    init_start!();
    init_end!();
}
