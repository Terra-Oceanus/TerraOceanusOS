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

    /// Maximum Data Transfer Size
    mdts: u8,

    /// Controller ID
    cntlid: u16,

    /// Version
    ver: u32,

    /// RTD3 Resume Latency
    rtd3r: u32,

    /// RTD3 Entry Latency
    rtd3e: u32,

    /// Optional Asynchronous Events Supported
    /// - Bits 0 ..= 7: Reserved
    /// - Bit 8: NSAN for Attached Namespace Attribute Notices
    /// - Bit 9: FAN for Firmware Activation Notices
    /// - Bit 10: Reserved
    /// - Bit 11: ANACN for Asymmetric Namespace Access Change Notices
    /// - Bit 12: PLEAN for Predictable Latency Event Aggregate Log Change Notices
    /// - Bit 13: LSIAN for LBA Status Information Alert Notices
    /// - Bit 14: EGEAN for Endurance Group Event Aggregate Log Page Change Notices
    /// - Bit 15: NNSS for Normal NVM Subsystem Shutdown
    /// - Bit 16: TTHR for Temperature Threshold Hysteresis Recovery
    /// - Bit 17: RGCNS for Reachability Groups Change Notices Support
    /// - Bit 18: Reserved
    /// - Bit 19: ANSAN for Allocated Namespace Attribute Notices
    /// - Bits 20 ..= 26: Reserved
    /// - Bit 27: ZDCN for Zone Descriptor Changed Notices
    /// - Bits 28 ..= 30: Reserved
    /// - Bit 31: DLPCN for Discovery Log Page Change Notification
    oaes: u32,

    /// Controller Attributes
    /// - Bit 0: HIDS for Host Identifier Support
    /// - Bit 1: NOPSPM for Non-Operational Power State Permissive Mode
    /// - Bit 2: NSETS for NVM Sets
    /// - Bit 3: RRLVLS for Read Recovery Levels
    /// - Bit 4: EGS for Endurance Groups
    /// - Bit 5: PLM for Predictable Latency Mode
    /// - Bit 6: TBKAS for Traffic Based Keep Alive Support
    /// - Bit 7: NG for Namespace Granularity
    /// - Bit 8: SQA for SQ Associations
    /// - Bit 9: ULIST for UUID List
    /// - Bit 10: MDS for Multi-Domain Subsystem
    /// - Bit 11: FCM for Fixed Capacity Management
    /// - Bit 12: VCN for Variable Capacity Management
    /// - Bit 13: DEG for Delete Endurance Group
    /// - Bit 14: DNVMS for Delete NVM Set
    /// - Bit 15: ELBAS for Extended LBA Formats Supported
    /// - Bit 16: MEM for MDTS and Size Limits Exclude Metadata
    /// - Bit 17: HMBR for HMB Restrict Non-Operational Power State Access
    /// - Bit 18: RHII for Reservations and Host Identifier Interaction
    /// - Bit 19: FDPS for Flexible Data Placement Support
    /// - Bits 20 ..= 31: Reserved
    ctratt: u32,
}
impl FromAddr for Data {}
impl Data {
    pub fn handle(&self) {}
}
