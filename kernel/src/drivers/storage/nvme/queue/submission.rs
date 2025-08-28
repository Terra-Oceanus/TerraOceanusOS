//! Submission

use core::ptr::{self, write_volatile};

use crate::memory::{Memory, physical::allocate};

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

    pub fn tail_cmd(&self) -> &'static mut Command {
        Command::get_mut(self.addr + (self.tail % self.size) as usize * ENTRY_SIZE)
    }

    pub fn enqueue(&mut self) {
        self.tail = self.tail.wrapping_add(1);
        unsafe { write_volatile(self.doorbell, (self.tail % self.size) as u32) };
    }
}
