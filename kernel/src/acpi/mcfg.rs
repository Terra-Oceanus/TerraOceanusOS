//! Memory-Mapped Configuration Space Base Address Description Table

use core::ptr::addr_of;

use crate::{
    Output,
    error::{ACPI, Error},
};

use super::{FromAddr, Header};

pub const SIGNATURE: &[u8; 4] = b"MCFG";

static mut ADDR: u64 = 0;

pub fn set_config(addr: u64) {
    unsafe { ADDR = addr }
}

#[repr(C, packed)]
struct BaseAddressAllocationStructure {
    base_address: u64,

    pci_segment_group_number: u16,

    start_pci_bus_number: u8,

    end_pci_bus_number: u8,

    reserved: [u8; 4],
}
impl BaseAddressAllocationStructure {
    fn base_address(&self) -> u64 {
        self.base_address
    }
    fn pci_segment_group_number(&self) -> usize {
        self.pci_segment_group_number as usize
    }
    fn start_pci_bus_number(&self) -> usize {
        self.start_pci_bus_number as usize
    }
    fn end_pci_bus_number(&self) -> usize {
        self.end_pci_bus_number as usize
    }
}

#[repr(C, packed)]
struct MCFG {
    header: Header,

    reserved: u64,

    structures: [BaseAddressAllocationStructure; 0],
}
impl MCFG {
    fn init(&self) -> Result<(), Error> {
        self.header.init(*SIGNATURE)?;
        let count = (self.header.length as usize - size_of::<MCFG>())
            / size_of::<BaseAddressAllocationStructure>();
        let structures = addr_of!(self.structures) as *const BaseAddressAllocationStructure;
        for i in 0..count {
            let structure = unsafe { &*structures.add(i) };
            structure.base_address().output();
            " ".output();
            (structure.pci_segment_group_number() as usize).output();
            " ".output();
            (structure.start_pci_bus_number() as usize).output();
            " ".output();
            (structure.end_pci_bus_number() as usize).output();
            "\n".output();
        }
        Ok(())
    }
}

pub fn init() -> Result<(), Error> {
    unsafe {
        if ADDR == 0 {
            return Err(Error::ACPI(ACPI::InvalidAddress));
        }
        MCFG::get_ref(ADDR).init()
    }
}
