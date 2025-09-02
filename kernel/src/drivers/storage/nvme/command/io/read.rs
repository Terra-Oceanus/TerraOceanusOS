//! Read

use super::super::super::Error;

impl super::super::Submission {
    /// - CDW0.OPC: 0x02
    ///   - Data Transfer: 0b10
    ///   - Function: 0b000000
    /// - NSID
    /// - CDW2
    ///   - Bits 0 ..= 31: ELBTU for Expected Logical Block Tags Upper
    /// - CDW3
    ///   - Bits 0 ..= 15: ELBTU for Expected Logical Block Tags Upper
    ///   - Bits 16 ..= 31: Reserved
    /// - MPTR
    /// - DPTR
    /// - CDW10
    ///   - Bits 0 ..= 31: SLBA for Starting LBA
    /// - CDW11
    ///   - Bits 0 ..= 31: SLBA for Starting LBA
    /// - CDW12
    ///   - Bits 0 ..= 15: NLB for Number of Logical Blocks
    ///   - Bits 16 ..= 19: CETYPE for Command Extension Type
    ///   - Bits 20 ..= 23: Reserved
    ///   - Bit 24: STC for Storage Tag Check
    ///   - Bit 25: Reserved
    ///   - Bits 26 ..= 29: PRINFO for Protection Information
    ///   - Bit 30: FUA for Force Unit Access
    ///   - Bit 31: LR for Limited Retry
    /// - CDW13
    ///   - if CDW12.CETYPE is cleared
    ///     - Bits 0 ..= 7: DSM for Dataset Management
    ///       - Bits 0 ..= 3: AF for Access Frequency
    ///         - 0x0: No frequency information provided.
    ///         - 0x1: Typical number of reads & writes expected for this LBA range
    ///         - 0x2: Infrequent writes & infrequent reads to the LBA range indicated
    ///         - 0x3: Infrequent writes & frequent reads to the LBA range indicated
    ///         - 0x4: Frequent writes & infrequent reads to the LBA range indicated
    ///         - 0x5: Frequent writes & frequent reads to the LBA range indicated
    ///         - 0x6: One time read
    ///         - 0x7: Speculative read
    ///         - 0x8: The LBA range is going to be overwritten in the near future
    ///         - 0x9 ..= 0xF: Reserved
    ///       - Bits 4 ..= 5: AL for Access Latency
    ///         - 0b00: None
    ///         - 0b01: Idle
    ///         - 0b10: Normal
    ///         - 0b11: Low
    ///       - Bit 6: SEQREQ for Sequential Request
    ///       - Bit 7: INCPRS for Incompressible
    ///     - Bits 8 ..= 15: Reserved
    ///   - if CDW12.CETYPE is non-zero
    ///     - Bits 0 ..= 15: CEV for Command Extension Value
    ///   - Bits 16 ..= 31: Reserved
    /// - CDW14
    ///   - Bits 0 ..= 31: ELBTL for Expected Logical Block Tags Lower
    /// - CDW15
    ///   - Bits 0 ..= 15: ELBAT for Expected Logical Block Application Tag
    ///   - Bits 16 ..= 31: ELBATM for Expected Logical Block Application Tag Mask
    pub fn to_read(&mut self, id: u32, addr: u64, lba_start: u64, lba_count: u32) {
        self.cdw0 |= 0x02;
        self.nsid = id;
        self.dptr = addr as u128;
        self.cdw10 = lba_start as u32;
        self.cdw11 = (lba_start >> 32) as u32;
        self.cdw12 = lba_count << 16;
    }
}

impl super::super::Completion {
    pub fn to_read(&self) -> Result<(), Error> {
        match self.sct() {
            0x0 => return self.gcs_sc_to_str(),
            0x1 => match self.sc() {
                0x80 => return Err(Error::Queue("Conflicting Attributes")),
                0x81 => return Err(Error::Queue("Invalid Protection Information")),
                _ => {}
            },
            _ => {}
        }
        Err(Error::Queue("Unknown Status Code Type"))
    }
}
