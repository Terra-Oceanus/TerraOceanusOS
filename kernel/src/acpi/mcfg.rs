//! Memory-Mapped Configuration Space Base Address Description Table

use core::slice;

use crate::drivers::pcie;

use super::{Error, FromAddr, Header};

pub const SIGNATURE: &[u8; 4] = b"MCFG";

static mut ADDR: u64 = 0;

pub fn set_config(addr: u64) {
    unsafe { ADDR = addr }
}

#[repr(C, packed)]
struct AllocationStructure {
    base_address: u64,

    pci_segment_group: u16,

    start_pci_bus: u8,
    end_pci_bus: u8,

    reserved: u32,
}

#[repr(C, packed)]
struct MCFG {
    header: Header,

    reserved: u64,

    structures: [AllocationStructure; 0],
}
impl MCFG {
    fn init(&self) -> Result<(), Error> {
        self.header.init(*SIGNATURE)?;
        for structure in unsafe {
            slice::from_raw_parts(
                self.structures.as_ptr(),
                (self.header.length as usize - size_of::<Self>())
                    / size_of::<AllocationStructure>(),
            )
        } {
            for bus in 0..=(structure.end_pci_bus - structure.start_pci_bus) {
                let bus_addr = structure.base_address + ((bus as u64) << 20);
                for device in 0..32 {
                    let device_addr = bus_addr + (device << 15);
                    let device_header = pcie::Header::get_ref(device_addr);
                    if !device_header.is_present() {
                        continue;
                    }
                    if !device_header.is_multi_function() {
                        continue;
                    }
                    for function in 1..8 {
                        let function_header = pcie::Header::get_ref(device_addr + (function << 12));
                        if !function_header.is_present() {
                            continue;
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

pub fn init() -> Result<(), Error> {
    unsafe {
        if ADDR == 0 {
            return Err(Error::InvalidAddress);
        }
        MCFG::get_ref(ADDR).init()
    }
}
