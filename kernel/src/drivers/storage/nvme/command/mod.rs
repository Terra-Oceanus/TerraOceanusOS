//! Command

use crate::mem::Memory;

use super::Error;

pub mod admin;
pub mod io;

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
    pub fn set_cid(&mut self, cid: u16) {
        self.cdw0 = (cid as u32) << 16;
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

    pub fn gcs_sc_to_str(&self) -> Result<(), Error> {
        match self.sc() {
            0x00 => Ok(()),
            0x01 => Err(Error::Queue("Invalid Command Opcode")),
            0x02 => Err(Error::Queue("Invalid Field in Command")),
            0x03 => Err(Error::Queue("Command ID Conflict")),
            0x04 => Err(Error::Queue("Data Transfer Error")),
            0x05 => Err(Error::Queue(
                "Commands Aborted due to Power Loss Notification",
            )),
            0x06 => Err(Error::Queue("Internal Error")),
            0x07 => Err(Error::Queue("Command Abort Requested")),
            0x08 => Err(Error::Queue("Command Aborted due to SQ Deletion")),
            0x09 => Err(Error::Queue("Command Aborted due to Failed Fused Command")),
            0x0A => Err(Error::Queue("Command Aborted due to Missing Fused Command")),
            0x0B => Err(Error::Queue("Invalid Namespace or Format")),
            0x0C => Err(Error::Queue("Command Sequence Error")),
            0x0D => Err(Error::Queue("Invalid SGL Segment Descriptor")),
            0x0E => Err(Error::Queue("Invalid Number of SGL Descriptors")),
            0x0F => Err(Error::Queue("Data SGL Length Invalid")),
            0x10 => Err(Error::Queue("Metadata SGL Length Invalid")),
            0x11 => Err(Error::Queue("SGL Descriptor Type Invalid")),
            0x12 => Err(Error::Queue("Invalid Use of Controller Memory Buffer")),
            0x13 => Err(Error::Queue("PRP Offset Invalid")),
            0x14 => Err(Error::Queue("Atomic Write Unit Exceeded")),
            0x15 => Err(Error::Queue("Operation Denied")),
            0x16 => Err(Error::Queue("SGL Offset Invalid")),
            0x18 => Err(Error::Queue("Host Identifier Inconsistent Format")),
            0x19 => Err(Error::Queue("Keep Alive Timer Expired")),
            0x1A => Err(Error::Queue("Keep Alive Timeout Invalid")),
            0x1B => Err(Error::Queue("Command Aborted due to Preempt and Abort")),
            0x1C => Err(Error::Queue("Sanitize Failed")),
            0x1D => Err(Error::Queue("Sanitize In Progress")),
            0x1E => Err(Error::Queue("SGL Data Block Granularity Invalid")),
            0x1F => Err(Error::Queue("Command Not Supported for Queue in CMB")),
            0x20 => Err(Error::Queue("Namespace is Write Protected")),
            0x21 => Err(Error::Queue("Command Interrupted")),
            0x22 => Err(Error::Queue("Transient Transport Error")),
            0x23 => Err(Error::Queue(
                "Command Prohibited by Command and Feature Lockdown",
            )),
            0x24 => Err(Error::Queue("Admin Command Media Not Ready")),
            0x25 => Err(Error::Queue("Invalid Key Tag")),
            0x26 => Err(Error::Queue("Host Dispersed Namespace Support Not Enabled")),
            0x27 => Err(Error::Queue("Host Identifier Not Initialized")),
            0x28 => Err(Error::Queue("Incorrect Key")),
            0x29 => Err(Error::Queue("FDP Disabled")),
            0x2A => Err(Error::Queue("Invalid Placement Handle List")),
            0x80 => Err(Error::Queue("LBA Out of Range")),
            0x81 => Err(Error::Queue("Capacity Exceeded")),
            0x82 => Err(Error::Queue("Namespace Not Ready")),
            0x83 => Err(Error::Queue("Reservation Conflict")),
            0x84 => Err(Error::Queue("Format In Progress")),
            0x85 => Err(Error::Queue("Invalid Value Size")),
            0x86 => Err(Error::Queue("Invalid Key Size")),
            0x87 => Err(Error::Queue("KV Key Does Not Exist")),
            0x88 => Err(Error::Queue("Unrecovered Error")),
            0x89 => Err(Error::Queue("Key Exists")),
            0xC0..=0xFF => Err(Error::Queue("Vendor Specific")),
            _ => Err(Error::Queue(
                "Unknown Status Code in Generic Command Status",
            )),
        }
    }
}
