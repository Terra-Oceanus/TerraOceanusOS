//! Port

use core::arch::asm;

pub const MASTER_PIC_COMMAND: u16 = 0x20;
pub const MASTER_PIC_DATA: u16 = 0x21;

pub const PS2_DATA: u16 = 0x60;
pub const PS2_COMMAND: u16 = 0x64;

pub const SLAVE_PIC_COMMAND: u16 = 0xA0;
pub const SLAVE_PIC_DATA: u16 = 0xA1;

#[inline(always)]
pub fn in_byte(port: u16) -> u8 {
    let byte: u8;
    unsafe {
        asm!(
            "in al, dx",
            lateout("al") byte,
            in("dx") port,
        )
    };
    byte
}

#[inline(always)]
pub fn out_byte(port: u16, byte: u8) {
    unsafe {
        asm!(
            "out dx, al",
            in("dx") port,
            in("al") byte,
        )
    };
}
