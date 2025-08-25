//! Non-Volatile Memory Express

use core::{
    hint::spin_loop,
    ptr::{read_volatile, write_volatile},
};

mod command;
mod error;
pub mod pcie;
mod queue;

pub use error::Error;
use queue::Queue;

use crate::{io::text::Output, traits::FromAddr};

static mut NVME: NVMe = NVMe::null();

pub fn set_config(addr: u64) {
    unsafe { NVME.pcie_addr = addr };
}

struct NVMe {
    pcie_addr: u64,

    base: u64,
    msi_x: u64,

    dstrd: u8,

    admin: Queue,
}
impl NVMe {
    const fn null() -> Self {
        Self {
            pcie_addr: 0,
            base: 0,
            msi_x: 0,
            dstrd: 0,
            admin: Queue::null(),
        }
    }

    fn init(&mut self) -> Result<(), crate::Error> {
        if self.pcie_addr == 0 {
            return Err(Error::InvalidAddress.into());
        }

        let pcie = crate::drivers::pcie::Type0::get_ref(self.pcie_addr);
        self.base = pcie.bar(0)?;

        Register::CC.write(0);
        while (Register::CSTS.read() & 0b1) == 1 {
            spin_loop();
        }

        self.dstrd = ((Register::CAP.read() >> 32) & 0xF) as u8;

        let admin_size = command::admin::ENTRY_COUNT;
        let (asq, acq) = self.admin.init(admin_size, admin_size)?;
        Register::AQA.write(((admin_size as u64 - 1) << 16) | (admin_size as u64 - 1));
        Register::ASQ.write(asq);
        Register::ACQ.write(acq);
        Register::CC.write({
            let cap = Register::CAP.read();
            1 | ({
                let css = ((cap >> 37) & 0xFF) as u8;
                if css & 0b1000_0000 != 0 {
                    0b111
                } else if css & 0b100_0000 != 0 {
                    0b110
                } else if css & 0b1 != 0 {
                    0b000
                } else {
                    return Err(Error::InvalidRegisterValue("CAP.CSS").into());
                }
            } << 4)
                | ({
                    if (((cap >> 48) & 0xF)..=((cap >> 52) & 0xF)).contains(&0) {
                        0x0
                    } else {
                        return Err(Error::InvalidRegisterValue("CAP.MPSMIN & CAP.MPSMAX").into());
                    }
                } << 7)
                | ({
                    let ams = ((cap >> 17) & 0b11) as u8;
                    if ams & 0b1 != 0 {
                        0b001
                    } else if ams & 0b10 != 0 {
                        0b111
                    } else {
                        0b000
                    }
                } << 11)
        });
        while (Register::CSTS.read() & 0b1) == 0 {
            spin_loop();
        }

        Register::INTMC.write(0xFFFFFFFF);

        let mut submission = command::Submission::identify_controller()?;
        let completion = self.admin.submit(&mut submission).poll();
        if completion.sct() == 0 && completion.sc() == 0 {
            let data = command::admin::identify::controller::Data::get_ref(submission.prp1());
            (data.vid() as u64).output();
        }

        Ok(())
    }
}

