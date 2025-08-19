//! Advanced Programmable Interrupt Controller

use crate::acpi::madt;

use super::msr;

mod error;
pub mod ioapic;
pub mod lapic;

pub use error::Error;

pub fn init() -> Result<(), crate::Error> {
    let ia32_apic_base = msr::read(msr::IA32_APIC_BASE);
    if (ia32_apic_base >> 11) & 1 == 0 {
        msr::write(msr::IA32_APIC_BASE, ia32_apic_base | 1 << 11);
    };

    let addr = madt::init()?;

    lapic::init(addr);
    ioapic::init();
    Ok(())
}
