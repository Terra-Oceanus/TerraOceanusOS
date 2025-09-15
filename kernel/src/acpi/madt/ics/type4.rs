//! Local APIC NMI
//!
//! NMI stands for Non-Maskable Interrupt

use crate::mem::Memory;

use super::{super::Error, Header};

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
impl Memory for Type4 {}
impl Type4 {
    fn handle(&self) -> Result<(), Error> {
        if self.header.length as usize != size_of::<Self>() {
            return Err(Error::InvalidLength(*super::super::SIGNATURE));
        }
        Ok(())
    }
}

pub fn handle(addr: usize) -> Result<(), Error> {
    Type4::get_ref(addr).handle()
}
