//! Identify

pub mod active_namespace_id_list;
pub mod controller;
pub mod namespace;

impl super::super::Submission {
    /// - CDW0.OPC: 0x06
    ///   - Data Transfer: 0b10
    ///   - Function: 0b000001
    /// - DPTR
    /// - CDW10
    ///   - Bits 0 ..= 7: CNS for Controller or Namespace Structure
    ///   - Bits 8 ..= 15: Reserved
    ///   - Bits 16 ..= 31: CNTID for Controller Identifier
    /// - CDW11
    ///   - Bits 0 ..= 15: CNSSID for CNS Specific Identifier
    ///   - Bits 16 ..= 23: Reserved
    ///   - Bits 24 ..= 31: CSI for Command Set Identifier
    /// - CDW14
    ///   - Bits 0 ..= 6: UIDX for UUID Index
    ///   - Bits 7 ..= 31: Reserved
    fn to_identify(&mut self, addr: usize) {
        self.cdw0 = 0x06;
        self.dptr = addr as u128;
    }
}
