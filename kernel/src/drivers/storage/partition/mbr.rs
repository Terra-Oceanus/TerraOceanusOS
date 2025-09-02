//! Master Boot Record

use crate::memory::Memory;

use super::Error;

#[repr(C, packed)]
struct MBR {
    boot_code: [u8; 440],

    unique_mbr_disk_signature: u32,

    unknown: u16,

    partition_record: [PartitionRecord; 4],

    /// 0xAA55
    signature: u16,
}
impl Memory for MBR {}
impl MBR {
    fn validate(&self) -> Result<&Self, Error> {
        self.partition_record[0].validate()?;
        if self.partition_record[1..].iter().any(|pr| !pr.is_null()) {
            return Err(Error::InvalidMBR("PartitionRecord Non-zero"));
        }
        if self.signature != 0xAA55 {
            return Err(Error::InvalidMBR("Signature"));
        }
        Ok(self)
    }
}

#[repr(C, packed)]
struct PartitionRecord {
    /// - 0x00: Non-bootable partition
    /// - 0x80: Bootable legacy partition
    boot_indicator: u8,

    starting_chs: [u8; 3],

    /// - 0xEE: Protective MBR
    /// - 0xEF: UEFI system partition
    os_type: u8,

    ending_chs: [u8; 3],

    starting_lba: u32,

    size_in_lba: u32,
}
impl PartitionRecord {
    fn validate(&self) -> Result<(), Error> {
        if self.boot_indicator != 0x00 {
            return Err(Error::InvalidMBR("PartitionRecord.BootIndicator"));
        }
        if self.os_type != 0xEE {
            return Err(Error::InvalidMBR("PartitionRecord.OSType"));
        }
        Ok(())
    }

    fn is_null(&self) -> bool {
        self.boot_indicator == 0
            && self.starting_chs == [0; 3]
            && self.os_type == 0
            && self.ending_chs == [0; 3]
            && self.starting_lba == 0
            && self.size_in_lba == 0
    }
}

pub fn validate() -> Result<(), crate::Error> {
    MBR::get_ref(super::super::read(0, 1, 0)?)
        .validate()?
        .delete()?;
    Ok(())
}
