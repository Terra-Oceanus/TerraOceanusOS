//! Standard Header

use crate::{drivers::storage::nvme, memory::Memory};

use super::Header;

#[macro_export]
macro_rules! find_capabilities {
    ($base:expr, $first:expr $(, $id:expr => $ptr:expr )+ $(,)?) => {{
        let mut offset = $first;
        while offset != 0 {
            let header = crate::drivers::pcie::capabilities::Header::get_ref($base + offset);
            $(
                if header.id() == $id {
                    *$ptr = header as *const _ as usize;
                }
            )+
            offset = header.next();
        }
    }};
}

#[repr(C)]
pub struct Type0 {
    pub header: Header,

    bar: [super::BAR; 6],

    reserved0: u32,

    subsystem_vendor_id: u16,
    subsystem_id: u16,

    /// - Bit 0: Expansion ROM Enable
    /// - Bits 1 ..= 3: Expansion ROM Validation Status
    ///   - 0b000: Validation not supported
    ///   - 0b001: Validation in Progress
    ///   - 0b010: Validation Pass Valid contents, trust test was not performed
    ///   - 0b011: Validation Pass Valid and trusted contents
    ///   - 0b100: Validation Fail Invalid contents
    ///   - 0b101: Validation Fail Valid but untrusted contents
    ///   - 0b110: Warning Pass Validation Passed with implementation specific warning. Valid contents, trust test was not performed
    ///   - 0b111: Warning Pass Validation Passed with implementation specific warning. Valid and trusted contents
    /// - Bits 4 ..= 7: Expansion ROM Validation Details
    /// - Bits 8 ..= 10: Reserved
    /// - Bits 11 ..= 31: Expansion ROM Base Address
    expansion_rom_base_address: u32,

    p_capabilities: u8,

    reserved1: [u8; 7],

    interrupt_line: u8,
    interrupt_pin: u8,

    reserved2: u16,
}
impl Memory for Type0 {}
impl Type0 {
    pub fn handle(&self) {
        match self.header.class_code[2] {
            // Mass Storage Controller
            0x01 => match self.header.class_code[1] {
                // Non-Volatile Memory Controller
                0x08 => match self.header.class_code[0] {
                    // NVM Express
                    0x02 => nvme::set_config(self as *const _ as usize),
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }
    }

    pub fn bar(&self, index: usize) -> usize {
        let bar = &self.bar[index];
        (if bar.is_memory() && bar.is_64bit() {
            (self.bar[index + 1].0 as usize) << 32
        } else {
            0
        }) | bar.addr()
    }

    pub fn p_capabilities(&self) -> usize {
        self.p_capabilities as usize
    }
}
