//! Identify Controller

use crate::traits::FromAddr;

impl super::super::super::Submission {
    /// - CNS: 0x01
    pub fn identify_controller() -> Result<Self, crate::Error> {
        let mut cmd = Self::identify()?;
        cmd.cdw10 = 0x01;
        Ok(cmd)
    }
}

#[repr(C, packed)]
pub struct Data {
    /// PCI Vendor ID
    vid: u16,

    /// PCI Subsystem Vendor ID
    ssvid: u16,

    /// Serial Number
    /// as an ASCII string
    sn: [u8; 20],

    /// Model Number
    /// as an ASCII string
    mn: [u8; 40],

    /// Firmware Revision
    /// as an ASCII string
    fr: [u8; 8],

    /// Recommended Arbitration Burst
    rab: u8,

    /// IEEE OUI Identifier
    ieee: [u8; 3],

    /// Controller Multi-Path I/O and Namespace Sharing Capabilities
    /// - Bit 0: MPORTS for Multiple Ports
    /// - Bit 1: MCTRS for Multiple Controllers
    /// - Bit 2: FT for Function Type
    /// - Bit 3: ANARS for Asymmetric Namespace Access Reporting Support
    /// - Bits 4 ..= 7: Reserved
    cmic: u8,
}
impl FromAddr for Data {}
impl Data {
    pub fn vid(&self) -> u16 {
        self.vid
    }
}
