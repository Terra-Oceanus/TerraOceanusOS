//! Active Namespace ID list

impl super::super::super::Submission {
    /// - CNS: 0x02
    pub fn active_namespace_id_list() -> Result<Self, crate::Error> {
        let mut cmd = Self::identify()?;
        cmd.cdw10 = 0x02;
        Ok(cmd)
    }
}
