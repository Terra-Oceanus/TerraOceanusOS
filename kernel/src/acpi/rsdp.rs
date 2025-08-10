//! Root System Description Pointer

use super::{Checksum, Error, FromAddr};

static mut ADDR: u64 = 0;

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
impl RSDP1_0 {
    fn init(&self) -> Result<(), Error> {
        if self.signature != *b"RSD PTR " {
            return Err(Error::InvalidSignature);
        }
        if !self.checksum(size_of::<Self>()) {
            return Err(Error::InvalidChecksum);
        }
        if self.revision != 2 {
            return Err(Error::InvalidRevision);
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
impl RSDP {
    fn init(&self) -> Result<u64, Error> {
        self.rsdp1_0.init()?;
        if self.length != size_of::<Self>() as u32 {
            return Err(Error::InvalidLength);
        }
        if !self.checksum(self.length as usize) {
            return Err(Error::InvalidChecksum);
        }
        if self.reserved.iter().any(|&b| b != 0) {
            return Err(Error::InvalidReserved);
        }
        Ok(self.xsdt_address)
    }
}

pub fn init(addr: u64) -> Result<u64, Error> {
    if addr == 0 {
        return Err(Error::InvalidAddress);
    }
    unsafe { ADDR = addr };
    RSDP::get_ref(addr).init()
}
