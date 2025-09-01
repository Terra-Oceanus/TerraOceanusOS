//! Delete I/O Completion Queue

impl super::super::Submission {
    /// - Opcode: 0x04
    ///   - Data Transfer: 0b00
    ///   - Function: 0b000001
    /// - Command Dword 10
    ///   - Bits 0 ..= 15: QID for Queue Identifier
    ///   - Bits 16 ..= 31: Reserved
    pub fn to_delete_io_completion_queue(&mut self, id: u32) {
        self.cdw0 = 0x04;
        self.cdw10 = id;
    }
}
