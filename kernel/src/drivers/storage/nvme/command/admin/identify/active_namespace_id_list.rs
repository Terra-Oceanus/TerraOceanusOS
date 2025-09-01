//! Active Namespace ID list

use crate::memory::Memory;

use super::super::super::super::Error;

impl super::super::super::Submission {
    /// - CDW10.CNS: 0x02
    pub fn to_active_namespace_id_list(&mut self, addr: usize) {
        self.to_identify(addr);
        self.cdw10 = 0x02;
    }
}

impl super::super::super::Completion {
    pub fn to_active_namespace_id_list(&self) -> Result<(), Error> {
        match self.sct() {
            0x0 => match self.sc() {
                0x00 => return Ok(()),
                0x0B => return Err(Error::Queue("Invalid Namespace or Format")),
                _ => {}
            },
            _ => {}
        }
        Err(Error::Queue("Unknown Status Code Type"))
    }
}

pub struct List(pub [u32; 1024]);
impl Memory for List {}
