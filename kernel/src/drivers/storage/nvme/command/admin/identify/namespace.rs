//! Identify Controller Data Structure

use crate::Memory;

impl super::super::super::Submission {
    /// - CNS: 0x00
    pub fn to_identify_namespace_data_structure(&mut self, id: u32) {
        self.nsid = id;
    }
}

#[repr(C, packed)]
pub struct Data {
    /// Namespace Size
    nsze: u64,
}
impl Memory for Data {}
impl Data {
    pub fn handle(&self) {}
}
