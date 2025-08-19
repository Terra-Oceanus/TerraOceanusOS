//! Controller

impl super::super::super::Submission {
    /// - CNS: 0x01
    pub fn identify_to_controller(&mut self) -> &mut Self {
        self.cdw10 = 0x01;
        self
    }
}
