//! x86_64

use crate::error::Error;

pub mod apic;
mod cpuid;
mod dt;
mod msr;

pub use dt::gdt;
use dt::idt;

pub fn init() -> Result<(), Error> {
    dt::init();
    apic::init()
}
