//! GUID Partition Table

use crate::{math::Checksum, memory::Memory, types::guid::GUID};

use super::Error;

#[repr(C, packed)]
struct Header {
    /// - "EFI PART"
    /// as 0x5452415020494645
    signature: u64,

    /// - 0x00010000
    revision: u32,

    header_size: u32,

    /// CRC32 checksum
    header_crc32: u32,

    reserved: u32,

    /// LBA of self
    my_lba: u64,

    /// LBA of the alternate Self
    alternate_lba: u64,

    first_usable_lba: u64,
    last_usable_lba: u64,

    disk_guid: GUID,

    partition_entry_lba: u64,

    number_of_partition_entries: u32,

    size_of_partition_entry: u32,

    partition_entry_array_crc32: u32,
}
impl Checksum for Header {}
impl Memory for Header {}
impl Header {
    fn validate(&mut self, lba: u64) -> Result<usize, crate::Error> {
        if self.signature != 0x5452415020494645 {
            return Err(Error::InvalidGPT("Signature").into());
        }

        let crc32 = self.header_crc32;
        self.header_crc32 = 0;
        if crc32 != self.crc32(self.header_size as usize) {
            return Err(Error::InvalidGPT("Header CRC32 Checksum").into());
        }
        self.header_crc32 = crc32;

        if self.my_lba != lba {
            return Err(Error::InvalidGPT("MyLBA").into());
        }

        let entries_size =
            (self.number_of_partition_entries * self.size_of_partition_entry) as usize;
        let entries = super::super::read(self.partition_entry_lba, 0, entries_size)?;
        let entry = PartitionEntry::get_ref(entries);
        if self.partition_entry_array_crc32 != entry.crc32(entries_size) {
            return Err(Error::InvalidGPT("Partition Entry Array CRC32 Checksum").into());
        }

        if lba == 1 {
            let backup = Self::get_mut(super::super::read(
                self.alternate_lba,
                0,
                size_of::<Self>(),
            )?);
            // let _ = backup.validate(self.alternate_lba)?;
            backup.delete()?;
        } else {
            entry.delete()?;
        }

        Ok(entries)
    }
}

struct PartitionEntry {
    partition_type_guid: GUID,

    unique_partition_guid: GUID,

    starting_lba: u64,
    ending_lba: u64,

    /// - Bit 0: Required Partition
    /// - Bit 1: No Block IO Protocol
    /// - Bit 2: Legacy BIOS Bootable
    /// - Bits 3 ..= 63: Reserved
    attributes: u64,

    /// Null-terminated string
    partition_name: [u8; 72],
}
impl Checksum for PartitionEntry {}
impl Memory for PartitionEntry {}

pub fn validate() -> Result<(), crate::Error> {
    let primary = Header::get_mut(super::super::read(1, 0, size_of::<Header>())?);
    let entries = primary.validate(1)?;
    for i in 0..primary.number_of_partition_entries as usize {
        let entry = PartitionEntry::get_ref(entries + i * primary.size_of_partition_entry as usize);
        if entry.partition_type_guid == GUID::UNUSED_PARTITION {
            continue;
        }
        crate::file_system::handle(entry.starting_lba, entry.ending_lba)?;
    }
    Ok(())
}
