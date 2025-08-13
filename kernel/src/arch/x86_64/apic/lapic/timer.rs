//! Timer

use super::{super::super::idt::Interrupt, Local};

pub fn init() {
    Local::Timer.write(((Local::Timer.read() & !0xFF) | Interrupt::Timer as u32) & !(1 << 16));
}
