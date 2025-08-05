//! Advanced Programmable Interrupt Controller

use crate::{acpi::madt, error::Error};

use super::msr;

pub mod ioapic;
pub mod lapic;

pub fn init() -> Result<(), Error> {
    let ia32_apic_base = msr::read(msr::IA32_APIC_BASE);
    if (ia32_apic_base >> 11) & 1 == 0 {
        msr::write(msr::IA32_APIC_BASE, ia32_apic_base | 1 << 11);
    };

    let addr = madt::init()?;

    lapic::init(addr)?;
    ioapic::init()?;
    Ok(())
}
