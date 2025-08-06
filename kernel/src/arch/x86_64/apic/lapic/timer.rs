//! Timer

use super::{super::super::idt::Interrupt, Local, read, write};

pub fn init() {
    write(
        Local::Timer,
        ((read(Local::Timer) & !0xFF) | Interrupt::Timer as u32) & !(1 << 16),
    );
}
