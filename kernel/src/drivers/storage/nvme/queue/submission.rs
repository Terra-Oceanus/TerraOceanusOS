//! Submission

use core::ptr::{self, write_volatile};

use crate::memory::{Memory, physical::allocate};

use super::super::{Error, command::Submission as Command};

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

    pub fn next_cmd(&mut self) -> &'static mut Command {
        let cmd = Command::get_mut(self.addr + (self.tail % self.size) as usize * ENTRY_SIZE);
        self.tail = self.tail.wrapping_add(1);
        cmd
    }

    pub fn doorbell(&mut self, n: u16) -> Result<(), Error> {
        if n >= self.size {
            return Err(Error::Queue("Overflow"));
        }
        unsafe { write_volatile(self.doorbell, (self.tail % self.size) as u32) };
        Ok(())
    }
}
