//! Extended System Description Table

use core::ptr::{addr_of, read_unaligned};

use crate::Memory;

use super::{Error, Header, fadt, madt, mcfg};

static mut ADDR: usize = 0;

pub fn set_config(addr: usize) {
    unsafe { ADDR = addr }
}

#[repr(C, packed)]
struct XSDT {
    header: Header,

    entries: [u64; 0],
}
impl Memory for XSDT {}
impl XSDT {
    fn init(&self) -> Result<(), Error> {
        self.header.init(*b"XSDT")?;
        let count = (self.header.length as usize - size_of::<Self>()) / size_of::<u64>();
        let entries = addr_of!(self.entries) as *const u64;
        for i in 0..count {
            let entry = unsafe { read_unaligned(entries.add(i)) } as usize;
            match &unsafe { &*(entry as *const Header) }.signature {
                fadt::SIGNATURE => fadt::set_config(entry),
                madt::SIGNATURE => madt::set_config(entry),
                mcfg::SIGNATURE => mcfg::set_config(entry),
                _ => {}
            };
        }
        Ok(())
    }
}

pub fn init(addr: usize) -> Result<(), Error> {
    if addr == 0 {
        return Err(Error::InvalidAddress);
    }
    unsafe { ADDR = addr };
    XSDT::get_ref(addr).init()
}
