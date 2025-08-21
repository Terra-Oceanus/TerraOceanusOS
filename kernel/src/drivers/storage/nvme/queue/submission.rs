//! Submission

use core::ptr::{self, copy_nonoverlapping, write_volatile};

use crate::memory::physical::allocate;

use super::super::command::Submission as Command;

const ENTRY_SIZE: usize = 64;

pub struct Submission {
    addr: u64,
    size: u16,

    tail: u16,

    doorbell: *mut u32,
}
impl Submission {
    pub const fn null() -> Self {
        Self {
            addr: 0,
            size: 0,
            tail: 0,
            doorbell: ptr::null_mut(),
        }
    }

    pub fn init(&mut self, id: u16, size: u16) -> Result<u64, crate::Error> {
        self.addr = allocate(size as u64 * ENTRY_SIZE as u64)?;
        self.size = size;
        self.doorbell =
            unsafe { super::super::ADDR + (2 * id as u64) * (1 << (2 + super::super::DSTRD)) }
                as *mut u32;
        Ok(self.addr)
    }

    pub fn enqueue(&mut self, cmd: &mut Command) {
        unsafe {
            copy_nonoverlapping(
                cmd as *const Command as *const u8,
                (self.addr as *mut u8).add((self.tail % self.size) as usize * ENTRY_SIZE),
                ENTRY_SIZE,
            );
            self.tail = self.tail.wrapping_add(1);
            write_volatile(self.doorbell, (self.tail % self.size) as u32);
        }
    }
}
