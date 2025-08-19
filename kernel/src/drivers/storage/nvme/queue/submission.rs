//! Submission

use core::ptr::{self, copy_nonoverlapping, write_volatile};

use crate::memory::physical::allocate;

const ENTRY_SIZE: usize = 64;

pub struct Submission {
    addr: u64,
    size: u16,

    tail: u16,

    doorbell: *mut u32,
}
impl Submission {
    pub const fn null() -> Self {
        Self {
            addr: 0,
            size: 0,
            tail: 0,
            doorbell: ptr::null_mut(),
        }
    }

    pub fn init(&mut self, id: u16, size: u16) -> Result<u64, crate::Error> {
        self.addr = allocate(size as u64 * ENTRY_SIZE as u64)?;
        self.size = size;
        self.doorbell =
            unsafe { super::super::ADDR + (2 * id as u64) * (1 << (2 + super::super::DSTRD)) }
                as *mut u32;
        Ok(self.addr)
    }

    fn enqueue(&mut self, cmd: Command) {
        unsafe {
            copy_nonoverlapping(
                &cmd as *const Command as *const u8,
                (self.addr as *mut u8).add((self.tail % self.size) as usize * 64),
                ENTRY_SIZE,
            );
            write_volatile(self.doorbell, self.tail as u32);
        }
        self.tail += self.tail.wrapping_add(1);
    }
}

#[repr(C, packed)]
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
    pub dptr: u128,

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
impl Command {
    pub fn null() -> Self {
        Self {
            cdw0: 0,
            nsid: 0,
            cdw2: 0,
            cdw3: 0,
            mptr: 0,
            dptr: 0,
            cdw10: 0,
            cdw11: 0,
            cdw12: 0,
            cdw13: 0,
            cdw14: 0,
            cdw15: 0,
        }
    }
}
