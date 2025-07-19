//! Advanced Programmable Interrupt Controller

use crate::{Error, Output, init_end, init_message, init_start};

use super::msr;

pub mod ioapic;
pub mod lapic;

pub fn init() -> Result<(), Error> {
    init_start!();
    init_message!(true, false, "APIC is ");
    let ia32_apic_base = msr::read(msr::IA32_APIC_BASE);
    if (ia32_apic_base >> 11) & 1 == 0 {
        init_message!(false, false, "disabled...");
        msr::write(msr::IA32_APIC_BASE, ia32_apic_base | 1 << 11);
    };
    init_message!(false, true, "enabled");
    lapic::init()?;
    ioapic::init()?;
    init_end!();
    Ok(())
}
