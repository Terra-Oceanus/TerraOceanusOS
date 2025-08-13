//! Non-Volatile Memory Express

use crate::{io::text::Output, traits::FromAddr};

#[repr(C)]
struct Registers {
    /// Controller Capabilities
    /// - Bits 0 ..= 15: MQES for Maximum Queue Entries Supported (RO)
    /// - Bit 16: CQR for Contiguous Queues Required (RO)
    /// - Bits 17 ..= 18: AMS for Arbitration Mechanism Supported (RO)
    ///   - Bit 0: WRRUPC for Weighted Round Robin with Urgent Priority Class
    ///   - Bit 1: VS for Vendor Specific
    /// - Bits 19 ..= 23: Reserved
    /// - Bits 24 ..= 31: TO for Timeout (RO)
    /// - Bits 32 ..= 35: DSTRD for Doorbell Stride (RO)
    /// - Bit 36: NSSRS for NVM Subsystem Reset Supported (RO)
    /// - Bits 37 ..= 44: CSS for Command Sets Supported (RO)
    ///   - Bit 0: NCSS for NVM Command Set Support
    ///   - Bits 1 ..= 5: Reserved
    ///   - Bit 6: IOCSS for I/O Command Set Support
    ///   - Bit 7: NOIOCSS for No I/O Command Set Support
    /// - Bit 45: BPS for Boot Partition Support (RO)
    /// - Bits 46 ..= 47: CPS for Controller Power Scope (RO)
    ///   - 0b00: Not Reported
    ///   - 0b01: Controller Scope
    ///   - 0b10: Domain Scope
    ///   - 0b11: NVM Subsystem Scope
    /// - Bits 48 ..= 51: MPSMIN for Memory Page Size Minimum (RO)
    /// - Bits 52 ..= 55: MPSMAX for Memory Page Size Maximum (RO)
    /// - Bit 56: PMRS for Persistent Memory Region Supported (RO)
    /// - Bit 57: CMBS for Controller Memory Buffer Supported (RO)
    /// - Bit 58: NSSS for NVM Subsystem Shutdown Supported (RO)
    /// - Bits 59 ..= 60: CRMS for Controller Ready Modes Supported (RO)
    ///   - Bit 0: CRWMS for Controller Ready With Media Support
    ///   - Bit 1: CRIMS for Controller Ready Independent of Media Support
    /// - Bit 61: NSSES for NVM Subsystem Shutdown Enhancements Supported (RO)
    /// - Bits 62 ..= 63: Reserved
    cap: u64,

    /// Version
    /// - Bits 0 ..= 7: TER for Tertiary Version
    /// - Bits 8 ..= 15: MNR for Minor Version
    /// - Bits 16 ..= 32: MJR for Major Version
    vs: u32,

    /// Interrupt Mask Set
    intms: u32,

    /// Interrupt Mask Clear
    intmc: u32,

    /// Controller Configuration
    cc: u32,

    reserved0: u32,

    /// Controller Status
    csts: u32,

    /// NVM Subsystem Reset
    nssr: u32,

    /// Admin Queue Attributes
    aqa: u32,

    /// Admin Submission Queue
    asq: u64,

    /// Admin Completion Queue
    acq: u64,

    /// Controller Memory Buffer Location
    cmbloc: u32,

    /// Controller Memory Buffer Size
    cmbsz: u32,

    /// Boot Partition Information
    bpinfo: u32,

    /// Boot Partition Read Select
    bprsel: u32,

    /// Boot Partition Memory Buffer Location
    bpmbl: u64,

    /// Controller Memory Buffer Memory Space Control
    cmbmsc: u64,

    /// Controller Memory Buffer Status
    cmbsts: u32,

    /// Controller Memory Buffer Elasticity Buffer Size
    cmbebs: u32,

    /// Controller Memory Buffer Sustained Write Throughput
    cmbswtp: u32,

    /// NVM Subsystem Shutdown
    nssd: u32,

    /// Controller Ready Timeouts
    crto: u32,

    reserved1: [u8; 0xE00 - 0x6C],

    /// Persistent Memory Capabilities
    pmrcap: u32,

    /// Persistent Memory Region Control
    pmrctl: u32,

    /// Persistent Memory Region Status
    pmrsts: u32,

    /// Persistent Memory Region Elasticity Buffer Size
    pmrebs: u32,

    /// Persistent Memory Region Sustained Write Throughput
    pmrswtp: u32,

    /// Persistent Memory Region Controller Memory Space Control Lower
    pmrmscl: u32,

    /// Persistent Memory Region Controller Memory Space Control Upper
    pmrmscu: u32,

    reserved2: [u8; 0x1000 - 0xE1C],
}
impl FromAddr for Registers {}
impl Registers {
    fn init(&self) {}
}

pub fn init(addr: u64) {
    Registers::get_ref(addr).init();
}
