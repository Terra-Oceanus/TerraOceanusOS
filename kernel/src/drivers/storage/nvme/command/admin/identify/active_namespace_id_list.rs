//! Active Namespace ID list

use crate::Memory;

impl super::super::super::Submission {
    /// - CNS: 0x02
    pub fn active_namespace_id_list() -> Result<Self, crate::Error> {
        let mut cmd = Self::identify()?;
        cmd.cdw10 = 0x02;
        Ok(cmd)
    }
}

struct List([u32; 1024]);
impl Memory for List {}
impl List {
    fn handle(&self) -> &Self {
        for &cur in self.0.iter().take_while(|&&id| id != 0) {}
        self
    }
}

pub fn handle(addr: u64) -> Result<(), crate::Error> {
    List::get_ref(addr).handle().delete()?;
    Ok(())
}
