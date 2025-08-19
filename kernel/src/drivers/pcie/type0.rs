//! Standard Header

use crate::traits::FromAddr;

use super::{super::storage::nvme, Header};

#[repr(C, packed)]
struct Type0 {
    header: Header,

    /// Base Address Register
    /// - Memory Space
    ///   - Bit 0: 0
    ///   - Bits 1 ..= 2: Type
    ///     - 0b00: 32-bits
    ///     - 0b01: Reserved
    ///     - 0b10: 64-bits
    ///   - Bit 3: Prefetchable
    ///   - Bits 4 ..= 31: 16-Byte Aligned Base Address
    /// - I/O Space
    ///   - Bit 0: 1
    ///   - Bit 1: Reserved
    ///   - Bits 2 ..= 31: 4-Byte Aligned Base Address
    bar: [u32; 6],

    p_cardbus_cis: u32,

    subsystem_id: u16,
    subsystem_vendor_id: u16,

    expansion_rom_base_address: u32,

    p_capabilities: u8,

    reserved0: [u8; 7],

    interrupt_line: u8,
    interrupt_pin: u8,

    reserved1: u16,
}
impl FromAddr for Type0 {}
impl Type0 {
    fn handle(&self) {
        match self.header.class {
            // Mass Storage Controller
            0x01 => match self.header.subclass {
                // Non-Volatile Memory Controller
                0x08 => match self.header.programming_interface {
                    // NVM Express
                    0x02 => {
                        nvme::set_config(
                            ((self.bar[1] as u64) << 32) | (self.bar[0] & 0xFFFFFFF0) as u64,
                        );
                    }
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }
    }
}

pub fn handle(addr: u64) {
    Type0::get_ref(addr).handle();
}
