//! Interrupt Source Override

use crate::{memory::Memory, x86_64::apic::ioapic};

use super::{super::Error, Header};

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
impl Memory for Type2 {}
impl Type2 {
    fn handle(&self) -> Result<(), crate::Error> {
        if self.header.length as usize != size_of::<Self>() {
            return Err(Error::InvalidLength.into());
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

pub fn handle(addr: usize) -> Result<(), crate::Error> {
    Type2::get_ref(addr).handle()
}
