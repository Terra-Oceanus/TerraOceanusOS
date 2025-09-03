//! Boot Sector

use crate::{io::text::Output, memory::Memory};

#[repr(C, packed)]
pub struct BS {
    jmp_boot: [u8; 3],

    oem_name: [u8; 8],

    bpb: BIOSParameterBlock,

    drv_num: u8,

    reserved0: u8,

    boot_sig: u8,

    vol_id: u32,

    vol_lab: [u8; 11],

    fil_sys_type: [u8; 8],

    reserved1: [u8; 420],

    /// 0xAA55
    signature: u16,
}
impl Memory for BS {}
impl BS {
    pub fn validate(&self) -> bool {}
}

#[repr(C, packed)]
struct BIOSParameterBlock {
    byts_per_sec: u16,

    sec_per_clus: u8,

    rsvd_sec_cnt: u16,

    num_fats: u8,

    root_ent_cnt: u16,

    tot_sec_16: u16,

    media: u8,

    fat_sz_16: u16,

    sec_per_trk: u16,

    num_heads: u16,

    hidd_sec: u32,

    tot_sec_32: u32,

    fat_sz_32: u32,

    ext_flags: u16,

    fs_ver: u16,

    root_clus: u32,

    fs_info: u16,

    bk_boot_sec: u16,

    reserved: [u8; 12],
}
