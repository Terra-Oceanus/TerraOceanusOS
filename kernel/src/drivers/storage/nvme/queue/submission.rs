//! Submission

use core::ptr::{self, copy_nonoverlapping, write_volatile};

use crate::memory::physical::allocate;

use super::super::command::Submission as Command;

const ENTRY_SIZE: usize = 64;

pub struct Submission {
    addr: usize,
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

    pub fn init(&mut self, size: u16, doorbell: usize) -> Result<usize, crate::Error> {
        self.addr = allocate(size as usize * ENTRY_SIZE)?;
        self.size = size;
        self.doorbell = doorbell as *mut u32;
        Ok(self.addr)
    }

    pub fn enqueue(&mut self, cmd: &Command) {
        unsafe {
            copy_nonoverlapping(
                cmd as *const Command,
                (self.addr as *mut Command).add((self.tail % self.size) as usize),
                1,
            );
            self.tail = self.tail.wrapping_add(1);
            write_volatile(self.doorbell, (self.tail % self.size) as u32);
        }
    }
}
