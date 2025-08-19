//! Completion

use core::ptr;

use crate::memory::physical::allocate;

const ENTRY_SIZE: usize = 16;

pub struct Completion {
    addr: u64,
    size: u16,

    head: u16,
    phase: bool,

    doorbell: *mut u32,
}
impl Completion {
    pub const fn null() -> Self {
        Self {
            addr: 0,
            size: 0,
            head: 0,
            phase: false,
            doorbell: ptr::null_mut(),
        }
    }

    pub fn init(&mut self, id: u16, size: u16) -> Result<u64, crate::Error> {
        self.addr = allocate(size as u64 * ENTRY_SIZE as u64)?;
        self.size = size;
        self.phase = true;
        self.doorbell =
            unsafe { super::super::ADDR + (2 * id as u64 + 1) * (1 << (2 + super::super::DSTRD)) }
                as *mut u32;
        Ok(self.addr)
    }
}