#[repr(u16)]
enum Register {
    /// Controller Capabilities
    /// - Bits 0 ..= 15: MQES for Maximum Queue Entries Supported
    /// - Bit 16: CQR for Contiguous Queues Required
    /// - Bits 17 ..= 18: AMS for Arbitration Mechanism Supported
    ///   - Bit 0: WRRUPC for Weighted Round Robin with Urgent Priority Class
    ///   - Bit 1: VS for Vendor Specific
    /// - Bits 19 ..= 23: Reserved
    /// - Bits 24 ..= 31: TO for Timeout
    /// - Bits 32 ..= 35: DSTRD for Doorbell Stride
    /// - Bit 36: NSSRS for NVM Subsystem Reset Supported
    /// - Bits 37 ..= 44: CSS for Command Sets Supported
    ///   - Bit 0: NCSS for NVM Command Set Support
    ///   - Bits 1 ..= 5: Reserved
    ///   - Bit 6: IOCSS for I/O Command Set Support
    ///   - Bit 7: NOIOCSS for No I/O Command Set Support
    /// - Bit 45: BPS for Boot Partition Support
    /// - Bits 46 ..= 47: CPS for Controller Power Scope
    ///   - 0b00: Not Reported
    ///   - 0b01: Controller Scope
    ///   - 0b10: Domain Scope
    ///   - 0b11: NVM Subsystem Scope
    /// - Bits 48 ..= 51: MPSMIN for Memory Page Size Minimum
    /// - Bits 52 ..= 55: MPSMAX for Memory Page Size Maximum
    /// - Bit 56: PMRS for Persistent Memory Region Supported
    /// - Bit 57: CMBS for Controller Memory Buffer Supported
    /// - Bit 58: NSSS for NVM Subsystem Shutdown Supported
    /// - Bits 59 ..= 60: CRMS for Controller Ready Modes Supported
    ///   - Bit 0: CRWMS for Controller Ready With Media Support
    ///   - Bit 1: CRIMS for Controller Ready Independent of Media Support
    /// - Bit 61: NSSES for NVM Subsystem Shutdown Enhancements Supported
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
    /// - Bit 0: EN for Enable
    /// - Bits 1 ..= 3: Reserved
    /// - Bits 4 ..= 6: CSS for I/O Command Set Selected
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
    /// - Bits 7 ..= 10: MPS for Memory Page Size
    /// - Bits 11 ..= 13: AMS for Arbitration Mechanism Selected
    ///   - 0b000: Round Robin
    ///   - 0b001: Weighted Round Robin with Urgent Priority Class
    ///   - 0b010 ..= 0b110: Reserved
    ///   - 0b111: Vendor Specific
    /// - Bits 14 ..= 15: SHN for Shutdown Notification *(R/W)*
    ///   - 0b00: No notification & No effect
    ///   - 0b01: Normal shutdown notification
    ///   - 0b10: Abrupt shutdown notification
    ///   - 0b11: Reserved
    /// - Bits 16 ..= 19: IOSQES for I/O Submission Queue Entry Size
    /// - Bits 20 ..= 23: IOCQES for I/O Completion Queue Entry Size
    /// - Bit 24: CRIME for Controller Ready Independent of Media Enable
    /// - Bits 25 ..= 31: Reserved
    CC = 0x14,

    /// Controller Status
    /// - Bit 0: RDY for Ready
    /// - Bit 1: CFS for Controller Fatal Status
    /// - Bits 2 ..= 3: SHST for Shutdown Status
    ///   - 0b00: Normal operation
    ///   - 0b01: Shutdown processing in progress
    ///   - 0b10: Shutdown processing complete
    ///   - 0b11: Reserved
    /// - Bit 4: NSSRO for NVM Subsystem Reset Occurred
    /// - Bit 5: PP for Processing Paused
    /// - Bit 6: ST for Shutdown Type
    /// - Bit 7 ..= 31: Reserved
    CSTS = 0x1C,

    /// NVM Subsystem Reset
    /// - Bits 0 ..= 31: NSSRC for NVM Subsystem Reset Control
    NSSR = 0x20,

    /// Admin Queue Attributes
    /// - Bits 0 ..= 11: ASQS for Admin Submission Queue Size
    ///   - Valid range: 1 ..= 4095
    /// - Bits 12 ..= 15: Reserved
    /// - Bits 16 ..= 27: ACQS for Admin Completion Queue Size
    ///   - Valid range: 1 ..= 4095
    /// - Bits 28 ..= 31: Reserved
    AQA = 0x24,

    /// Admin Submission Queue
    /// - Bits 0 ..= 11: Reserved
    /// - Bits 12 ..= 63: ASQB for Admin Submission Queue Base
    ASQ = 0x28,

    /// Admin Completion Queue
    /// - Bits 0 ..= 11: Reserved
    /// - Bits 12 ..= 63: ACQB for Admin Completion Queue Base
    ACQ = 0x30,

    /// Controller Memory Buffer Location
    /// - Bits 0 ..= 2: BIR for Base Indicator Register
    /// - Bit 3: CQMMS for CMB Queue Mixed Memory Support
    /// - Bit 4: CQPDS for CMB Queue Physically Discontiguous Support
    /// - Bit 5: CDPMLS for CMB Data Pointer Mixed Locations Support
    /// - Bit 6: CDPCILS for CMB Data Pointer and Command Independent Locations Support
    /// - Bit 7: CDMMMS for CMB Data Metadata Mixed Memory Support
    /// - Bit 8: CQDA for CMB Queue Dword Alignment
    /// - Bits 9 ..= 11: Reserved
    /// - Bits 12 ..= 31: OFST for Offset
    CMBLOC = 0x38,

