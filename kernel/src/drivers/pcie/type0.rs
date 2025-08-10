//! Standard Header

use crate::traits::FromAddr;

use super::{Error, Header};

#[repr(C, packed)]
struct Type0 {
    header: Header,

    /// Base Address Register
    bar: [u32; 6],

    p_cardbus_cis: u32,

    subsystem_id: u16,
    subsystem_vendor_id: u16,

    expansion_rom_base_address: u32,

    p_capabilities: u8,

    reserved: [u8; 7],

    interrupt_line: u8,
    interrupt_pin: u8,
    min_grant: u8,
    max_latency: u8,
}
impl FromAddr for Type0 {}
impl Type0 {
    fn handle(&self) -> Result<(), Error> {
        Ok(())
    }
}

pub fn handle(addr: u64) -> Result<(), Error> {
    Type0::get_ref(addr).handle()
}
