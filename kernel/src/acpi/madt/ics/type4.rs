//! Local APIC NMI
//!
//! NMI stands for Non-Maskable Interrupt

use crate::{
    Output,
    error::{ACPI, Error},
    init_message,
};

use super::{FromAddr, Header, polarity_to_str, trigger_mode_to_str};

#[repr(C, packed)]
struct Type4 {
    header: Header,

    /// 0xFF for all
    acpi_processor_uid: u8,

    /// - Bits 0 ..= 1: Polarity
    /// - Bits 2 ..= 3: Trigger Mode
    /// - Bits 4 ..= 15: Reserved
    flags: u16,

    local_apic_lint: u8,
}
impl Type4 {
    fn handle(&self) -> Result<(), Error> {
        if self.header.length as usize != size_of::<Self>() {
            return Err(Error::ACPI(ACPI::InvalidLength));
        }
        if self.flags & !0b1111 != 0 {
            return Err(Error::ACPI(ACPI::InvalidReserved));
        }
        let polarity = (self.flags & 0b11) as u8;
        let trigger_mode = ((self.flags >> 2) & 0b11) as u8;
        init_message!(false, false, "NMI for ACPI Processor UID(");
        if self.acpi_processor_uid == 0xFF {
            init_message!(false, false, "All");
        } else {
            init_message!(false, false, self.acpi_processor_uid as usize);
        }
        init_message!(
            false,
            true,
            ") with polarity(",
            polarity_to_str(polarity),
            ") & trigger mode(",
            trigger_mode_to_str(trigger_mode),
            ") & Local APIC LINT(",
            self.local_apic_lint as usize,
            ") detected"
        );
        Ok(())
    }
}

pub fn handle(addr: u64) -> Result<(), Error> {
    Type4::get_ref(addr).handle()?;
    Ok(())
}
