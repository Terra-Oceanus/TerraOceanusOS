//! Root System Description Pointer

use crate::{math::Checksum, memory::Memory};

use super::Error;

const FAKE_SIGNATURE: &[u8; 4] = b"RSDP";

static mut ADDR: usize = 0;

#[repr(C, packed)]
struct RSDP1_0 {
    /// "RSD PTR "
    signature: [u8; 8],

    checksum: u8,

    oem_id: [u8; 6],

    /// - 0: ACPI 1.0
    /// - 2: ACPI 2.0+
    revision: u8,

    /// Deprecated
    rsdt_address: u32,
}
impl Checksum for RSDP1_0 {}
impl Memory for RSDP1_0 {}
impl RSDP1_0 {
    fn init(&self) -> Result<(), Error> {
        if self.signature != *b"RSD PTR " {
            return Err(Error::InvalidSignature(*FAKE_SIGNATURE));
        }
        if !self.checksum(size_of::<Self>()) {
            return Err(Error::InvalidChecksum(*FAKE_SIGNATURE));
        }
        if self.revision != 2 {
            return Err(Error::InvalidRevision(*FAKE_SIGNATURE));
        }
        Ok(())
    }
}

#[repr(C, packed)]
struct RSDP {
    rsdp1_0: RSDP1_0,

    length: u32,

    xsdt_address: u64,

    extended_checksum: u8,

    reserved: [u8; 3],
}
impl Checksum for RSDP {}
impl Memory for RSDP {}
impl RSDP {
    fn init(&self) -> Result<usize, Error> {
        self.rsdp1_0.init()?;
        if self.length != size_of::<Self>() as u32 {
            return Err(Error::InvalidLength(*FAKE_SIGNATURE));
        }
        if !self.checksum(self.length as usize) {
            return Err(Error::InvalidChecksum(*FAKE_SIGNATURE));
        }
        Ok(self.xsdt_address as usize)
    }
}

pub fn init(addr: usize) -> Result<usize, Error> {
    if addr == 0 {
        return Err(Error::InvalidAddress(*FAKE_SIGNATURE));
    }
    unsafe { ADDR = addr };
    RSDP::get_ref(addr).init()
}
