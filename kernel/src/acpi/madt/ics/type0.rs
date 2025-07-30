//! Processor Local APIC

use crate::{
    Output,
    error::{ACPI, Error},
    init_message,
};

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
        init_message!(
            false,
            false,
            "CPU with ACPI Processor UID(",
            self.acpi_processor_uid as usize,
            ") & APIC ID(",
            self.apic_id as usize,
            ") detected..."
        );
        if self.flags & 1 == 1 {
            if self.flags & !1 != 0 {
                return Err(Error::ACPI(ACPI::InvalidReserved));
            }
            init_message!(false, true, "unprocessed");
        } else {
            if self.flags >> 1 & 1 == 1 {
                if self.flags & !0b11 != 0 {
                    return Err(Error::ACPI(ACPI::InvalidReserved));
                }
                init_message!(false, true, "can be enabled at runtime");
            } else {
                init_message!(false, true, "is unavailable");
            }
        }
        Ok(())
    }
}

pub fn handle(addr: u64) -> Result<(), Error> {
    Type0::get_ref(addr).handle()?;
    Ok(())
}
