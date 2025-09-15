//! Identify Controller Data Structure

use crate::{mem::Memory, types::guid::GUID};

impl super::super::super::Submission {
    /// - CDW10.CNS: 0x01
    pub fn to_identify_controller_data_structure(&mut self, addr: usize) {
        self.to_identify(addr);
        self.cdw10 = 0x01;
    }
}

#[repr(C, packed)]
struct Data {
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

    /// Read Recovery Levels Supported
    /// - Bit 4: Default
    rrls: u16,

    /// Boot Partition Capabilities
    /// - Bits 0 ..= 1: RPMBBPWPS for RPMB Boot Partition Write Protection Support
    ///   - 0b00: Not specified
    ///   - 0b01: Not supported
    ///   - 0b10: Supported
    ///   - 0b11: Reserved
    /// - Bit 2: SFBPWPS for Set Features Boot Partition Write Protection Support
    ///   - 0: Not supported
    ///   - 1: Supported
    /// - Bits 3 ..= 7: Reserved
    bpcap: u8,

    reserved0: u8,

    /// NVM Subsystem Shutdown Latency
    nssl: u32,

    reserved1: u16,

    /// Power Loss Signaling Information
    /// - Bit 0: PLSEPF for PLS Emergency Power Fail
    /// - Bit 1: PLSFQ for PLS Forced Quiescence
    /// - Bits 2 ..= 7: Reserved
    plsi: u8,

    /// Controller Type
    /// - 0x0: Reserved
    /// - 0x1: I/O controller
    /// - 0x2: Discovery controller
    /// - 0x3: Administrative controller
    /// - 0x4 ..= 0xFF: Reserved
    cntrltype: u8,

    /// FRU Globally Unique Identifier
    fguid: GUID,

    /// Command Retry Delay Time 1
    crdt1: u16,

    /// Command Retry Delay Time 2
    crdt2: u16,

    /// Command Retry Delay Time 3
    crdt3: u16,

    /// Controller Reachability Capabilities
    /// - Bit 0: RRSUP for Reachability Reporting Supported
    /// - Bit 1: RGIDC for Reachability Group ID Changeable
    /// - Bits 2 ..= 7: Reserved
    crcap: u8,

    reserved2: [u8; 118],

    /// NVM Subsystem Report
    /// - Bit 0: NVMESD for NVMe Storage Device
    /// - Bit 1: NVMEE for NVMe Enclosure
    /// - Bits 2 ..= 7: Reserved
    nvmsr: u8,

    /// VPD Write Cycle Information
    /// - Bits 0 ..= 6: VWCR for VPD Write Cycles Remaining
    /// - Bit 7: VWCRV for VPD Write Cycles Remaining Valid
    vwci: u8,

    /// Management Endpoint Capabilities
    /// - Bit 0: TWPME for 2-Wire Port Management Endpoint
    /// - Bit 1: PCIEME for PCIe Port Management Endpoint
    /// - Bits 2 ..= 7: Reserved
    mec: u8,

    /// Optional Admin Command Support
    /// - Bit 0: SSRS for Security Send Receive Supported
    /// - Bit 1: FNVMS for Format NVM Supported
    /// - Bit 2: FWDS for Firmware Download Supported
    /// - Bit 3: NMS for Namespace Management Supported
    /// - Bit 4: DSTS for Device Self-test Supported
    /// - Bit 5: DIRS for Directives Supported
    /// - Bit 6: NSRS for NVMe-MI Send Receive Supported
    /// - Bit 7: VMS for Virtualization Management Supported
    /// - Bit 8: DBCS for Doorbell Buffer Config Supported
    /// - Bit 9: GLSS for Get LBA Status Supported
    /// - Bit 10: CFLS for Command and Feature Lockdown Supported
    /// - Bit 11: HMLMS for Host Managed Live Migration Support
    /// - Bits 12 ..= 15: Reserved
    oacs: u16,

    /// Abort Command Limit
    acl: u8,

    /// Asynchronous Event Request Limit
    aerl: u8,

    /// Firmware Updates
    /// - Bit 0: FFSRO for First Firmware Slot Read Only
    /// - Bits 1 ..= 3: NOFS for Number Of Firmware Slots
    /// - Bit 4: FAWR for Firmware Activation Without Reset
    /// - Bit 5: SMUD for Support Multiple Update Detection
    /// - Bits 6 ..= 7: Reserved
    frmw: u8,

