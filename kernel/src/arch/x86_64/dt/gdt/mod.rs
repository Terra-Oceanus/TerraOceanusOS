//! Global Descriptor Table

use core::{arch::asm, ptr::addr_of};

use crate::{Output, init_end, init_message, init_start};

use super::Descriptor;

mod tss;

static mut GDT: Table = Table::new();

#[repr(u8)]
pub enum SegmentSelector {
    KernelCode = 0x08,
    KernelData = 0x10,
}

#[repr(C)]
struct SegmentDescriptor {
    /// Segment Limit 0 ..= 15
    segment_limit_low: u16,

    /// Base Address 0 ..= 15
    base_address_low: u16,

    /// Base Address 16 ..= 23
    base_address_middle: u8,

    /// - Bits 0 ..= 3: Type for Segment type
    /// - Bit 4: S for Descriptor type
    ///   - 0: system
    ///   - 1: code or data
    /// - Bits 5 ..= 6: DPL for Descriptor privilege level
    /// - Bit 7: P for Segment present
    access: u8,

    /// - Bits 0 ..= 3: Segment Limit 16 ..= 19
    /// - Bit 4: AVL for Available for use by system software
    /// - Bit 5: L for 64-bit code segment
    /// - Bit 6: D/B for Default operation size
    ///   - 0: 16-bit segment
    ///   - 1: 32-bit segment
    /// - Bit 7: G for Granularity
    flags: u8,

    /// Base Address 24 ..= 31
    base_address_high: u8,
}
impl SegmentDescriptor {
    const fn null() -> Self {
        Self {
            segment_limit_low: 0,
            base_address_low: 0,
            base_address_middle: 0,
            access: 0,
            flags: 0,
            base_address_high: 0,
        }
    }

    const fn kernel_code() -> Self {
        Self {
            segment_limit_low: 0xFFFF,
            base_address_low: 0,
            base_address_middle: 0,
            access: 0b10011010,
            flags: 0b10101111,
            base_address_high: 0,
        }
    }

    const fn kernel_data() -> Self {
        Self {
            segment_limit_low: 0xFFFF,
            base_address_low: 0,
            base_address_middle: 0,
            access: 0b10010010,
            flags: 0b11001111,
            base_address_high: 0,
        }
    }

    const fn user_code() -> Self {
        Self {
            segment_limit_low: 0xFFFF,
            base_address_low: 0,
            base_address_middle: 0,
            access: 0b11111010,
            flags: 0b10101111,
            base_address_high: 0,
        }
    }

    const fn user_data() -> Self {
        Self {
            segment_limit_low: 0xFFFF,
            base_address_low: 0,
            base_address_middle: 0,
            access: 0b11110010,
            flags: 0b11001111,
            base_address_high: 0,
        }
    }
}

#[repr(C)]
struct ExtendedSegmentDescriptor {
    segment_descriptor: SegmentDescriptor,

    /// Base Address 32 ..= 63
    base_address_extended: u32,

    reserved: u32,
}
impl ExtendedSegmentDescriptor {
    const fn task_state() -> Self {
        Self {
            segment_descriptor: SegmentDescriptor {
                segment_limit_low: 0,
                base_address_low: 0,
                base_address_middle: 0,
                access: 0b10001001,
                flags: 0b00000000,
                base_address_high: 0,
            },
            base_address_extended: 0,
            reserved: 0,
        }
    }
}

#[repr(C)]
struct Table {
    null: SegmentDescriptor,
    kernel_code: SegmentDescriptor,
    kernel_data: SegmentDescriptor,
    user_code: SegmentDescriptor,
    user_data: SegmentDescriptor,
    tss: ExtendedSegmentDescriptor,
}
impl Table {
    const fn new() -> Self {
        Self {
            null: SegmentDescriptor::null(),
            kernel_code: SegmentDescriptor::kernel_code(),
            kernel_data: SegmentDescriptor::kernel_data(),
            user_code: SegmentDescriptor::user_code(),
            user_data: SegmentDescriptor::user_data(),
            tss: ExtendedSegmentDescriptor::task_state(),
        }
    }
}

pub fn init() {
    init_start!();
    let base = tss::get_addr();
    let limit = size_of::<tss::TSS>() as u64;
    unsafe {
        GDT.tss.segment_descriptor.segment_limit_low = limit as u16;
        GDT.tss.segment_descriptor.base_address_low = base as u16;
        GDT.tss.segment_descriptor.base_address_middle = (base >> 16) as u8;
        GDT.tss.segment_descriptor.flags |= (limit >> 16) as u8;
        GDT.tss.segment_descriptor.base_address_high = (base >> 24) as u8;
        GDT.tss.base_address_extended = (base >> 32) as u32;
    }
    init_message!(true, false, "Loading GDT...");
    unsafe {
        asm!(
            "lgdt [{}]",
            in(reg) &Descriptor::new::<Table>(addr_of!(GDT) as u64),
        )
    };
    init_message!(false, true, "finished");
    init_end!();
}
