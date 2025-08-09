//! Advanced Configuration and Power Interface

use core::slice::from_raw_parts;

use crate::error::{ACPI, Error};

mod dsdt;
mod facs;
mod fadt;
pub mod madt;
pub mod mcfg;
mod rsdp;
mod xsdt;

trait FromAddr: Sized {
    fn get_ref(addr: u64) -> &'static Self {
        unsafe { &*(addr as *const Self) }
    }
}
impl<T> FromAddr for T {}

trait Checksum {
    fn checksum(&self, size: usize) -> bool {
        unsafe {
            from_raw_parts(self as *const Self as *const u8, size)
                .iter()
                .copied()
                .fold(0u8, u8::wrapping_add)
                == 0
        }
    }
}

pub fn init(rsdp_addr: u64) -> Result<(), Error> {
    let xsdt_addr = rsdp::init(rsdp_addr)?;
    xsdt::init(xsdt_addr)?;
    fadt::init()?;
    Ok(())
}

#[repr(C, packed)]
struct Header {
    signature: [u8; 4],

    length: u32,

    revision: u8,

    checksum: u8,

    oem_id: [u8; 6],
    oem_table_id: [u8; 8],
    oem_revision: u32,

    creator_id: [u8; 4],
    creator_revision: [u8; 4],
}
impl Checksum for Header {}
impl Header {
    fn init(&self, signature: [u8; 4]) -> Result<(), Error> {
        if self.signature != signature {
            return Err(Error::ACPI(ACPI::InvalidSignature));
        }
        if !self.checksum(self.length as usize) {
            return Err(Error::ACPI(ACPI::InvalidChecksum));
        }
        Ok(())
    }
}
