//! Create I/O Submission Queue

use super::super::super::Error;

impl super::super::Submission {
    /// - Opcode: 0x01
    ///   - Data Transfer: 0b01
    ///   - Function: 0b000000
    /// - DPTR.PRP1
    ///   - A 64-bit base memory address pointer if CDW11.PC is set
    ///   - A PRP List pointer if CDW11.PC is cleared
    /// - CDW10
    ///   - Bits 0 ..= 15: QID for Queue Identifier
    ///   - Bits 16 ..= 31: QSIZE for Queue Size
    /// - CDW11
    ///   - Bit 0: PC for Physically Contiguous
    ///   - Bits 1 ..= 2: QPRIO for Queue Priority
    ///     - 0b00: Urgent
    ///     - 0b01: High
    ///     - 0b10: Medium
    ///     - 0b11: Low
    ///   - Bits 3 ..= 15: Reserved
    ///   - Bits 16 ..= 31: CQID for Completion Queue Identifier
    /// - CDW12
    ///   - Bits 0 ..= 15: NVMSETID for NVM Set Identifier
    ///   - Bits 16 ..= 31: Reserved
    pub fn to_create_io_submission_queue(&mut self, addr: u64, id: u32, size: u32) {
        self.cdw0 |= 0x01;
        self.dptr = addr as u128;
        self.cdw10 = ((size - 1) << 16) | id;
        self.cdw11 = (id << 16) | 1;
    }
}

impl super::super::Completion {
    pub fn to_create_io_submission_queue(&self) -> Result<(), Error> {
        match self.sct() {
            0x0 => return self.gcs_sc_to_str(),
            0x1 => match self.sc() {
                0x00 => return Err(Error::Queue("Completion Queue Invalid")),
                0x01 => return Err(Error::Queue("Invalid Queue Identifier")),
                0x02 => return Err(Error::Queue("Invalid Queue Size")),
                _ => {}
            },
            _ => {}
        }
        Err(Error::Queue("Unknown Status Code Type"))
    }
}
