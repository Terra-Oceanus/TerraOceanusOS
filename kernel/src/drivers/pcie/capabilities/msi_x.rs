//! MSI-X

use core::ptr;

use super::super::Error;

pub struct MSIX {
    pub addr: usize,

    tables: *mut Table,
    table_count: usize,
}
impl MSIX {
    pub const ID: u8 = 0x11;

    /// - Bits 0 ..= 10: Table Size
    /// - Bits 11 ..= 13: Reserved
    /// - Bit 14: Function Mask
    /// - Bit 15: MSI-X Enable
    const MESSAGE_CONTROL: usize = 0x02;

    /// - Bits 0 ..= 2: Table BIR
    ///   - 0: Base Address Register 0x10
    ///   - 1: Base Address Register 0x14
    ///   - 2: Base Address Register 0x18
    ///   - 3: Base Address Register 0x1C
    ///   - 4: Base Address Register 0x20
    ///   - 5: Base Address Register 0x24
    ///   - 6 ..= 7: Reserved
    /// - Bits 3 ..= 31: Table Offset
    const TABLE: usize = 0x04;

    /// - Bits 0 ..= 2: PBA BIR
    /// - Bits 3 ..= 31: PBA Offset
    const PBA: usize = 0x08;

    pub const fn null() -> Self {
        Self {
            addr: 0,
            tables: ptr::null_mut(),
            table_count: 0,
        }
    }

    pub fn disable(&self) {
        let mc = (self.addr + Self::MESSAGE_CONTROL) as *mut u16;
        unsafe { mc.write_volatile(mc.read_volatile() & !(1 << 15)) };
    }

    pub fn enable(&self) {
        let mc = (self.addr + Self::MESSAGE_CONTROL) as *mut u16;
        unsafe { mc.write_volatile((mc.read_volatile() & !(1 << 14)) | (1 << 15)) };
    }

    pub fn table_bir(&self) -> Result<usize, Error> {
        match unsafe { ((self.addr + Self::TABLE) as *mut u32).read_volatile() } & 0b111 {
            n if (0..=5).contains(&n) => Ok(n as usize),
            _ => Err(Error::InvalidRegisterValue("Table BIR")),
        }
    }

    pub fn set_tables(&mut self, addr: usize) {
        self.tables = (addr
            + (unsafe { ((self.addr + Self::TABLE) as *mut u32).read_volatile() } & !0b111)
                as usize) as *mut Table;
        self.table_count =
            (unsafe { ((self.addr + Self::MESSAGE_CONTROL) as *mut u16).read_volatile() }
                & 0b111_1111_1111) as usize;
    }

    pub fn configure(&self, table_index: usize, vector: u8) -> Result<(), Error> {
        if table_index > self.table_count {
            return Err(Error::InvalidIndex("MSI-X Table"));
        }
        unsafe {
            self.tables
                .add(table_index)
                .write_volatile(Table::new(vector))
        }
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
            message_address: (0xFEE << 20) | ((crate::x86_64::apic::lapic::id() as u64) << 12),
            message_data: vector.into(),
            vector_control: 0,
        }
    }
}
