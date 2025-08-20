//! MSI-X

pub const CAPABILITY_ID: u8 = 0x11;

struct Capability {
    header: super::Header,

    /// - Bits 0 ..= 10: Table Size
    /// - Bits 11 ..= 13: Reserved
    /// - Bit 14: Function Mask
    /// - Bit 15: MSI-X Enable
    message_control: u16,

    /// - Bits 0 ..= 2: Table BIR
    ///   - 0: Base Address Register 10h
    ///   - 1: Base Address Register 14h
    ///   - 2: Base Address Register 18h
    ///   - 3: Base Address Register 1Ch
    ///   - 4: Base Address Register 20h
    ///   - 5: Base Address Register 24h
    ///   - 6 ..= 7: Reserved
    /// - Bits 3 ..= 31: Table Offset
    table: u32,

    /// - Bits 0 ..= 2: PBA BIR
    /// - Bits 3 ..= 31: PBA Offset
    pba: u32,
}
impl Capability {
    fn enable(&mut self) {
        self.message_control |= 1 << 15;
    }
}
