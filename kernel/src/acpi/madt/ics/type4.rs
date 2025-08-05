//! Local APIC NMI
//!
//! NMI stands for Non-Maskable Interrupt

use crate::error::{ACPI, Error};

use super::{FromAddr, Header};

#[repr(C, packed)]
struct Type4 {
    header: Header,

    /// 0xFF for all
    acpi_processor_uid: u8,

    /// - Bits 0 ..= 1: Polarity
    /// - Bits 2 ..= 3: Trigger Mode
    /// - Bits 4 ..= 15: Reserved
    flags: u16,

    local_apic_lint: u8,
}
impl Type4 {
    fn handle(&self) -> Result<(), Error> {
        if self.header.length as usize != size_of::<Self>() {
            return Err(Error::ACPI(ACPI::InvalidLength));
        }
        if self.flags & !0b1111 != 0 {
            return Err(Error::ACPI(ACPI::InvalidReserved));
        }
        Ok(())
    }
}

pub fn handle(addr: u64) -> Result<(), Error> {
    Type4::get_ref(addr).handle()?;
    Ok(())
}
