//! Extended System Description Table

use core::ptr::{addr_of, read_unaligned};

use crate::{Error, Output, init_check, init_end, init_message, init_start};

use super::{FromAddr, Header, fadt, madt};

static mut ADDR: u64 = 0;

pub fn set_config(addr: u64) {
    unsafe { ADDR = addr }
}

#[repr(C, packed)]
struct XSDT {
    header: Header,

    entries: [u64; 0],
}
impl XSDT {
    fn init(&self) -> Result<(), Error> {
        init_start!();
        self.header.init(*b"XSDT")?;
        let n = (self.header.length as usize - size_of::<Self>()) / size_of::<u64>();
        let entries = addr_of!(self.entries) as *const u64;
        for i in 0..n {
            let entry = unsafe { read_unaligned(entries.add(i)) };
            let signature = &unsafe { &*(entry as *const Header) }.signature;
            init_message!(
                true,
                true,
                "Table with signature(",
                str::from_utf8(signature).map_err(|_e| Error::InvalidSignature)?,
                ") detected...",
                match signature {
                    fadt::SIGNATURE => {
                        fadt::set_config(entry);
                        "recorded"
                    }
                    madt::SIGNATURE => {
                        madt::set_config(entry);
                        "recorded"
                    }
                    _ => "ignored",
                }
            );
        }
        init_end!();
        Ok(())
    }
}

pub fn init() -> Result<(), Error> {
    unsafe {
        init_check!(ADDR);
        XSDT::get_ref(ADDR).init()
    }
}
