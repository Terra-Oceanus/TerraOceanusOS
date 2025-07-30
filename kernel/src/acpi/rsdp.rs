//! Root System Description Pointer

use crate::{
    Output,
    error::{ACPI, Error},
    init_check, init_end, init_message, init_start,
};

use super::{Checksum, FromAddr, xsdt};

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

    /// *Deprecated*
    rsdt_address: u32,
}
impl RSDP1_0 {
    fn init(&self) -> Result<(), Error> {
        if self.signature != *b"RSD PTR " {
            return Err(Error::ACPI(ACPI::InvalidSignature));
        }
        if !self.checksum(size_of::<Self>()) {
            return Err(Error::ACPI(ACPI::InvalidChecksum));
        }
        if self.revision != 2 {
            return Err(Error::ACPI(ACPI::InvalidRevision));
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
impl RSDP {
    fn init(&self) -> Result<(), Error> {
        init_start!();
        self.rsdp1_0.init()?;
        if self.length != size_of::<Self>() as u32 {
            return Err(Error::ACPI(ACPI::InvalidLength));
        }
        if !self.checksum(self.length as usize) {
            return Err(Error::ACPI(ACPI::InvalidChecksum));
        }
        if self.reserved.iter().any(|&b| b != 0) {
            return Err(Error::ACPI(ACPI::InvalidReserved));
        }
        init_message!(true, false, "Table XSDT detected...");
        xsdt::set_config(self.xsdt_address);
        init_message!(false, true, "recorded");
        init_end!();
        Ok(())
    }
}

pub fn init(addr: u64) -> Result<(), Error> {
    unsafe { ADDR = addr };
    init_check!(addr);
    RSDP::get_ref(addr).init()
}
