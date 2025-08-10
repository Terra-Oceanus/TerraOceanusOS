//! x86_64

pub mod apic;
mod cpuid;
mod dt;
mod error;
mod msr;

pub use dt::gdt;
use dt::idt;
pub use error::Error;

pub fn init() -> Result<(), crate::Error> {
    dt::init();
    apic::init()
}
