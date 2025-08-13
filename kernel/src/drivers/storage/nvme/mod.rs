//! Non-Volatile Memory Express

use core::ptr::read_volatile;

static mut ADDR: u64 = 0;

pub fn set_config(addr: u64) {
    unsafe { ADDR = addr }
}

#[repr(u16)]
enum Register {
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
    CAP = 0x0,

    /// Version
    /// - Bits 0 ..= 7: TER for Tertiary Version
    /// - Bits 8 ..= 15: MNR for Minor Version
    /// - Bits 16 ..= 32: MJR for Major Version
    VS = 0x8,

    /// Interrupt Mask Set
    /// - Bits 0 ..= 31: IVMS for Interrupt Vector Mask Set
    INTMS = 0xC,

    /// Interrupt Mask Clear
    /// - Bits 0 ..= 31: IVMC for Interrupt Vector Mask Clear
    INTMC = 0x10,

    /// Controller Configuration
    /// - Bit 0: EN for Enable (R/W)
    /// - Bits 1 ..= 3: Reserved
    /// - Bits 4 ..= 6: CSS for I/O Command Set Selected (R/W)
    ///   - 0b000:
    ///     - NVM Command Set if CAP.CSS.NCSS is set
    ///     - Reserved if CAP.CSS.NCSS is clear
    ///   - 0b001 ..= 0b101: Reserved
    ///   - 0b110:
    ///     - All Supported I/O Command Sets if CAP.CSS.IOCSS is set
    ///     - Reserved if CAP.CSS.IOCSS is clear
    ///   - 0b111:
    ///     - Admin Command Set only if CAP.CSS.NOIOCSS is set
    ///     - Reserved if CAP.CSS.NOIOCSS is clear
    /// - Bits 7 ..= 10: MPS for Memory Page Size (R/W)
    /// - Bits 11 ..= 13: AMS for Arbitration Mechanism Selected (R/W)
    ///   - 0b000: Round Robin
    ///   - 0b001: Weighted Round Robin with Urgent Priority Class
    ///   - 0b010 ..= 0b110: Reserved
    ///   - 0b111: Vendor Specific
    /// - Bits 14 ..= 15: SHN for Shutdown Notification *(R/W)*
    ///   - 0b00: No notification & No effect
    ///   - 0b01: Normal shutdown notification
    ///   - 0b10: Abrupt shutdown notification
    ///   - 0b11: Reserved
    /// - Bits 16 ..= 19: IOSQES for I/O Submission Queue Entry Size (R/W if I/O queues supported)
    /// - Bits 20 ..= 23: IOCQES for I/O Completion Queue Entry Size (R/W if I/O queues supported)
    /// - Bit 24: CRIME for Controller Ready Independent of Media Enable
    /// - Bits 25 ..= 31: Reserved
    CC = 0x14,

    /// Controller Status
    /// - Bit 0: RDY for Ready (RO)
    /// - Bit 1: CFS for Controller Fatal Status (RO)
    /// - Bits 2 ..= 3: SHST for Shutdown Status (RO)
    ///   - 0b00: Normal operation
    ///   - 0b01: Shutdown processing in progress
    ///   - 0b10: Shutdown processing complete
    ///   - 0b11: Reserved
    /// - Bit 4: NSSRO for NVM Subsystem Reset Occurred (R/W1C)
    /// - Bit 5: PP for Processing Paused (RO)
    /// - Bit 6: ST for Shutdown Type (RO)
    /// - Bit 7 ..= 31: Reserved
    CSTS = 0x1C,

    /// NVM Subsystem Reset
    /// - Bits 0 ..= 31: NSSRC for NVM Subsystem Reset Control
    NSSR = 0x20,

    /// Admin Queue Attributes
    /// - Bits 0 ..= 11: ASQS for Admin Submission Queue Size (R/W)
    /// - Bits 12 ..= 15: Reserved
    /// - Bits 16 ..= 27: ACQS for Admin Completion Queue Size (R/W)
    /// - Bits 28 ..= 31: Reserved
    AQA = 0x24,

    /// Admin Submission Queue
    /// - Bits 0 ..= 11: Reserved
    /// - Bits 12 ..= 63: ASQB for Admin Submission Queue Base (R/W)
    ASQ = 0x28,

    /// Admin Completion Queue
    /// - Bits 0 ..= 11: Reserved
    /// - Bits 12 ..= 63: ACQB for Admin Completion Queue Base (R/W)
    ACQ = 0x30,

