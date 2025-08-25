//! Controller

impl super::super::super::Submission {
    /// - CNS: 0x01
    pub fn identify_controller() -> Result<Self, crate::Error> {
        let mut identify = Self::identify()?;
        identify.cdw10 = 0x01;
        Ok(identify)
    }
}
