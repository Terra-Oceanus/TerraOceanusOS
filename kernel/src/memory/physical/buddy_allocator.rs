//! Buddy Allocator

use core::slice::from_raw_parts_mut;

use super::PAGE_SIZE;

static mut BUDDY_ALLOCATOR: BuddyAllocator = BuddyAllocator::null();

#[repr(C, packed)]
struct PageInfo {
    /// - True: Free
    /// - False: Used
    state: bool,

    order: u8,

    next: usize,
}
impl PageInfo {
    fn null() -> Self {
        Self {
            state: true,
            order: 0,
            next: 0,
        }
    }
}

#[repr(C, packed)]
pub struct BuddyAllocator {
    page_count: u32,

    max_order: u8,

    page_info: &'static mut [PageInfo],

    free_list: &'static mut [usize],
}
impl BuddyAllocator {
    const fn null() -> Self {
        Self {
            page_count: 0,
            max_order: 0,
            page_info: &mut [],
            free_list: &mut [],
        }
    }

    pub fn pre_init(memory_size: usize) -> usize {
        unsafe {
            BUDDY_ALLOCATOR.page_count = ((memory_size + PAGE_SIZE - 1) / PAGE_SIZE) as u32;
            BUDDY_ALLOCATOR.max_order = {
                let mut order = 0;
                let mut pages = BUDDY_ALLOCATOR.page_count;
                while pages > 1 {
                    pages >>= 1;
                    order += 1;
                }
                order
            };
            size_of::<PageInfo>() * BUDDY_ALLOCATOR.page_count as usize
                + size_of::<usize>() * (BUDDY_ALLOCATOR.max_order as usize + 1)
        }
    }

    pub fn init(addr: u64, size: usize) {
        unsafe {
            BUDDY_ALLOCATOR.page_info =
                from_raw_parts_mut(addr as *mut PageInfo, BUDDY_ALLOCATOR.page_count as usize);
            for i in 0..(BUDDY_ALLOCATOR.page_count as usize) {
                BUDDY_ALLOCATOR.page_info[i] = PageInfo::null();
                if i >= (addr as usize) / PAGE_SIZE
                    && i < (addr as usize + size + PAGE_SIZE - 1) / PAGE_SIZE
                {
                    BUDDY_ALLOCATOR.page_info[i].state = false;
                }
            }

            BUDDY_ALLOCATOR.free_list = from_raw_parts_mut(
                (addr + size_of::<PageInfo>() as u64 * BUDDY_ALLOCATOR.page_count as u64)
                    as *mut usize,
                BUDDY_ALLOCATOR.max_order as usize + 1,
            );
            for i in 0..=(BUDDY_ALLOCATOR.max_order as usize) {
                BUDDY_ALLOCATOR.free_list[i] = 0;
            }
        }
    }
}