    /// Controller Memory Buffer Size
    /// - Bit 0: SQS for Submission Queue Support
    /// - Bit 1: CQS for Completion Queue Support
    /// - Bit 2: LISTS for PRP SGL List Support
    /// - Bit 3: RDS for Read Data Support
    /// - Bit 4: WDS for Write Data Support
    /// - Bits 5 ..= 7: Reserved
    /// - Bits 8 ..= 11: SZU for Size Units
    ///   - 0x0: 4 KiB
    ///   - 0x1: 64 KiB
    ///   - 0x2: 1 MiB
    ///   - 0x3: 16 MiB
    ///   - 0x4: 256 MiB
    ///   - 0x5: 4 GiB
    ///   - 0x6: 64 GiB
    ///   - 0x7 ..= 0xF: Reserved
    /// - Bits 12 ..= 31: SZ for Size
    CMBSZ = 0x3C,

    /// Boot Partition Information
    /// - Bits 0 ..= 14: BPSZ for Boot Partition Size
    /// - Bits 15 ..= 23: Reserved
    /// - Bits 24 ..= 25: BRS for Boot Read Status
    ///   - 0b00: No Boot Partition read operation requested
    ///   - 0b01: Boot Partition read in progress
    ///   - 0b10: Boot Partition read completed successfully
    ///   - 0b11: Error completing Boot Partition read
    /// - Bits 26 ..= 30: Reserved
    /// - Bit 31: ABPID for Active Boot Partition ID
    BPINFO = 0x40,

    /// Boot Partition Read Select
    /// - Bits 0 ..= 9: BPRSZ for Boot Partition Read Size
    /// - Bits 10 ..= 29: BPROF for Boot Partition Read Offset
    /// - Bit 30: Reserved
    /// - Bit 31: BPID for Boot Partition Identifer
    BPRSEL = 0x44,

    /// Boot Partition Memory Buffer Location
    /// - Bits 0 ..= 11: Reserved
    /// - Bits 12 ..= 63: BMBBA for Boot Partition Memory Buffer Base Address
    BPMBL = 0x48,

    /// Controller Memory Buffer Memory Space Control
    /// - Bit 0: CRE for Capabilities Registers Enabled
    /// - Bit 1: CMSE for Controller Memory Space Enable
    /// - Bits 2 ..= 11: Reserved
    /// - Bits 12 ..= 63: CBA for Controller Base Address
    CMBMSC = 0x50,

    /// Controller Memory Buffer Status
    /// - Bit 0: CBAI for Controller Base Address Invalid
    /// - Bits 1 ..= 31: Reserved
    CMBSTS = 0x58,

    /// Controller Memory Buffer Elasticity Buffer Size
    /// - Bits 0 ..= 3: CMBSZU for CMB Elasticity Buffer Size Units
    ///   - 0x0: Bytes
    ///   - 0x1: 1 KiB
    ///   - 0x2: 1 MiB
    ///   - 0x3: 1 GiB
    ///   - 0x4 ..= 0xF: Reserved
    /// - Bit 4: CMBRBB for CMB Read Bypass Behavior
    /// - Bits 5 ..= 7: Reserved
    /// - Bits 8 ..= 31: CMBWBZ for CMB Elasticity Buffer Size Base
    CMBEBS = 0x5C,

    /// Controller Memory Buffer Sustained Write Throughput
    /// - Bits 0 ..= 3: CMBSWTU for CMB Sustained Write Throughput Units
    ///   - 0x0: Bytes/s
    ///   - 0x1: 1 KiB/s
    ///   - 0x2: 1 MiB/s
    ///   - 0x3: 1 GiB/s
    ///   - 0x4 ..= 0xF: Reserved
    /// - Bits 4 ..= 7: Reserved
    /// - Bits 8 ..= 31: CMBSWTV for CMB Sustained Write Throughput
    CMBSWTP = 0x60,

    /// NVM Subsystem Shutdown
    /// - Bits 0 ..= 31: NSSC for NVM Subsystem Shutdown Control
    NSSD = 0x64,

    /// Controller Ready Timeouts
    /// - Bits 0 ..= 15: CRWMT for Controller Ready With Media Timeout
    /// - Bits 16 ..= 31: CRIMT for Controller Ready Independent of Media Timeout
    CRTO = 0x68,

