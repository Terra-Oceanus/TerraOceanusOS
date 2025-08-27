//! Identify Controller Data Structure

use crate::Memory;

impl super::super::super::Submission {
    /// - CNS: 0x00
    pub fn identify_namespace_data_structure(id: u32) -> Result<Self, crate::Error> {
        let mut cmd = Self::identify()?;
        cmd.nsid = id;
        Ok(cmd)
    }
}

#[repr(C, packed)]
struct Data {
    /// Namespace Size
    nsze: u64,
}
impl Memory for Data {}
impl Data {
    fn handle(&self) -> &Self {
        self
    }
}

pub fn handle(addr: usize) -> Result<(), crate::Error> {
    Data::get_ref(addr).handle().delete()?;
    Ok(())
}
