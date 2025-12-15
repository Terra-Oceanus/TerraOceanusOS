//! Processor Local APIC

use crate::mem::Memory;

use super::{super::Error, Header};

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
impl Memory for Type0 {}
impl Type0 {
    fn handle(&self) -> Result<(), Error> {
        if self.header.length as usize != size_of::<Self>() {
            return Err(Error::InvalidLength(*super::super::SIGNATURE));
        }
        Ok(())
    }
}

pub fn handle(addr: usize) -> Result<(), Error> {
    Type0::get_ref(addr).handle()
}
