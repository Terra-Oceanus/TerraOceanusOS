//! Delete I/O Submission Queue

impl super::super::Submission {
    /// - Opcode: 0x00
    ///   - Data Transfer: 0b00
    ///   - Function: 0b000000
    /// - Command Dword 10
    ///   - Bits 0 ..= 15: QID for Queue Identifier
    ///   - Bits 16 ..= 31: Reserved
    pub fn to_delete_io_submission_queue(&mut self, id: u32) {
        self.cdw10 = id;
    }
}
