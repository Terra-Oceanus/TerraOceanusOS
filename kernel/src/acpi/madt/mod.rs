//! Multiple APIC Description Table

use core::ptr::{addr_of, read_unaligned};

use crate::{io::port, mem::Memory};

use super::{Error, Header};

mod ics;

pub const SIGNATURE: &[u8; 4] = b"APIC";

static mut ADDR: usize = 0;

pub fn set_config(addr: usize) {
    unsafe { ADDR = addr }
}

#[repr(C, packed)]
struct MADT {
    header: Header,

    local_interrupt_controller_address: u32,

    /// - Bit 0: PCAT_COMPAT
    /// - Bits 1 ..= 31: Reserved
    flags: u32,

    interrupt_controller_structures: [u8; 0],
}
impl Memory for MADT {}
impl MADT {
    fn init(&self) -> Result<u32, crate::Error> {
        self.header.init(*SIGNATURE)?;

        // Programmable Interrupt Controller
        if self.flags & 1 == 1 {
            port::out_byte(port::MASTER_PIC_DATA, 0xFF);
            port::out_byte(port::SLAVE_PIC_DATA, 0xFF);
        }

        let mut offset = 0usize;
        let structures = addr_of!(self.interrupt_controller_structures) as *const u8;
        while offset < self.header.length as usize - size_of::<Self>() {
            unsafe {
                let entry = structures.add(offset);
                let header = read_unaligned(entry as *const ics::Header);
                match header.type_ {
                    0 => ics::type0::handle(entry as usize)?,
                    1 => ics::type1::handle(entry as usize)?,
                    2 => ics::type2::handle(entry as usize)?,
                    4 => ics::type4::handle(entry as usize)?,
                    _ => {}
                }
                offset += header.length as usize;
            }
        }
        Ok(self.local_interrupt_controller_address)
    }
}

pub fn init() -> Result<u32, crate::Error> {
    unsafe { MADT::get_ref(ADDR).init() }
}