    /// Persistent Memory Capabilities
    /// - Bits 0 ..= 2: Reserved
    /// - Bit 3: RDS for Read Data Support
    /// - Bit 4: WDS for Write Data Support
    /// - Bits 5 ..= 7: BIR for Base Indicator Register
    /// - Bits 8 ..= 9: PMRTU for Persistent Memory Region Time Units
    ///   - 0b00: 500 ms
    ///   - 0b01: mins
    ///   - 0b10 ..= 0b11: Reserved
    /// - Bits 10 ..= 13: PMRWBM for Persistent Memory Region Write Barrier Mechanisms
    ///   - Bit 0: CMR for Completion of Memory Read
    ///   - Bit 1: CPMRSTSR for Completion of PMRSTS Read
    ///   - Bits 2 ..= 3: Reserved
    /// - Bits 14 ..= 15: Reserved
    /// - Bits 16 ..= 23: PMRTO for Persistent Memory Region Timeout
    /// - Bit 24: CMSS for Controller Memory Space Supported
    /// - Bits 25 ..= 31: Reserved
    PMRCAP = 0xE00,

    /// Persistent Memory Region Control
    /// - Bit 0: EN for Enable
    /// - Bits 1 ..= 31: Reserved
    PMRCTL = 0xE04,

    /// Persistent Memory Region Status
    /// - Bits 0 ..= 7: ERR for Error
    /// - Bit 8: NRDY for Not Ready
    /// - Bits 9 ..= 11: HSTS for Health Status
    ///   - 0b000: Normal Operation
    ///   - 0b001: Restore Error
    ///   - 0b010: Read Only
    ///   - 0b011: Unreliable
    ///   - 0b100 ..= 0b111: Reserved
    /// - Bit 12: CBAI for Controller Base Address Invalid
    /// - Bits 13 ..= 31: Reserved
    PMRSTS = 0xE08,

    /// Persistent Memory Region Elasticity Buffer Size
    /// - Bits 0 ..= 3: PMRSZU for PMR Elasticity Buffer Size Units
    ///   - 0x0: Bytes
    ///   - 0x1: 1 KiB
    ///   - 0x2: 1 MiB
    ///   - 0x3: 1 GiB
    ///   - 0x4 ..= 0xF: Reserved
    /// - Bit 4: PMRRBB for PMR Read Bypass Behavior
    /// - Bits 5 ..= 7: Reserved
    /// - Bits 8 ..= 31: PMRWBZ for PMR Elasticity Buffer Size Base
    PMREBS = 0xE0C,

    /// Persistent Memory Region Sustained Write Throughput
    /// - Bits 0 ..= 3: PMRSWTU for PMR Sustained Write Throughput Units
    /// - Bits 4 ..= 7: Reserved
    /// - Bits 8 ..= 31: PMRSWTV for PMR Sustained Write Throughput
    PMRSWTP = 0xE10,

    /// Persistent Memory Region Controller Memory Space Control Lower
    /// - Bit 0: Reserved
    /// - Bit 1: CMSE for Controller Memory Space Enable
    /// - Bits 2 ..= 11: Reserved
    /// - Bits 12 ..= 31: CBA for Controller Base Address
    PMRMSCL = 0xE14,

    /// Persistent Memory Region Controller Memory Space Control Upper
    /// - Bits 0 ..= 31: CBA for Controller Base Address
    PMRMSCU = 0xE18,

    /// - Submission: Base + (2 * X) * (1 << (2 + CAP.DSTRD))
    /// - Completion: Base + (2 * X + 1) * (1 << (2 + CAP.DSTRD))
    DOORBELL = 0x1000,
}
impl Register {
    fn is_u64_register(&self) -> bool {
        matches!(
            self,
            Register::CAP | Register::ASQ | Register::ACQ | Register::BPMBL | Register::CMBMSC
        )
    }

    fn addr(self) -> u64 {
        unsafe { NVME.base + self as u64 }
    }

    fn sqdb(id: u16) -> u64 {
        Self::DOORBELL.addr() + (2 * id as u64) * (1 << (2 + unsafe { NVME.dstrd }))
    }

    fn cqdb(id: u16) -> u64 {
        Self::DOORBELL.addr() + (2 * id as u64 + 1) * (1 << (2 + unsafe { NVME.dstrd }))
    }

    fn read(self) -> u64 {
        if self.is_u64_register() {
            unsafe { read_volatile(self.addr() as *const u64) }
        } else {
            unsafe { read_volatile(self.addr() as *const u32) as u64 }
        }
    }

    fn write(self, value: u64) {
        if self.is_u64_register() {
            unsafe {
                write_volatile(self.addr() as *mut u64, value);
            }
        } else {
            unsafe {
                write_volatile(self.addr() as *mut u32, (value & 0xFFFFFFFF) as u32);
            }
        }
    }
}

pub fn init() -> Result<(), crate::Error> {
    unsafe { (*(&raw mut NVME)).init() }
}
