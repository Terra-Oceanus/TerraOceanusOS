//! Create I/O Completion Queue

impl super::super::Submission {
    /// - CDW0.OPC: 0x05
    ///   - Data Transfer: 0b01
    ///   - Function: 0b000001
    /// - DPTR.PRP1
    ///   - A 64-bit base memory address pointer if CDW11.PC is set
    ///   - A PRP List pointer if CDW11.PC is clear
    /// - CDW10
    ///   - Bits 0 ..= 15: QID for Queue Identifier
    ///   - Bits 16 ..= 31: QSIZE for Queue Size
    /// - CDW11
    ///   - Bit 0: PC for Physically Contiguous
    ///   - Bit 1: IEN for Interrupts Enabled
    ///   - Bits 2 ..= 15: Reserved
    ///   - Bits 16 ..= 31: IV for Interrupt Vector
    pub fn to_create_io_completion_queue(&mut self, addr: u64, id: u32, size: u32, vector: u32) {
        self.cdw0 = 0x05;
        self.dptr = addr as u128;
        self.cdw10 = (size << 16) | id;
        self.cdw11 = 1;
        if vector != 0 {
            self.cdw11 |= (vector << 16) | (1 << 1);
        }
    }
}
