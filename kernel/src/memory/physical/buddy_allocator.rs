//! Buddy Allocator

use super::PAGE_SIZE;

struct PageInfo {
    /// - True: Free
    /// - False: Used
    state: bool,

    order: u8,

    next: usize,
}

pub struct BuddyAllocator {
    page_count: usize,

    max_order: usize,

    page_info: &'static mut [PageInfo],

    free_list: &'static mut [usize],
}
