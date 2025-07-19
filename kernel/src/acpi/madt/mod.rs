//! Multiple APIC Description Table

use core::ptr::{addr_of, read_unaligned};

use crate::{
    Error, Output, init_check, init_end, init_message, init_start, io::port, x86_64::apic::lapic,
};

use super::{FromAddr, Header};

mod ics;

pub const SIGNATURE: &[u8; 4] = b"APIC";

static mut ADDR: u64 = 0;

pub fn set_config(addr: u64) {
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
impl MADT {
    fn init(&self) -> Result<(), Error> {
        init_start!();
        self.header.init(*SIGNATURE)?;
        let structure_length = self.header.length as usize - size_of::<Self>();
        init_message!(
            true,
            false,
            "Local Interrupt Controller Address(",
            self.local_interrupt_controller_address as u64,
            ") detected..."
        );
        lapic::set_config(self.local_interrupt_controller_address);
        init_message!(false, true, "recorded");
        if self.flags & 1 == 1 {
            init_message!(true, false, "PIC detected...");
            port::out_byte(port::MASTER_PIC_DATA, 0xFF);
            port::out_byte(port::SLAVE_PIC_DATA, 0xFF);
            init_message!(false, true, "disabled");
        }

        let structures = addr_of!(self.interrupt_controller_structures) as *const u8;
        let mut offset = 0usize;
        while offset < structure_length {
            unsafe {
                let header = read_unaligned(structures.add(offset) as *const ics::Header);
                init_message!(true, false, "Type", header.type_ as usize, ": ");
                match header.type_ {
                    0 => ics::type0::handle(structures.add(offset) as u64)?,
                    1 => ics::type1::handle(structures.add(offset) as u64)?,
                    2 => ics::type2::handle(structures.add(offset) as u64)?,
                    4 => ics::type4::handle(structures.add(offset) as u64)?,
                    _ => init_message!(false, true, "Unprocessed"),
                }
                offset += header.length as usize;
            }
        }
        init_end!();
        Ok(())
    }
}

pub fn init() -> Result<(), Error> {
    unsafe {
        init_check!(ADDR);
        MADT::get_ref(ADDR).init()
    }
}
