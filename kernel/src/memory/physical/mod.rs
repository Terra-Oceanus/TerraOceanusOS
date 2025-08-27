//! Physical

use super::{Error, PAGE_SIZE};

mod buddy_allocator;

use buddy_allocator::BuddyAllocator;

#[repr(C)]
struct Descriptor {
    /// - 0: Reserved
    /// - 1: Loader Code
    /// - 2: Loader Data
    /// - 3: Boot Services Code
    /// - 4: Boot Services Data
    /// - 5: Runtime Services Code
    /// - 6: Runtime Services Data
    /// - 7: Conventional
    /// - 8: Unusable
    /// - 9: ACPI Reclaim
    /// - 10: ACPI Non Volatile
    /// - 11: MMIO
    /// - 12: MMIO Port Space
    /// - 13: PAL Code
    /// - 14: Persistent Memory
    /// - 15: Unaccepted
    /// - 16: Max
    type_: u32,

    phys_start: u64,
    virt_start: u64,

    page_count: u64,

    /// - Bit 0: Uncacheable
    /// - Bit 1: Write Combine
    /// - Bit 2: Write Through
    /// - Bit 3: Write Back
    /// - Bit 4: Uncacheable Exported
    /// - Bit 12: Write Protect
    /// - Bit 13: Read Protect
    /// - Bit 14: Execute Protect
    /// - Bit 15: Non Volatile
    /// - Bit 16: More Reliable
    /// - Bit 17: Read Only
    /// - Bit 18: Special Purpose
    /// - Bit 19: CPU Crypto
    /// - Bits 44 ..= 59: ISA Mask
    /// - Bit 62: ISA Valid
    /// - Bit 63: Runtime
    attributes: u64,
}

pub fn init(entry: usize, descriptor_size: usize, descriptor_count: usize) -> Result<(), Error> {
    let mut size = 0;
    for i in 0..descriptor_count {
        let descriptor = unsafe { &*((entry + i * descriptor_size) as *const Descriptor) };
        let phys_end = descriptor.phys_start as usize + PAGE_SIZE * descriptor.page_count as usize;
        if phys_end > size {
            size = phys_end
        };
    }
    let pending_allocation_page_count =
        (BuddyAllocator::pre_init(size) + PAGE_SIZE - 1) / PAGE_SIZE;
    let mut allocate_addr = 0;
    for i in 0..descriptor_count {
        let descriptor = unsafe { &*((entry + i * descriptor_size) as *const Descriptor) };
        if descriptor.type_ != 7 {
            continue;
        }
        if (descriptor.page_count as usize) < pending_allocation_page_count {
            continue;
        }
        allocate_addr = descriptor.phys_start as usize;
        break;
    }
    BuddyAllocator::init(allocate_addr);
    for i in 0..descriptor_count {
        let descriptor = unsafe { &*((entry + i * descriptor_size) as *const Descriptor) };
        if descriptor.type_ != 7 {
            continue;
        }
        if descriptor.phys_start as usize == allocate_addr
            && descriptor.page_count as usize > pending_allocation_page_count
        {
            BuddyAllocator::add(
                descriptor.phys_start as usize + pending_allocation_page_count * PAGE_SIZE,
                descriptor.page_count as usize - pending_allocation_page_count,
            )?;
        } else {
            BuddyAllocator::add(
                descriptor.phys_start as usize,
                descriptor.page_count as usize,
            )?;
        }
    }
    Ok(())
}

pub fn allocate(size: usize) -> Result<usize, Error> {
    BuddyAllocator::allocate(size)
}

pub fn deallocate(addr: usize) -> Result<(), Error> {
    BuddyAllocator::deallocate(addr)
}