    /// Controller Memory Buffer Location
    /// - Bits 0 ..= 2: BIR for Base Indicator Register (RO)
    /// - Bit 3: CQMMS for CMB Queue Mixed Memory Support (RO)
    /// - Bit 4: CQPDS for CMB Queue Physically Discontiguous Support (RO)
    /// - Bit 5: CDPMLS for CMB Data Pointer Mixed Locations Support (RO)
    /// - Bit 6: CDPCILS for CMB Data Pointer and Command Independent Locations Support (RO)
    /// - Bit 7: CDMMMS for CMB Data Metadata Mixed Memory Support (RO)
    /// - Bit 8: CQDA for CMB Queue Dword Alignment (RO)
    /// - Bits 9 ..= 11: Reserved
    /// - Bits 12 ..= 31: OFST for Offset (RO)
    CMBLOC = 0x38,

    /// Controller Memory Buffer Size
    /// - Bit 0: SQS for Submission Queue Support (RO)
    /// - Bit 1: CQS for Completion Queue Support (RO)
    /// - Bit 2: LISTS for PRP SGL List Support (RO)
    /// - Bit 3: RDS for Read Data Support (RO)
    /// - Bit 4: WDS for Write Data Support (RO)
    /// - Bits 5 ..= 7: Reserved
    /// - Bits 8 ..= 11: SZU for Size Units (RO)
    ///   - 0x0: 4 KiB
    ///   - 0x1: 64 KiB
    ///   - 0x2: 1 MiB
    ///   - 0x3: 16 MiB
    ///   - 0x4: 256 MiB
    ///   - 0x5: 4 GiB
    ///   - 0x6: 64 GiB
    ///   - 0x7 ..= 0xF: Reserved
    /// - Bits 12 ..= 31: SZ for Size (RO)
    CMBSZ = 0x3C,

    /// Boot Partition Information
    /// - Bits 0 ..= 14: BPSZ for Boot Partition Size (RO)
    /// - Bits 15 ..= 23: Reserved
    /// - Bits 24 ..= 25: BRS for Boot Read Status (RO)
    ///   - 0b00: No Boot Partition read operation requested
    ///   - 0b01: Boot Partition read in progress
    ///   - 0b10: Boot Partition read completed successfully
    ///   - 0b11: Error completing Boot Partition read
    /// - Bits 26 ..= 30: Reserved
    /// - Bit 31: ABPID for Active Boot Partition ID (RO)
    BPINFO = 0x40,

    /// Boot Partition Read Select
    /// - Bits 0 ..= 9: BPRSZ for Boot Partition Read Size (R/W)
    /// - Bits 10 ..= 29: BPROF for Boot Partition Read Offset (R/W)
    /// - Bit 30: Reserved
    /// - Bit 31: BPID for Boot Partition Identifer (R/W)
    BPRSEL = 0x44,

    /// Boot Partition Memory Buffer Location
    /// - Bits 0 ..= 11: Reserved
    /// - Bits 12 ..= 63: BMBBA for Boot Partition Memory Buffer Base Address (R/W)
    BPMBL = 0x48,

    /// Controller Memory Buffer Memory Space Control
    /// - Bit 0: CRE for Capabilities Registers Enabled (R/W)
    /// - Bit 1: CMSE for Controller Memory Space Enable (R/W)
    /// - Bits 2 ..= 11: Reserved
    /// - Bits 12 ..= 63: CBA for Controller Base Address (R/W)
    CMBMSC = 0x50,

    /// Controller Memory Buffer Status
    /// - Bit 0: CBAI for Controller Base Address Invalid (RO)
    /// - Bits 1 ..= 31: Reserved
    CMBSTS = 0x58,

    /// Controller Memory Buffer Elasticity Buffer Size
    /// - Bits 0 ..= 3: CMBSZU for CMB Elasticity Buffer Size Units (RO)
    ///   - 0x0: Bytes
    ///   - 0x1: 1 KiB
    ///   - 0x2: 1 MiB
    ///   - 0x3: 1 GiB
    ///   - 0x4 ..= 0xF: Reserved
    /// - Bit 4: CMBRBB for CMB Read Bypass Behavior (RO)
    /// - Bits 5 ..= 7: Reserved
    /// - Bits 8 ..= 31: CMBWBZ for CMB Elasticity Buffer Size Base (RO)
    CMBEBS = 0x5C,

    /// Controller Memory Buffer Sustained Write Throughput
    /// - Bits 0 ..= 3: CMBSWTU for CMB Sustained Write Throughput Units (RO)
    ///   - 0x0: Bytes/s
    ///   - 0x1: 1 KiB/s
    ///   - 0x2: 1 MiB/s
    ///   - 0x3: 1 GiB/s
    ///   - 0x4 ..= 0xF: Reserved
    /// - Bits 4 ..= 7: Reserved
    /// - Bits 8 ..= 31: CMBSWTV for CMB Sustained Write Throughput (RO)
    CMBSWTP = 0x60,

