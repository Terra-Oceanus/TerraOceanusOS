//! MSI-X

use core::ptr::{read_volatile, write_volatile};

use super::super::{BAR, Error};

pub const CAPABILITY_ID: u8 = 0x11;

#[repr(u8)]
pub enum Capability {
    /// - Bits 0 ..= 10: Table Size
    /// - Bits 11 ..= 13: Reserved
    /// - Bit 14: Function Mask
    /// - Bit 15: MSI-X Enable
    MessageControl = 0x2,

    /// - Bits 0 ..= 2: Table BIR
    ///   - 0: Base Address Register 0x10
    ///   - 1: Base Address Register 0x14
    ///   - 2: Base Address Register 0x18
    ///   - 3: Base Address Register 0x1C
    ///   - 4: Base Address Register 0x20
    ///   - 5: Base Address Register 0x24
    ///   - 6 ..= 7: Reserved
    /// - Bits 3 ..= 31: Table Offset
    Table = 0x4,

    /// - Bits 0 ..= 2: PBA BIR
    /// - Bits 3 ..= 31: PBA Offset
    PBA = 0x8,
}
impl Capability {
    fn read(self, addr: u64) -> u32 {
        unsafe {
            match self {
                Capability::MessageControl => {
                    read_volatile((addr + self as u64) as *const u16) as u32
                }
                Capability::Table | Capability::PBA => {
                    read_volatile((addr + self as u64) as *const u32)
                }
            }
        }
    }

    fn write(self, addr: u64, value: u32) {
        unsafe {
            match self {
                Capability::MessageControl => {
                    write_volatile((addr + self as u64) as *mut u16, value as u16)
                }
                Capability::Table | Capability::PBA => {
                    write_volatile((addr + self as u64) as *mut u32, value)
                }
            }
        }
    }

    pub fn configure(addr: u64, base: u64, vector: u8) -> Result<(), Error> {
        Capability::MessageControl.write(addr, Capability::MessageControl.read(addr) | 1 << 15);

        let table = Capability::Table.read(addr);
        let bir: u64 = (table & 0b111) as u64;
        let bar = unsafe { read_volatile((base + (0x10 + bir * 0x4)) as *const BAR) };
        if !bar.is_memory() {
            return Err(Error::Unsupported);
        }
        unsafe {
            (((if bar.is_64bit() {
                (read_volatile((base + (0x10 + (bir + 1) * 0x4)) as *const u32) as u64) << 32
            } else {
                0
            } | bar.addr())
                + (table >> 3) as u64) as *mut Table)
                .write_volatile(Table::new(vector))
        };
        Ok(())
    }
}

#[repr(C)]
struct Table {
    /// - Bits 0 ..= 1: Reserved
    /// - Bit 2: DM for Destination Mode
    ///   - 0 if RH is set: Logical
    ///   - 1 if RH is set: Physical
    ///   - Ignored if RH is clear
    /// - Bit 3: RH for Redirection Hint Indication
    /// - Bits 4 ..= 11: Reserved
    /// - Bits 12 ..= 19: Destination ID
    ///   - 0xFF if RH is set & DM is clear
    /// - Bits 20 ..= 31: 0xFEE
    /// - Bits 32 ..= 63: Reserved
    message_address: u64,

    /// - Bits 0 ..= 7: Vector
    /// - Bits 8 ..= 10: Delivery Mode
    ///   - 0b000: Fixed
    ///   - 0b001: Lowest Priority
    ///   - 0b010: SMI
    ///   - 0b011: Reserved
    ///   - 0b100: NMI
    ///   - 0b101: INIT
    ///   - 0b110: Reserved
    ///   - 0b111: ExtINT
    /// - Bits 11 ..= 13: Reserved
    /// - Bit 14: Level
    ///   - 0 if Trigger Mode is set: Deassert
    ///   - 1 if Trigger Mode is set: Assert
    ///   - Ignored if Trigger Mode is clear
    /// - Bit 15: Trigger Mode
    ///   - 0: Edge
    ///   - 1: Level
    /// - Bits 16 ..= 31: Reserved
    message_data: u32,

    /// - Bit 0: Mask Bit
    /// - Bits 1 ..= 31: Reserved
    vector_control: u32,
}
impl Table {
    fn new(vector: u8) -> Self {
        Self {
            message_address: ((0xFEE << 20) | (crate::x86_64::apic::lapic::id() << 12)).into(),
            message_data: vector.into(),
            vector_control: 0,
        }
    }
}
