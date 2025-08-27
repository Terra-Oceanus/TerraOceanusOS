//! Completion

use core::{
    hint::spin_loop,
    ptr::{self, write_bytes, write_volatile},
};

use crate::{Memory, memory::physical::allocate};

use super::super::command::Completion as Command;

const ENTRY_SIZE: usize = 16;

pub struct Completion {
    addr: usize,
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
            phase: true,
            doorbell: ptr::null_mut(),
        }
    }

    pub fn init(&mut self, size: u16, doorbell: usize) -> Result<usize, crate::Error> {
        self.addr = allocate(size as usize * ENTRY_SIZE)?;
        self.size = size;
        unsafe { write_bytes(self.addr as *mut Command, 0, self.size as usize) };
        self.doorbell = doorbell as *mut u32;
        Ok(self.addr)
    }

    pub fn dequeue(&mut self) -> &'static Command {
        let command = loop {
            let command = Command::get_ref(self.addr + (self.head as usize * ENTRY_SIZE));
            if command.phase() == self.phase {
                break command;
            }
            spin_loop();
        };
        self.head = {
            if self.head + 1 == self.size {
                self.phase = !self.phase;
                0
            } else {
                self.head + 1
            }
        };
        unsafe { write_volatile(self.doorbell, self.head as u32) };

        command
    }
}
