//! Command

use core::ptr::write_bytes;

use crate::Memory;

pub mod admin;

#[repr(C, packed)]
pub struct Submission {
    /// Command Dword 0
    /// - Bits 0 ..= 7: OPC for Opcode
    ///   - Bits 0 ..= 1: DTD for Data Transfer Direction
    ///     - 0b00: No Data Transfer
    ///     - 0b01: Host to Controller Transfer
    ///     - 0b10: Controller to Host Transfer
    ///     - 0b11: Bi-Directional Transfers
    ///   - Bits 2 ..= 7: FN for Function
    /// - Bits 8 ..= 9: FUSE for Fused Operation
    ///   - 0b00: Normal operation
    ///   - 0b01: First command of Fused operation
    ///   - 0b10: Second command of Fused operation
    ///   - 0b11: Reserved
    /// - Bits 10 ..= 13: Reserved
    /// - Bits 14 ..= 15: PSDT for PRP or SGL for Data Transfer
    ///   - 0b00: PRPS Used
    ///   - 0b01: SGLs Used MPTR Address
    ///   - 0b10: SGLs Used MPTR SGL Segment
    ///   - 0b11: Reserved
    /// - Bits 16 ..= 31: CID for Command Identifier
    cdw0: u32,

    /// Namespace Identifier
    /// - 0x0: Not used
    /// - 0xFFFFFFFF: Broadcast
    nsid: u32,

    /// Command Dword 2
    cdw2: u32,

    /// Command Dword 3
    cdw3: u32,

    /// Metadata Pointer
    mptr: u64,

    /// Data Pointer
    /// - PRP if CDW0.PSDT is 0b00
    ///   - Bytes 0 ..= 7: PRP1 for PRP Entry 1
    ///   - Bytes 8 ..= 15: PRP2 for PRP Entry 2
    /// - SGL1 for SGL Entry 1 if CDW0.PSDT is 0b01 ..= 0b10
    dptr: u128,

    /// Command Dword 10
    cdw10: u32,

    /// Command Dword 11
    cdw11: u32,

    /// Command Dword 12
    cdw12: u32,

    /// Command Dword 13
    cdw13: u32,

    /// Command Dword 14
    cdw14: u32,

    /// Command Dword 15
    cdw15: u32,
}
impl Memory for Submission {}
impl Submission {
    pub fn clear(&mut self) -> &'static mut Self {
        unsafe {
            write_bytes(self as *mut Self, 0, 1);
            &mut *(self as *mut Self)
        }
    }
}

#[repr(C)]
pub struct Completion {
    dw0: u32,
    dw1: u32,

    /// - Bits 0 ..= 15: SQHD for SQ Head Pointer
    /// - Bits 16 ..= 31: SQID for SQ Identifier
    dw2: u32,

    /// - Bits 0 ..= 15: CID for Command Identifier
    /// - Bit 16: P for Phase Tag
    /// - Bits 17 ..= 31: STATUS for Status
    ///   - Bits 0 ..= 7: SC for Status Code
    ///   - Bits 8 ..= 10: SCT for Status Code Type
    ///   - Bits 11 ..= 12: CRD for Command Retry Delay
    ///   - Bit 13: M for More
    ///   - Bit 14: DNR for Do Not Retry
    dw3: u32,
}
impl Memory for Completion {}
impl Completion {
    pub fn phase(&self) -> bool {
        ((self.dw3 >> 16) & 0b1) == 1
    }

    pub fn sc(&self) -> u8 {
        (self.dw3 >> 17) as u8
    }

    /// Status Code Type
    /// - 0x0: Generic Command Status
    /// - 0x1: Command Specific Status
    /// - 0x2: Media and Data Integrity Errors
    /// - 0x3: Path Related Status
    /// - 0x4 ..= 0x6: Reserved
    /// - 0x7: Vendor Specific
    pub fn sct(&self) -> u8 {
        (((self.dw3 >> 17) >> 8) & 0b111) as u8
    }
}
