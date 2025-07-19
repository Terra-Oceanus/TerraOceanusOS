//! x86_64

use crate::{Error, Output, init_end, init_start};

pub mod apic;
mod cpuid;
mod dt;
mod msr;

pub use dt::gdt;
use dt::idt;

pub fn init() -> Result<(), Error> {
    init_start!();
    dt::init();
    apic::init()?;
    init_end!();
    Ok(())
}
