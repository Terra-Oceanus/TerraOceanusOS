//! Extended

#[repr(C)]
struct Header {
    extended_capability_id: u16,

    /// - Bits 0 ..= 3: Capability Version
    /// - Bits 4 ..= 15: Next Capability Offset
    info: u16,
}
impl Header {
    fn version(&self) -> u8 {
        (self.info & 0xF) as u8
    }

    fn next_capability_offset(&self) -> u16 {
        self.info >> 4
    }
}