    /// Log Page Attributes
    /// - Bit 0: SMARTS for SMART Support
    /// - Bit 1: CSES for Commands Supported and Effects Support
    /// - Bit 2: LPEDS for Log Page Extended Data Support
    /// - Bit 3: TS for Telemetry Support
    /// - Bit 4: PES for Persistent Event Support
    /// - Bit 5: MLPS for Miscellaneous Log Page Support
    /// - Bit 6: DA4S for Data Area 4 Support
    /// - Bit 7: Reserved
    lpa: u8,

    /// Error Log Page Entries
    elpe: u8,

    /// Number of Power States Support
    npss: u8,

    /// Admin Vendor Specific Command Configuration
    /// - Bit 0: VSCF for Vendor Specific Command Format
    /// - Bits 1 ..= 7: Reserved
    avscc: u8,

    /// Autonomous Power State Transition Attributes
    /// - Bit 0: APTS for Autonomous Power Transition Support
    /// - Bits 1 ..= 7: Reserved
    apsta: u8,

    /// Warning Composite Temperature Threshold
    wctemp: u16,

    /// Critical Composite Temperature Threshold
    cctemp: u16,

    /// Maximum Time for Firmware Activation
    mtfa: u16,

    /// Host Memory Buffer Preferred Size
    hmpre: u32,

    /// Host Memory Buffer Minimum Size
    hmmin: u32,

    /// Total NVM Capacity
    tnvmcap: u128,

    /// Unallocated NVM Capacity
    unvmcap: u128,

    /// Replay Protected Memory Block Support
    /// - Bits 0 ..= 2: NRPMBU for Number of RPMB Units
    /// - Bits 3 ..= 5: AUTHM for Authentication Method
    ///   - 0b000: HMAC SHA-256
    ///   - 0b001 ..= 0b111: Reserved
    /// - Bits 6 ..= 15: Reserved
    /// - Bits 16 ..= 23: TSZE for Total Size
    /// - Bits 24 ..= 31: ASZE for Access Size
    rpmbs: u32,

    /// Extended Device Self-test Time
    edstt: u16,

    /// Device Self-test Options
    /// - Bit 0: SDSO for Single Device Self-test Operation
    /// - Bit 1: HIRS for Host-Initiated Refresh Support
    /// - Bits 2 ..= 7: Reserved
    dsto: u8,

    /// Firmware Update Granularity
    fwug: u8,

    /// Keep Alive Support
    kas: u16,

    /// Host Controlled Thermal Management Attributes
    /// - Bit 0: HCTMS for Host Controlled Thermal Management Support
    /// - Bits 1 ..= 15: Reserved
    hctma: u16,

    /// Minimum Thermal Management Temperature
    mntmt: u16,

    /// Maximum Thermal Management Temperature
    mxtmt: u16,

    /// Sanitize Capabilities
    /// - Bit 0: CES for Crypto Erase Support
    /// - Bit 1: BES for Block Erase Support
    /// - Bit 2: OWS for Overwrite Support
    /// - Bit 3: VERS for Verification Support
    /// - Bits 4 ..= 28: Reserved
    /// - Bit 29: NDI for No-Deallocate Inhibited
    /// - Bits 30 ..= 31: NODMMAS for No-Deallocate Modifies Media After Sanitize
    sanicap: u32,

    /// Host Memory Buffer Minimum Descriptor Entry Size
    hmminds: u32,

    /// Host Memory Maximum Descriptors Entries
    hmmaxd: u16,

    /// NVM Set Identifier Maximum
    nsetidmax: u16,

    /// Endurance Group Identifier Maximum
    endgidmax: u16,

    /// ANA Transition Time
    anatt: u8,

    /// Asymmetric Namespace Access Capabilities
    /// - Bit 0: RANAOS for Report ANA Optimized State
    /// - Bit 1: RANANOS for Report ANA Non-Optimized State
    /// - Bit 2: RANAIS for Report ANA Inaccessible State
    /// - Bit 3: RANAPLS for Report ANA Persistent Loss State
    /// - Bit 4: RANACS for Report ANA Change State
    /// - Bit 5: Reserved
    /// - Bit 6: ANAGIDLWAS for ANA Group ID Locked When Attached Support
    /// - Bit 7: ANAGIDS for ANA Group ID Support
    anacap: u8,

