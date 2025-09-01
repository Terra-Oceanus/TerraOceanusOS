//! GUID Partition Table

use crate::memory::Memory;

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

    /// LBA of this structure
    my_lba: u64,

    /// LBA of the alternate GPT Header
    alternate_lba: u64,

    first_usable_lba: u64,
    last_usable_lba: u64,

    disk_guid: u128,

    partition_entry_lba: u64,

    number_of_partition_entries: u32,

    size_of_partition_entry: u32,

    partition_entry_array_crc32: u32,
}
impl Memory for Header {}
impl Header {
    fn validate(&self) -> Result<(), Error> {
        if self.signature != 0x5452415020494645 {
            return Err(Error::InvalidGPT("Signature"));
        }
        Ok(())
    }
}

pub fn validate(addr: usize) -> Result<(), Error> {
    Header::get_ref(addr).validate()
}
