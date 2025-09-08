//! Directory

#[repr(C, packed)]
pub struct Entry {
    /// Short name
    /// as 8-char name + 3-char extension
    ///
    /// First byte:
    /// - 0x00: from self is free
    /// - 0x05: instead use of 0xE5
    /// - 0xE5: self is free
    name: [u8; 11],

    /// Attributes
    /// - Bit 0: Read only
    /// - Bit 1: Hidden
    /// - Bit 2: System
    /// - Bit 3: Volume ID
    /// - Bit 4: Directory
    /// - Bit 5: Archive
    /// - Bits 6 ..= 7: Reserved
    attr: u8,

    reserved: u8,

    /// Creation time tenth
    /// - 0 ..= 199
    crt_time_tenth: u8,

    /// Creation time
    /// - Bits 0 ..= 4: 2-second count in 0 ..= 29
    /// - Bits 5 ..= 10: Minutes in 0 ..= 59
    /// - Bits 11 ..= 15: Hours in 0 ..= 23
    crt_time: u16,

    /// Creation date
    /// - Bits 0 ..= 4: Day of month in 1 ..= 31
    /// - Bits 5 ..= 8: Month of year in 1 ..= 12
    /// - Bits 9 ..= 15: Count of years from 1980 in 0 ..= 127
    crt_date: u16,

    /// Last access date
    /// - Bits 0 ..= 4: Day of month in 1 ..= 31
    /// - Bits 5 ..= 8: Month of year in 1 ..= 12
    /// - Bits 9 ..= 15: Count of years from 1980 in 0 ..= 127
    lst_acc_date: u16,

    /// High word of first cluster number
    fst_clus_hi: u16,

    /// Last write time
    /// - Bits 0 ..= 4: 2-second count in 0 ..= 29
    /// - Bits 5 ..= 10: Minutes in 0 ..= 59
    /// - Bits 11 ..= 15: Hours in 0 ..= 23
    wrt_time: u16,

    /// Last write date
    /// - Bits 0 ..= 4: Day of month in 1 ..= 31
    /// - Bits 5 ..= 8: Month of year in 1 ..= 12
    /// - Bits 9 ..= 15: Count of years from 1980 in 0 ..= 127
    wrt_date: u16,

    /// Low word of first cluster number
    fst_clus_lo: u16,

    file_size: u32,
}
impl Entry {
    pub fn name(&self) -> &[u8; 11] {
        &self.name
    }

    pub fn is_dir(&self) -> bool {
        self.attr & 0b1_0000 != 0
    }

    pub fn first_cluster(&self) -> usize {
        ((self.fst_clus_hi as usize) << 16) | self.fst_clus_lo as usize
    }
}
