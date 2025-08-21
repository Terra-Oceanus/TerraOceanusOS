//! PCI Express

use crate::{traits::FromAddr, x86_64::idt::Interrupt};

use super::super::super::pcie::capabilities::{Header, extended, msi_x};

pub fn handle_capabilities(base: u64, p_capabilities: u8) -> Result<(), crate::Error> {
    let mut count = 1;

    let mut offset: u64 = p_capabilities as u64;
    while offset != 0 {
        let header = Header::get_ref(base + offset);
        match header.id() {
            msi_x::CAPABILITY_ID => {
                msi_x::Capability::configure(
                    header as *const Header as u64,
                    base,
                    Interrupt::NVMe as u8,
                )?;
                count -= 1;
            }
            _ => {}
        }
        offset = header.next();
    }
    offset = 0x100;
    while offset != 0 {
        let header = extended::Header::get_ref(base + offset);
        match header.id() {
            0x0000 => break,
            _ => {}
        }
        offset = header.next();
    }

    if count != 0 {
        Err(super::Error::InvalidCapability.into())
    } else {
        Ok(())
    }
}
