//! Processor Local APIC

use crate::error::{ACPI, Error};

use super::{FromAddr, Header};

#[repr(C, packed)]
struct Type0 {
    header: Header,

    acpi_processor_uid: u8,

    apic_id: u8,

    /// - Bit 0: Enabled
    /// - Bit 1: Online Capable
    /// - Bits 2 ..= 31: Reserved
    flags: u32,
}
impl Type0 {
    fn handle(&self) -> Result<(), Error> {
        if self.header.length as usize != size_of::<Self>() {
            return Err(Error::ACPI(ACPI::InvalidLength));
        }
        if self.flags & 1 == 1 {
            if self.flags & !1 != 0 {
                return Err(Error::ACPI(ACPI::InvalidReserved));
            }
        } else {
            if self.flags >> 1 & 1 == 1 {
                if self.flags & !0b11 != 0 {
                    return Err(Error::ACPI(ACPI::InvalidReserved));
                }
            }
        }
        Ok(())
    }
}

pub fn handle(addr: u64) -> Result<(), Error> {
    Type0::get_ref(addr).handle()?;
    Ok(())
}
