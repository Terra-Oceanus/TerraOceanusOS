//! Buddy Allocator

use core::slice::from_raw_parts_mut;

use crate::math::Math;

use super::{Error, PAGE_SIZE};

static mut BUDDY_ALLOCATOR: BuddyAllocator = BuddyAllocator::null();

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
            state: false,
            order: 0,
            next: 0,
        }
    }
}

pub struct BuddyAllocator {
    page_count: usize,

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
            BUDDY_ALLOCATOR.page_count = (memory_size + PAGE_SIZE - 1) / PAGE_SIZE;
            BUDDY_ALLOCATOR.max_order = BUDDY_ALLOCATOR.page_count.log2() as u8;
            size_of::<PageInfo>() * BUDDY_ALLOCATOR.page_count
                + size_of::<usize>() * (BUDDY_ALLOCATOR.max_order as usize + 1)
        }
    }

    pub fn init(addr: usize) {
        unsafe {
            BUDDY_ALLOCATOR.page_info =
                from_raw_parts_mut(addr as *mut PageInfo, BUDDY_ALLOCATOR.page_count);
            for i in 0..(BUDDY_ALLOCATOR.page_count) {
                BUDDY_ALLOCATOR.page_info[i] = PageInfo::null();
            }

            BUDDY_ALLOCATOR.free_list = from_raw_parts_mut(
                (addr + size_of::<PageInfo>() * BUDDY_ALLOCATOR.page_count) as *mut usize,
                BUDDY_ALLOCATOR.max_order as usize + 1,
            );
            for i in 0..=(BUDDY_ALLOCATOR.max_order as usize) {
                BUDDY_ALLOCATOR.free_list[i] = 0;
            }
        }
    }

    pub fn add(addr: usize, mut count: usize) -> Result<(), Error> {
        let mut index = addr / PAGE_SIZE;
        while count > 0 {
            let mut order = unsafe { BUDDY_ALLOCATOR.max_order };
            while order > 0 && ((1 << order) > count || (index & ((1 << order) - 1)) != 0) {
                order -= 1;
            }
            if index + (1 << order) > unsafe { BUDDY_ALLOCATOR.page_count } {
                return Err(Error::InvalidIndex);
            }

            unsafe {
                BUDDY_ALLOCATOR.page_info[index].state = true;
                BUDDY_ALLOCATOR.page_info[index].order = order;
                BUDDY_ALLOCATOR.page_info[index].next = BUDDY_ALLOCATOR.free_list[order as usize];
                BUDDY_ALLOCATOR.free_list[order as usize] = index;
            }

            index += 1 << order;
            count -= 1 << order;
        }
        Ok(())
    }

    pub fn allocate(size: usize) -> Result<usize, Error> {
        let pages = (size + PAGE_SIZE - 1) / PAGE_SIZE;
        if pages == 0 || pages > (1 << unsafe { BUDDY_ALLOCATOR.max_order }) {
            return Err(Error::InvalidAllocationSize);
        }

        let mut order = 0;
        while (1 << order) < pages {
            order += 1;
        }

        let mut current_order = order;
        unsafe {
            while current_order <= BUDDY_ALLOCATOR.max_order {
                let head = BUDDY_ALLOCATOR.free_list[current_order as usize];
                if head != 0 {
                    BUDDY_ALLOCATOR.free_list[current_order as usize] =
                        BUDDY_ALLOCATOR.page_info[head].next;

                    while current_order > order {
                        current_order -= 1;
                        let buddy = head + (1 << current_order);
                        BUDDY_ALLOCATOR.page_info[buddy].state = true;
                        BUDDY_ALLOCATOR.page_info[buddy].order = current_order;
                        BUDDY_ALLOCATOR.page_info[buddy].next =
                            BUDDY_ALLOCATOR.free_list[current_order as usize];
                        BUDDY_ALLOCATOR.free_list[current_order as usize] = buddy;
                    }

                    BUDDY_ALLOCATOR.page_info[head].state = false;
                    BUDDY_ALLOCATOR.page_info[head].order = order;
                    BUDDY_ALLOCATOR.page_info[head].next = 0;

                    return Ok(head * PAGE_SIZE);
                }
                current_order += 1;
            }
        }
        Err(Error::OutOfMemory)
    }

    pub fn deallocate(addr: usize) -> Result<(), Error> {
        let index = addr / PAGE_SIZE;
        unsafe {
            if index >= BUDDY_ALLOCATOR.page_count {
                return Err(Error::InvalidIndex);
            }

            let mut index = index;
            BUDDY_ALLOCATOR.page_info[index].state = true;

            let mut order = BUDDY_ALLOCATOR.page_info[index].order;
            while order < BUDDY_ALLOCATOR.max_order {
                let buddy = index ^ (1 << order);

                if !BUDDY_ALLOCATOR.page_info[buddy].state
                    || BUDDY_ALLOCATOR.page_info[buddy].order != order
                {
                    break;
                }

                let mut prev = 0;
                let mut curr = BUDDY_ALLOCATOR.free_list[order as usize];
                while curr != 0 {
                    if curr == buddy {
                        if prev == 0 {
                            BUDDY_ALLOCATOR.free_list[order as usize] =
                                BUDDY_ALLOCATOR.page_info[curr].next;
                        } else {
                            BUDDY_ALLOCATOR.page_info[prev].next =
                                BUDDY_ALLOCATOR.page_info[curr].next;
                        }
                        break;
                    }
                    prev = curr;
                    curr = BUDDY_ALLOCATOR.page_info[curr].next;
                }

                index = if index < buddy { index } else { buddy };
                order += 1;
            }
            BUDDY_ALLOCATOR.page_info[index].order = order;
            BUDDY_ALLOCATOR.page_info[index].next = BUDDY_ALLOCATOR.free_list[order as usize];
            BUDDY_ALLOCATOR.free_list[order as usize] = index;
        }
        Ok(())
    }
}
