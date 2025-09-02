//! I/O APIC

use crate::{memory::Memory, x86_64::apic::ioapic};

use super::{super::Error, Header};

#[repr(C, packed)]
struct Type1 {
    header: Header,

    io_apic_id: u8,

    reserved: u8,

    io_apic_address: u32,

    global_system_interrupt_base: u32,
}
impl Memory for Type1 {}
impl Type1 {
    fn handle(&self) -> Result<(), crate::Error> {
        if self.header.length as usize != size_of::<Self>() {
            return Err(Error::InvalidLength.into());
        }
        let addr = self.io_apic_address;
        let base = self.global_system_interrupt_base;
        ioapic::append(addr, base)?;
        Ok(())
    }
}

pub fn handle(addr: usize) -> Result<(), crate::Error> {
    Type1::get_ref(addr).handle()
}
