//! Identify

use crate::memory::physical::allocate;

pub mod controller;

impl super::super::Submission {
    /// - Opcode: 0x06
    ///   - Data Transfer: 0b10
    ///   - Function: 0b000001
    /// - Data Pointer
    /// - Command Dword 10
    ///   - Bits 0 ..= 7: CNS for Controller or Namespace Structure
    ///   - Bits 8 ..= 15: Reserved
    ///   - Bits 16 ..= 31: CNTID for Controller Identifier
    /// - Command Dword 11
    ///   - Bits 0 ..= 15: CNSSID for CNS Specific Identifier
    ///   - Bits 16 ..= 23: Reserved
    ///   - Bits 24 ..= 31: CSI for Command Set Identifier
    /// - Command Dword 14
    ///   - Bits 0 ..= 6: UIDX for UUID Index
    ///   - Bits 7 ..= 31: Reserved
    fn identify() -> Result<Self, crate::Error> {
        let mut cmd = Self::null();
        cmd.cdw0 = 0x06;
        cmd.dptr = allocate(0x1000)? as u128;
        Ok(cmd)
    }
}