    /// ANA Group Identifier Maximum
    anagrpmax: u32,

    /// Number of ANA Group Identifiers
    nanagrpid: u32,

    /// Persistent Event Log Size
    pels: u32,

    /// Domain Identifier
    did: u16,

    /// Key Per I/O Capabilities
    /// - Bit 0: KPIOS for Key Per I/O Supported
    /// - Bit 1: KPIOSC for Key Per I/O Scope
    /// - Bits 2 ..= 7: Reserved
    kpioc: u8,

    reserved3: u8,

    /// Maximum Processing Time for Firmware Activation Without Reset
    mptfawr: u16,

    reserved4: [u8; 6],

    /// Max Endurance Group Capacity
    megcap: u128,

    /// Temperature Threshold Hysteresis Attributes
    /// - Bits 0 ..= 2: TMPTHMH for Temperature Threshold Maximum Hysteresis
    /// - Bits 3 ..= 7: Reserved
    tmpthha: u8,

    reserved5: u8,

    /// Command Quiesce Time
    cqt: u16,

    reserved6: [u8; 124],

    /// Submission Queue Entry Size
    /// - Bits 0 ..= 3: MINSQES for Minimum I/O Submission Queue Entry Size
    /// - Bits 4 ..= 7: MAXSQES for Maximum I/O Submission Queue Entry Size
    sqes: u8,

    /// Completion Queue Entry Size
    /// - Bits 0 ..= 3: MINCQES for Minimum I/O Completion Queue Entry Size
    /// - Bits 4 ..= 7: MAXCQES for Maximum I/O Completion Queue Entry Size
    cqes: u8,

    /// Maximum Outstanding Commands
    maxcmd: u16,

    /// Number of Namespaces
    ///
    /// The maximum value of a valid NSID
    nn: u32,

    /// Optional NVM Command Support
    /// - Bit 0: NVMCMPS for Compare Command Support
    /// - Bit 1: NVMWUSV for Write Uncorrectable Support Variants
    /// - Bit 2: NVMDSMSV for Dataset Management Support Variants
    /// - Bit 3: NVMWZSV for Write Zeroes Support Variants
    /// - Bit 4: SSFS for Save and Select Feature Support
    /// - Bit 5: RESERVS for Reservations Support
    /// - Bit 6: TSS for Timestamp Support
    /// - Bit 7: NVMVFYS for Verify Support
    /// - Bit 8: NVMCPYS for Copy Support
    /// - Bit 9: NVMCSA for NVM Copy Single Atomicity
    /// - Bit 10: NVMAFC for NVM All Fast Copy
    /// - Bit 11: MAXWZD for Maximum Write Zeroes with Deallocate
    /// - Bit 12: NSXS for Namespace Zeroes Support
    /// - Bits 13 ..= 15: Reserved
    oncs: u16,

    /// Fused Operation Support
    /// - Bit 0: FCWS for Fused Compare and Write Supported
    /// - Bits 1 ..= 7: Reserved
    fuses: u16,

    /// Format NVM Attributes
    /// - Bit 0: FNS for Format Namespace Scope
    /// - Bit 1: SENS for Secure Erase Namespace Scope
    /// - Bit 2: CRYES for Cryptographic Erase Supported
    /// - Bit 3: FNVMBS for Format NVM Broadcast Support
    /// - Bits 4 ..= 7: Reserved
    fna: u8,

    /// Volatile Write Cache
    /// - Bit 0: VWCP for Volatile Write Cache Present
    /// - Bits 1 ..= 2: FB for Flush Behavior
    /// - Bits 3 ..= 7: Reserved
    vwc: u8,

    /// Atomic Write Unit Normal
    awun: u16,

    /// Atomic Write Unit Power Fail
    awupf: u16,

    /// I/O Command Set Vendor Specific Command Configuration
    /// - Bit 0: SNVSCF for Same NVM Vendor Specific Command Format
    /// - Bits 1 ..= 7: Reserved
    icsvscc: u8,

    /// Namespace Write Protection Capabilities
    /// - Bit 0: NWPWPS for No Write Protect and Write Protect Support
    /// - Bit 1: WPUPCS for Write Protect Until Power Cycle Support
    /// - Bit 2: PWPS for Permanent Write Protect Support
    /// - Bits 3 ..= 7: Reserved
    nwpc: u8,

