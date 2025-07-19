//! Timer

use crate::{Output, init_end, init_start};

use super::{super::super::idt::Interrupt, Local, read, write};

pub fn init() {
    init_start!();
    write(
        Local::Timer,
        ((read(Local::Timer) & !0xFF) | Interrupt::Timer as u32) & !(1 << 16),
    );
    init_end!();
}
