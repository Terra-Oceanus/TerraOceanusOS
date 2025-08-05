//! Interrupt Source Override

use crate::{
    error::{ACPI, Error},
    x86_64::apic::ioapic,
};

use super::{FromAddr, Header};

#[repr(C, packed)]
struct Type2 {
    header: Header,

    /// 0 for ISA
    bus: u8,

    source: u8,

    global_system_interrupt: u32,

    /// - Bits 0 ..= 1: Polarity
    /// - Bits 2 ..= 3: Trigger Mode
    /// - Bits 4 ..= 15: Reserved
    flags: u16,
}
impl Type2 {
    fn handle(&self) -> Result<(), Error> {
        if self.header.length as usize != size_of::<Self>() {
            return Err(Error::ACPI(ACPI::InvalidLength));
        }
        if self.flags & !0b1111 != 0 {
            return Err(Error::ACPI(ACPI::InvalidReserved));
        }
        ioapic::handle_override(
            self.source,
            self.global_system_interrupt,
            (self.flags & 0b11) as u8,
            ((self.flags >> 2) & 0b11) as u8,
        )?;
        Ok(())
    }
}

pub fn handle(addr: u64) -> Result<(), Error> {
    Type2::get_ref(addr).handle()?;
    Ok(())
}