    /// Atomic Compare & Write Unit
    acwu: u16,

    /// Copy Descriptor Formats Supported
    /// - Bit 0: CDF0S for Copy Descriptor Format 0 Support
    /// - Bit 1: CDF1S for Copy Descriptor Format 1 Support
    /// - Bit 2: CDF2S for Copy Descriptor Format 2 Support
    /// - Bit 3: CDF3S for Copy Descriptor Format 3 Support
    /// - Bit 4: CDF4S for Copy Descriptor Format 4 Support
    /// - Bits 5 ..= 15: Reserved
    cdfs: u16,

    /// SGL Support
    /// - Bits 0 ..= 1: SGLS for SGL Support
    ///   - 0b00: Not supported
    ///   - 0b01: Supported with no alignment nor granularity requirement for Data Blocks
    ///   - 0b10: Supported with a dword alignment and granularity requirement for Data Blocks
    ///   - 0b11: Reserved
    /// - Bit 2: KSDBDS for Keyed SGL Data Block Descriptor Support
    /// - Bits 3 ..= 7: Reserved
    /// - Bits 8 ..= 15: SDT for SGL Descriptor Threshold
    /// - Bit 16: SBBDS for SGL Bit Bucket Descriptor Supported
    /// - Bit 17: MBA for Metadata Buffer Alignment
    /// - Bit 18: LLDTS for Length Larger than Data Transfer Support
    /// - Bit 19: MSDS for MPTR SGL Descriptor Support
    /// - Bit 20: SAOS for SGL Address Offset Supported
    /// - Bit 21: TSDBDS for Transport SGL Data Block Descriptor Support
    /// - Bits 22 ..= 31: Reserved
    sgls: u32,

    /// Maximum Number of Allowed Namespaces
    mnan: u32,

    /// Maximum Domain Namespace Attachments
    maxdna: u128,

    /// Maximum I/O Controller Namespace Attachments
    maxcna: u32,

    /// Optimal Aggregated Queue Depth
    oaqd: u32,

    /// Recommended Host-Initiated Refresh Interval
    rhiri: u8,

    /// Host-Initiated Refresh Time
    hirt: u8,

    /// Controller Maximum Memory Range Tracking Descriptors
    cmmrtd: u16,

    /// NVM Subsystem Maximum Memory Range Tracking Descriptors
    nmmrtd: u16,

    /// Minimum Memory Range Tracking Granularity
    minmrtg: u8,

    /// Maximum Memory Range Tracking Granularity
    maxmrtg: u8,

    /// Tracking Attributes
    /// - Bit 0: THMCS for Track Host Memory Changes Support
    /// - Bit 1: TUDCS for Track User Data Changes Support
    /// - Bit 2: MRTLL for Memory Range Tracking Length Limit
    /// - Bits 3 ..= 7: Reserved
    trattr: u8,

    reserved7: u8,

    /// Maximum Controller User Data Migration Queues
    mcudmq: u16,

    /// Maximum NVM Subsystem User Data Migration Queues
    mnsudmq: u16,

    /// Maximum CDQ Memory Ranges
    mcmr: u16,

    /// NVM Subsystem Maximum CDQ Memory Ranges
    nmcmr: u16,

    /// Maximum Controller Data Queue PRP Count
    mcdqpc: u16,

    reserved8: [u8; 180],

    /// NVM Subsystem NVMe Qualified Name
    /// as a UTF-8 null-terminated string
    subnqn: [u8; 256],

    reserved9: [u8; 1024],

    /// Power State Descriptor
    psd: [PowerStateDescriptorData; 32],

    /// Vendor Specific
    vs: [u8; 1024],
}
impl Memory for Data {}
impl Data {
    fn handle(&self) -> &Self {
        self
    }
}

#[repr(C)]
struct PowerStateDescriptorData {
    /// - Bits 0 ..= 15: MP for Maximum Power
    /// - Bits 16 ..= 23: Reserved
    /// - Bit 24: MXPS for Max Power Scale
    /// - Bit 25: NOPS for Non-Operational State
    /// - Bits 26 ..= 31: Reserved
    /// - Bits 32 ..= 63: ENLAT for Entry Latency
    qword0: u64,

