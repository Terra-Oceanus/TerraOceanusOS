//! Storage

mod error;
pub mod nvme;

pub use error::Error;

use crate::memory::Memory;

pub fn init() -> Result<(), crate::Error> {
    nvme::init()?;

    let mbr = ProtectiveMBR::get_ref(nvme::read(0, 1)?);
    if !mbr.is_protective() {
        return Err(Error::InvalidMBR.into());
    }
    mbr.delete()?;

    Ok(())
}

#[repr(C, packed)]
struct ProtectiveMBR {
    boot_code: [u8; 440],

    unique_mbr_disk_signature: u32,

    unknown: u16,

    partition_record: [PartitionRecord; 4],

    signature: u16,
}
impl Memory for ProtectiveMBR {}
impl ProtectiveMBR {
    fn is_protective(&self) -> bool {
        if !self.partition_record[0].is_entire_disk() {
            return false;
        }
        if !self.partition_record[1].is_null() {
            return false;
        }
        if !self.partition_record[2].is_null() {
            return false;
        }
        if !self.partition_record[3].is_null() {
            return false;
        }
        self.signature == 0xAA55
    }
}

#[repr(C, packed)]
struct PartitionRecord {
    /// - 0x00
    boot_indicator: u8,

    /// - 0x000200
    starting_chs: [u8; 3],

    /// - 0xEE
    os_type: u8,

    /// - The CHS address of the last logical block
    ending_chs: [u8; 3],

    /// - 0x0000_0001
    starting_lba: u32,

    /// - The size of the disk - 1
    size_in_lba: u32,
}
impl PartitionRecord {
    fn is_entire_disk(&self) -> bool {
        if self.boot_indicator != 0x00 {
            return false;
        }
        if self.starting_chs != [0x00, 0x02, 0x00] {
            return false;
        }
        if self.os_type != 0xEE {
            return false;
        }
        self.starting_lba == 0x0000_0001
    }

    fn is_null(&self) -> bool {
        if self.boot_indicator != 0 {
            return false;
        }
        if self.starting_chs != [0, 0, 0] {
            return false;
        }
        if self.os_type != 0 {
            return false;
        }
        if self.ending_chs != [0, 0, 0] {
            return false;
        }
        if self.starting_lba != 0 {
            return false;
        }
        self.size_in_lba == 0
    }
}
