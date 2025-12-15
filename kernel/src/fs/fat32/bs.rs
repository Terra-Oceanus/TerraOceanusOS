//! Boot Sector

use crate::mem::Memory;

#[repr(C, packed)]
pub struct BS {
    /// Jump instruction to boot code
    /// - 0xEB 0x?? 0x90
    /// - 0xE9 0x?? 0x??
    jmp_boot: [u8; 3],

    oem_name: [u8; 8],

    bpb: BIOSParameterBlock,

    /// Int 0x13 Drive number
    drv_num: u8,

    reserved0: u8,

    /// - 0x29
    boot_sig: u8,

    /// Volume serial number
    vol_id: u32,

    /// Volume label
    vol_lab: [u8; 11],

    /// - "FAT32   "
    fil_sys_type: [u8; 8],

    reserved1: [u8; 420],

    /// 0xAA55
    signature: u16,
}
impl Memory for BS {}
impl BS {
    pub fn validate(&self) -> u16 {
        if self.signature == 0xAA55
            && self.bpb.validate()
            && match self.boot_sig {
                0x29 => self.fil_sys_type == *b"FAT32   ",
                _ => true,
            }
            && self.reserved0 == 0
        {
            self.bpb.fs_info
        } else {
            0
        }
    }

    pub fn sector_bytes(&self) -> usize {
        self.bpb.byts_per_sec as usize
    }

    pub fn cluster_bytes(&self) -> usize {
        self.bpb.sec_per_clus as usize * self.sector_bytes()
    }

    fn data_sector_offset(&self) -> usize {
        (self.bpb.rsvd_sec_cnt as usize + self.bpb.num_fats as usize * self.bpb.fat_sz_32 as usize)
            * self.sector_bytes()
    }

    fn cluster_offset(&self, cluster: usize) -> usize {
        self.data_sector_offset() + (cluster - 2) * self.cluster_bytes()
    }

    pub fn root_offset(&self) -> usize {
        self.cluster_offset(self.bpb.root_clus as usize)
    }
}

#[repr(C, packed)]
struct BIOSParameterBlock {
    /// Bytes per sector
    /// - 512
    /// - 1024
    /// - 2048
    /// - 4096
    byts_per_sec: u16,

    /// Sectors per cluster
    /// - 1
    /// - 2
    /// - 4
    /// - 8
    /// - 16
    /// - 32
    /// - 64
    /// - 128
    sec_per_clus: u8,

    /// Count of reserved sectors in Reserved region
    /// - 32
    rsvd_sec_cnt: u16,

    /// Number of FAT data structures
    num_fats: u8,

    reserved0: u32,

    /// - 0xF0: Removable
    /// - 0xF8: Fixed
    /// - 0xF9
    /// - 0xFA
    /// - 0xFB
    /// - 0xFC
    /// - 0xFD
    /// - 0xFE
    /// - 0xFF
    media: u8,

    reserved1: u16,

    /// Sectors per track for Int 0x13
    sec_per_trk: u16,

    /// Number of heads for Int 0x13
    num_heads: u16,

    /// Count of hidden sectors for Int 0x13
    hidd_sec: u32,

    /// Totol count of sectors
    tot_sec_32: u32,

    /// Count of sectors occupied by Self
    fat_sz_32: u32,

    /// - Bits 0 ..= 3: Active FAT number if mirroring is disabled
    /// - Bits 4 ..= 6: Reserved
    /// - Bit 7: Mirroring
    ///   - 0: All FATs are mirrored at runtime
    ///   - 1: Only the specified active FAT is used
    /// - Bits 8 ..= 15: Reserved
    ext_flags: u16,

    /// - Bits 0 ..= 7: Minor revision number
    /// - Bits 8 ..= 15: Major revision number
    fs_ver: u16,

    /// Cluster number of the first cluster of the root directory
    root_clus: u32,

    /// Sector number of FSINFO structure
    fs_info: u16,

    /// Sector number of a copy of the boot record
    bk_boot_sec: u16,

    reserved2: [u8; 12],
}
impl BIOSParameterBlock {
    fn validate(&self) -> bool {
        matches!(self.byts_per_sec, 512 | 1024 | 2048 | 4096)
            && matches!(self.sec_per_clus, 1 | 2 | 4 | 8 | 16 | 32 | 64 | 128)
            && self.byts_per_sec as u32 * self.sec_per_clus as u32 <= 32 * 1024
            && self.rsvd_sec_cnt == 32
            && self.num_fats == 2
            && matches!(
                self.media,
                0xF0 | 0xF8 | 0xF9 | 0xFA | 0xFB | 0xFC | 0xFD | 0xFE | 0xFF
            )
            && self.tot_sec_32 != 0
            && self.reserved0 == 0
            && self.reserved1 == 0
            && self.reserved2 == [0; 12]
    }
}