    /// - Bits 0 ..= 31: EXLAT for Exit Latency
    /// - Bits 32 ..= 36: RRT for Relative Read Throughput
    /// - Bits 37 ..= 39: Reserved
    /// - Bits 40 ..= 44: RRL for Relative Read Latency
    /// - Bits 45 ..= 47: Reserved
    /// - Bits 48 ..= 52: RWT for Relative Write Throughput
    /// - Bits 53 ..= 55: Reserved
    /// - Bits 56 ..= 60: RWL for Relative Write Latency
    /// - Bits 61 ..= 63: Reserved
    qword1: u64,

    /// - Bits 0 ..= 15: IDLP for Idle Power
    /// - Bits 16 ..= 21: Reserved
    /// - Bits 22 ..= 23: IPS for Idle Power Scale
    ///   - 0b00: Not reported
    ///   - 0b01: 0.0001 W
    ///   - 0b10: 0.01 W
    ///   - 0b11: Reserved
    /// - Bits 24 ..= 31: Reserved
    /// - Bits 32 ..= 47: ACTP for Active Power
    /// - Bits 48 ..= 50: APW for Active Power Workload
    /// - Bits 51 ..= 53: Reserved
    /// - Bits 54 ..= 55: APS for Active Power Scale
    ///   - 0b00: Not reported
    ///   - 0b01: 0.0001 W
    ///   - 0b10: 0.01 W
    ///   - 0b11: Reserved
    /// - Bits 56 ..= 63: EPFRT for Emergency Power Fail Recovery Time
    ///   - 0: Not reported
    ///   - 1 ..= 99: Time value
    ///   - 100 ..= 255: Reserved
    qword2: u64,

    /// - Bits 0 ..= 7: FQVT for Forced Quiescence Vault Time
    ///   - 0: Not reported
    ///   - 1 ..= 99: Time value
    ///   - 100 ..= 255: Reserved
    /// - Bits 8 ..= 15: EPFVT for Emergency Power Fail Vault Time
    ///   - 0: Not reported
    ///   - 1 ..= 99: Time value
    ///   - 100 ..= 255: Reserved
    /// - Bits 16 ..= 19: EPFRTS for Emergency Power Fail Recovery Time Scale
    ///   - 0x0: 1 us
    ///   - 0x1: 10 us
    ///   - 0x2: 100 us
    ///   - 0x3: 1 ms
    ///   - 0x4: 10 ms
    ///   - 0x5: 100 ms
    ///   - 0x6: 1 s
    ///   - 0x7: 10 s
    ///   - 0x8: 100 s
    ///   - 0x9: 1_000 s
    ///   - 0xA: 10_000 s
    ///   - 0xB: 100_000 s
    ///   - 0xC: 1_000_000 s
    ///   - 0xD ..= 0xF: Reserved
    /// - Bits 20 ..= 23: FQVTS for Forced Quiescence Vault Time Scale
    ///   - 0x0: 1 us
    ///   - 0x1: 10 us
    ///   - 0x2: 100 us
    ///   - 0x3: 1 ms
    ///   - 0x4: 10 ms
    ///   - 0x5: 100 ms
    ///   - 0x6: 1 s
    ///   - 0x7: 10 s
    ///   - 0x8: 100 s
    ///   - 0x9: 1_000 s
    ///   - 0xA: 10_000 s
    ///   - 0xB: 100_000 s
    ///   - 0xC: 1_000_000 s
    ///   - 0xD ..= 0xF: Reserved
    /// - Bits 24 ..= 27: EPFVTS for Emergency Power Fail Vault Time Scale
    ///   - 0x0: 1 us
    ///   - 0x1: 10 us
    ///   - 0x2: 100 us
    ///   - 0x3: 1 ms
    ///   - 0x4: 10 ms
    ///   - 0x5: 100 ms
    ///   - 0x6: 1 s
    ///   - 0x7: 10 s
    ///   - 0x8: 100 s
    ///   - 0x9: 1_000 s
    ///   - 0xA: 10_000 s
    ///   - 0xB: 100_000 s
    ///   - 0xC: 1_000_000 s
    ///   - 0xD ..= 0xF: Reserved
    /// - Bits 28 ..= 63: Reserved
    qword3: u64,
}

pub fn handle(addr: usize) -> Result<(), crate::Error> {
    Data::get_ref(addr).handle().delete()?;
    Ok(())
}
