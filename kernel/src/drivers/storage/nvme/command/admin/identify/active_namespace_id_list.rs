//! Active Namespace ID list

use crate::memory::Memory;

impl super::super::super::Submission {
    /// - CNS: 0x02
    pub fn to_active_namespace_id_list(&mut self) {
        self.cdw10 = 0x02;
    }
}

pub struct List(pub [u32; 1024]);
impl Memory for List {}
