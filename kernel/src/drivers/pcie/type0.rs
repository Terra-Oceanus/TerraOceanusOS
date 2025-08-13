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
    ///     - 0b11: Reserved
    ///   - Bit 3: Undefined
    ///   - Bits 4 ..= 31: 16-Byte Aligned Base Address
    /// - I/O Space
    ///   - Bit 0: 1
    ///   - Bit 1: Reserved
    ///   - Bits 2 ..= 31: 4-Byte Aligned Base Address
    bar: [u32; 6],

    reserved0: u32,

    subsystem_vendor_id: u16,
    subsystem_id: u16,

    /// - Bit 0: Expansion ROM Enable (RW)
    /// - Bits 1 ..= 3: Expansion ROM Validation Status (RO)
    ///   - 0b000: Validation not supported
    ///   - 0b001: Validation in Progress
    ///   - 0b010: Validation Pass Valid contents, trust test was not performed
    ///   - 0b011: Validation Pass Valid and trusted contents
    ///   - 0b100: Validation Fail Invalid contents
    ///   - 0b101: Validation Fail Valid but untrusted contents
    ///   - 0b110: Warning Pass Validation Passed with implementation specific warning. Valid contents, trust test was not performed
    ///   - 0b111: Warning Pass Validation Passed with implementation specific warning. Valid and trusted contents
    /// - Bits 4 ..= 7: Expansion ROM Validation Details (RO)
    /// - Bits 8 ..= 10: Reserved
    /// - Bits 11 ..= 31: Expansion ROM Base Address (RW)
    expansion_rom_base_address: u32,

    p_capabilities: u8,

    reserved1: [u8; 7],

    interrupt_line: u8,
    interrupt_pin: u8,

    reserved2: u16,
}
impl FromAddr for Type0 {}
impl Type0 {
    fn handle(&self) {
        match self.header.class_code[2] {
            // Mass Storage Controller
            0x01 => match self.header.class_code[1] {
                // Non-Volatile Memory Controller
                0x08 => match self.header.class_code[0] {
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
