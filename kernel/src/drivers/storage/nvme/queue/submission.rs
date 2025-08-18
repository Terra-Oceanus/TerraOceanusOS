//! Submission

pub struct Submission {
    queue: super::Queue,

    tail: u16,
}

#[repr(C)]
pub struct Command {
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
    ///   - Bits 0 ..= 7: PRP1 for PRP Entry 1
    ///   - Bits 8 ..= 15: PRP2 for PRP Entry 2
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
impl Command {}