    /// NVM Subsystem Shutdown
    /// - Bits 0 ..= 31: NSSC for NVM Subsystem Shutdown Control
    NSSD = 0x64,

    /// Controller Ready Timeouts
    /// - Bits 0 ..= 15: CRWMT for Controller Ready With Media Timeout (RO)
    /// - Bits 16 ..= 31: CRIMT for Controller Ready Independent of Media Timeout (RO)
    CRTO = 0x68,

    /// Persistent Memory Capabilities
    /// - Bits 0 ..= 2: Reserved
    /// - Bit 3: RDS for Read Data Support (RO)
    /// - Bit 4: WDS for Write Data Support (RO)
    /// - Bits 5 ..= 7: BIR for Base Indicator Register (RO)
    /// - Bits 8 ..= 9: PMRTU for Persistent Memory Region Time Units (RO)
    ///   - 0b00: 500 ms
    ///   - 0b01: mins
    ///   - 0b10 ..= 0b11: Reserved
    /// - Bits 10 ..= 13: PMRWBM for Persistent Memory Region Write Barrier Mechanisms (RO)
    ///   - Bit 0: CMR for Completion of Memory Read
    ///   - Bit 1: CPMRSTSR for Completion of PMRSTS Read
    ///   - Bits 2 ..= 3: Reserved
    /// - Bits 14 ..= 15: Reserved
    /// - Bits 16 ..= 23: PMRTO for Persistent Memory Region Timeout (RO)
    /// - Bit 24: CMSS for Controller Memory Space Supported (RO)
    /// - Bits 25 ..= 31: Reserved
    PMRCAP = 0xE00,

    /// Persistent Memory Region Control
    /// - Bit 0: EN for Enable (R/W)
    /// - Bits 1 ..= 31: Reserved
    PMRCTL = 0xE04,

    /// Persistent Memory Region Status
    /// - Bits 0 ..= 7: ERR for Error (RO)
    /// - Bit 8: NRDY for Not Ready (RO)
    /// - Bits 9 ..= 11: HSTS for Health Status (RO)
    ///   - 0b000: Normal Operation
    ///   - 0b001: Restore Error
    ///   - 0b010: Read Only
    ///   - 0b011: Unreliable
    ///   - 0b100 ..= 0b111: Reserved
    /// - Bit 12: CBAI for Controller Base Address Invalid (RO)
    /// - Bits 13 ..= 31: Reserved
    PMRSTS = 0xE08,

    /// Persistent Memory Region Elasticity Buffer Size
    /// - Bits 0 ..= 3: PMRSZU for PMR Elasticity Buffer Size Units (RO)
    ///   - 0x0: Bytes
    ///   - 0x1: 1 KiB
    ///   - 0x2: 1 MiB
    ///   - 0x3: 1 GiB
    ///   - 0x4 ..= 0xF: Reserved
    /// - Bit 4: PMRRBB for PMR Read Bypass Behavior (RO)
    /// - Bits 5 ..= 7: Reserved
    /// - Bits 8 ..= 31: PMRWBZ for PMR Elasticity Buffer Size Base (RO)
    PMREBS = 0xE0C,

    /// Persistent Memory Region Sustained Write Throughput
    /// - Bits 0 ..= 3: PMRSWTU for PMR Sustained Write Throughput Units (RO)
    /// - Bits 4 ..= 7: Reserved
    /// - Bits 8 ..= 31: PMRSWTV for PMR Sustained Write Throughput (RO)
    PMRSWTP = 0xE10,

    /// Persistent Memory Region Controller Memory Space Control Lower
    /// - Bit 0: Reserved
    /// - Bit 1: CMSE for Controller Memory Space Enable (R/W)
    /// - Bits 2 ..= 11: Reserved
    /// - Bits 12 ..= 31: CBA for Controller Base Address (R/W)
    PMRMSCL = 0xE14,

    /// Persistent Memory Region Controller Memory Space Control Upper
    /// - Bits 0 ..= 31: CBA for Controller Base Address (R/W)
    PMRMSCU = 0xE18,
}
impl Register {
    fn read(self) -> u64 {
        if matches!(
            self,
            Register::CAP | Register::ASQ | Register::ACQ | Register::BPMBL | Register::CMBMSC
        ) {
            unsafe { read_volatile((ADDR + self as u64) as *const u64) }
        } else {
            unsafe { read_volatile((ADDR + self as u64) as *const u32) as u64 }
        }
    }
}

pub fn init() {}