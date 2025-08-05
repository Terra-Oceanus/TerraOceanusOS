//! I/O APIC

use crate::{
    error::{ACPI, Error},
    x86_64::apic::ioapic,
};

use super::{FromAddr, Header};

#[repr(C, packed)]
struct Type1 {
    header: Header,

    io_apic_id: u8,

    reserved: u8,

    io_apic_address: u32,

    global_system_interrupt_base: u32,
}
impl Type1 {
    fn handle(&self) -> Result<(), Error> {
        if self.header.length as usize != size_of::<Self>() {
            return Err(Error::ACPI(ACPI::InvalidLength));
        }
        if self.reserved != 0 {
            return Err(Error::ACPI(ACPI::InvalidReserved));
        }
        let addr = self.io_apic_address;
        let base = self.global_system_interrupt_base;
        ioapic::append(addr, base)?;
        Ok(())
    }
}

pub fn handle(addr: u64) -> Result<(), Error> {
    Type1::get_ref(addr).handle()?;
    Ok(())
}
