//! Interrupt Source Override

use crate::{
    Output,
    error::{ACPI, Error},
    init_message,
    x86_64::apic::ioapic,
};

use super::{FromAddr, Header, polarity_to_str, trigger_mode_to_str};

#[repr(C, packed)]
struct Type2 {
    header: Header,

    /// 0 for ISA
    bus: u8,

    source: u8,

    global_system_interrupt: u32,

    /// - Bits 0 ..= 1: Polarity
    /// - Bits 2 ..= 3: Trigger Mode
    /// - Bits 4 ..= 15: Reserved
    flags: u16,
}
impl Type2 {
    fn handle(&self) -> Result<(), Error> {
        if self.header.length as usize != size_of::<Self>() {
            return Err(Error::ACPI(ACPI::InvalidLength));
        }
        if self.flags & !0b1111 != 0 {
            return Err(Error::ACPI(ACPI::InvalidReserved));
        }
        let src = self.source;
        let dst = self.global_system_interrupt;
        let polarity = (self.flags & 0b11) as u8;
        let trigger_mode = ((self.flags >> 2) & 0b11) as u8;
        init_message!(false, false, "Override ");
        if src as u32 != dst {
            init_message!(false, false, "from IRQ(", src as usize, ") to ",)
        }
        init_message!(
            false,
            false,
            "GSI(",
            dst as usize,
            ") with polarity(",
            polarity_to_str(polarity),
            ") & trigger mode(",
            trigger_mode_to_str(trigger_mode)
        );
        if self.bus != 0 {
            init_message!(false, false, ") on bus(", self.bus as usize);
        }
        init_message!(false, false, ") detected...");
        ioapic::handle_override(src, dst, polarity, trigger_mode)?;
        init_message!(false, true, "handled");
        Ok(())
    }
}

pub fn handle(addr: u64) -> Result<(), Error> {
    Type2::get_ref(addr).handle()?;
    Ok(())
}
