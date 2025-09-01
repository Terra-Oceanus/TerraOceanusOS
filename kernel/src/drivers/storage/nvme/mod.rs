//! Non-Volatile Memory Express

use core::hint::spin_loop;

use crate::{
    Math, Memory,
    drivers::pcie::{self, capabilities::MSIX},
    find_capabilities,
    memory::physical::allocate,
};

mod command;
mod error;
mod queue;

pub use error::Error;
use queue::Queue;

static mut NVME: NVMe = NVMe::null();

pub fn set_config(addr: usize) {
    unsafe { NVME.pcie_addr = addr };
}

struct NVMe {
    pcie_addr: usize,

    addr: usize,

    msi_x: MSIX,

    dstrd: u8,

    admin: Queue,
    io: Queue,

    ns: Namespace,
}
impl NVMe {
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
    const CAP: usize = 0x0;

    /// Version
    /// - Bits 0 ..= 7: TER for Tertiary Version
    /// - Bits 8 ..= 15: MNR for Minor Version
    /// - Bits 16 ..= 31: MJR for Major Version
    const VS: usize = 0x8;

    /// Interrupt Mask Set
    /// - Bits 0 ..= 31: IVMS for Interrupt Vector Mask Set
    const INTMS: usize = 0xC;

    /// Interrupt Mask Clear
    /// - Bits 0 ..= 31: IVMC for Interrupt Vector Mask Clear
    const INTMC: usize = 0x10;

    /// Controller Configuration
    /// - Bit 0: EN for Enable
    /// - Bits 1 ..= 3: Reserved
    /// - Bits 4 ..= 6: CSS for I/O Command Set Selected
    ///   - 0b000:
    ///     - NVM Command Set if CAP.CSS.NCSS is set
    ///     - Reserved if CAP.CSS.NCSS is cleared
    ///   - 0b001 ..= 0b101: Reserved
    ///   - 0b110:
    ///     - All Supported I/O Command Sets if CAP.CSS.IOCSS is set
    ///     - Reserved if CAP.CSS.IOCSS is cleared
    ///   - 0b111:
    ///     - Admin Command Set only if CAP.CSS.NOIOCSS is set
    ///     - Reserved if CAP.CSS.NOIOCSS is cleared
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
    const CC: usize = 0x14;

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
    const CSTS: usize = 0x1C;

    /// NVM Subsystem Reset
    /// - Bits 0 ..= 31: NSSRC for NVM Subsystem Reset Control
    const NSSR: usize = 0x20;

    /// Admin Queue Attributes
    /// - Bits 0 ..= 11: ASQS for Admin Submission Queue Size
    ///   - Valid range: 1 ..= 4095
    /// - Bits 12 ..= 15: Reserved
    /// - Bits 16 ..= 27: ACQS for Admin Completion Queue Size
    ///   - Valid range: 1 ..= 4095
    /// - Bits 28 ..= 31: Reserved
    const AQA: usize = 0x24;

    /// Admin Submission Queue
    /// - Bits 0 ..= 11: Reserved
    /// - Bits 12 ..= 63: ASQB for Admin Submission Queue Base
    const ASQ: usize = 0x28;

    /// Admin Completion Queue
    /// - Bits 0 ..= 11: Reserved
    /// - Bits 12 ..= 63: ACQB for Admin Completion Queue Base
    const ACQ: usize = 0x30;

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
    const CMBLOC: usize = 0x38;

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
    const CMBSZ: usize = 0x3C;

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
    const BPINFO: usize = 0x40;

    /// Boot Partition Read Select
    /// - Bits 0 ..= 9: BPRSZ for Boot Partition Read Size
    /// - Bits 10 ..= 29: BPROF for Boot Partition Read Offset
    /// - Bit 30: Reserved
    /// - Bit 31: BPID for Boot Partition Identifer
    const BPRSEL: usize = 0x44;

    /// Boot Partition Memory Buffer Location
    /// - Bits 0 ..= 11: Reserved
    /// - Bits 12 ..= 63: BMBBA for Boot Partition Memory Buffer Base Address
    const BPMBL: usize = 0x48;

    /// Controller Memory Buffer Memory Space Control
    /// - Bit 0: CRE for Capabilities Registers Enabled
    /// - Bit 1: CMSE for Controller Memory Space Enable
    /// - Bits 2 ..= 11: Reserved
    /// - Bits 12 ..= 63: CBA for Controller Base Address
    const CMBMSC: usize = 0x50;

    /// Controller Memory Buffer Status
    /// - Bit 0: CBAI for Controller Base Address Invalid
    /// - Bits 1 ..= 31: Reserved
    const CMBSTS: usize = 0x58;

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
    const CMBEBS: usize = 0x5C;

    /// Controller Memory Buffer Sustained Write Throughput
    /// - Bits 0 ..= 3: CMBSWTU for CMB Sustained Write Throughput Units
    ///   - 0x0: Bytes/s
    ///   - 0x1: 1 KiB/s
    ///   - 0x2: 1 MiB/s
    ///   - 0x3: 1 GiB/s
    ///   - 0x4 ..= 0xF: Reserved
    /// - Bits 4 ..= 7: Reserved
    /// - Bits 8 ..= 31: CMBSWTV for CMB Sustained Write Throughput
    const CMBSWTP: usize = 0x60;

    /// NVM Subsystem Shutdown
    /// - Bits 0 ..= 31: NSSC for NVM Subsystem Shutdown Control
    const NSSD: usize = 0x64;

    /// Controller Ready Timeouts
    /// - Bits 0 ..= 15: CRWMT for Controller Ready With Media Timeout
    /// - Bits 16 ..= 31: CRIMT for Controller Ready Independent of Media Timeout
    const CRTO: usize = 0x68;

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
    const PMRCAP: usize = 0xE00;

    /// Persistent Memory Region Control
    /// - Bit 0: EN for Enable
    /// - Bits 1 ..= 31: Reserved
    const PMRCTL: usize = 0xE04;

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
    const PMRSTS: usize = 0xE08;

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
    const PMREBS: usize = 0xE0C;

    /// Persistent Memory Region Sustained Write Throughput
    /// - Bits 0 ..= 3: PMRSWTU for PMR Sustained Write Throughput Units
    /// - Bits 4 ..= 7: Reserved
    /// - Bits 8 ..= 31: PMRSWTV for PMR Sustained Write Throughput
    const PMRSWTP: usize = 0xE10;

    /// Persistent Memory Region Controller Memory Space Control Lower
    /// - Bit 0: Reserved
    /// - Bit 1: CMSE for Controller Memory Space Enable
    /// - Bits 2 ..= 11: Reserved
    /// - Bits 12 ..= 31: CBA for Controller Base Address
    const PMRMSCL: usize = 0xE14;

    /// Persistent Memory Region Controller Memory Space Control Upper
    /// - Bits 0 ..= 31: CBA for Controller Base Address
    const PMRMSCU: usize = 0xE18;

    /// - Submission: Base + (2 * X) * (1 << (2 + CAP.DSTRD))
    /// - Completion: Base + (2 * X + 1) * (1 << (2 + CAP.DSTRD))
    const DOORBELL: usize = 0x1000;

    const fn null() -> Self {
        Self {
            pcie_addr: 0,
            addr: 0,
            msi_x: MSIX::null(),
            dstrd: 0,
            admin: Queue::null(),
            io: Queue::null(),
            ns: Namespace {
                id: 0,
                lba_count: 0,
                lba_size: 0,
            },
        }
    }

    fn is_u64_register(offset: usize) -> bool {
        matches!(
            offset,
            Self::CAP | Self::ASQ | Self::ACQ | Self::BPMBL | Self::CMBMSC
        )
    }

    fn read(&self, offset: usize) -> u64 {
        unsafe {
            if Self::is_u64_register(offset) {
                ((self.addr + offset) as *const u64).read_volatile()
            } else {
                ((self.addr + offset) as *const u32).read_volatile().into()
            }
        }
    }

    fn write(&self, offset: usize, val: u64) {
        unsafe {
            if Self::is_u64_register(offset) {
                ((self.addr + offset) as *mut u64).write_volatile(val)
            } else {
                ((self.addr + offset) as *mut u32).write_volatile(val as u32)
            }
        }
    }

    fn init(&mut self) -> Result<(), crate::Error> {
        if self.pcie_addr == 0 {
            return Err(Error::InvalidAddress("PCIe").into());
        }
        let pcie = pcie::Type0::get_mut(self.pcie_addr);
        pcie.header.set_memory_space(true);
        pcie.header.set_bus_master(true);
        pcie.header.set_interrupt(false);

        self.addr = pcie.bar(0);
        find_capabilities!(self.pcie_addr, pcie.p_capabilities(),
            MSIX::ID => &mut self.msi_x.addr,
        );

        self.write(Self::CC, 0);
        while (self.read(Self::CSTS) & 0b1) == 1 {
            spin_loop();
        }

        // MSI-X
        {
            if self.msi_x.addr == 0 {
                return Err(Error::InvalidAddress("MSI-X").into());
            }
            self.msi_x.disable();
            self.msi_x.set_tables(pcie.bar(self.msi_x.table_bir()?));
            self.msi_x
                .configure(0, crate::x86_64::idt::Interrupt::NVMe as u8)?;
            self.write(Self::INTMC, 0xFFFFFFFF);
            self.msi_x.enable();
        }

        let cap = self.read(Self::CAP);
        self.dstrd = ((cap >> 32) & 0xF) as u8;

        let admin_size = command::admin::ENTRY_COUNT;
        let (asq, acq) = self.admin.init(
            self.addr + Self::DOORBELL,
            self.dstrd,
            admin_size,
            admin_size,
        )?;
        self.write(
            Self::AQA,
            ((admin_size as u64 - 1) << 16) | (admin_size as u64 - 1),
        );
        self.write(Self::ASQ, asq as u64);
        self.write(Self::ACQ, acq as u64);
        self.write(Self::CC, {
            let cap = self.read(Self::CAP);
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
                | ((size_of::<command::Submission>().log2() & 0xF) << 16) as u64
                | ((size_of::<command::Completion>().log2() & 0xF) << 20) as u64
        });
        while (self.read(Self::CSTS) & 0b1) == 0 {
            spin_loop();
        }

        // Identify Namespace
        {
            use command::admin::identify;

            let list = identify::active_namespace_id_list::List::new()?;
            self.admin
                .next_submission()
                .to_active_namespace_id_list(list.addr());
            self.admin.doorbell_submission(1)?;
            self.admin.next_completion().to_active_namespace_id_list()?;
            self.admin.doorbell_completion();

            if list.0[1] != 0 {
                return Err(Error::InvalidRegisterValue("Namespace ID").into());
            }
            self.ns.id = list.0[0];
            let data = identify::namespace::Data::new()?;
            self.admin
                .next_submission()
                .to_identify_namespace_data_structure(data.addr(), self.ns.id);
            self.admin.doorbell_submission(1)?;
            self.admin
                .next_completion()
                .to_identify_namespace_data_structure()?;
            self.admin.doorbell_completion();

            (self.ns.lba_count, self.ns.lba_size) = data.handle()?;

            data.delete()?;
            list.delete()?;
        }

        // Create I/O Queue
        {
            let size = ((cap & 0xFFFF) + 1) as u16;
            let (iosq, iocq) = self
                .io
                .init(self.addr + Self::DOORBELL, self.dstrd, size, size)?;
            let id = self.io.id() as u32;

            self.admin.next_submission().to_create_io_completion_queue(
                iocq as u64,
                id,
                size as u32,
                0,
            );
            self.admin.next_submission().to_create_io_submission_queue(
                iosq as u64,
                id,
                size as u32,
            );
            self.admin.doorbell_submission(2)?;
            self.admin
                .next_completion()
                .to_create_io_completion_queue()?;
            self.admin
                .next_completion()
                .to_create_io_submission_queue()?;
            self.admin.doorbell_completion();
        }

        Ok(())
    }

    fn read_lba(&mut self, start: u64, count: u32) -> Result<usize, crate::Error> {
        let addr = allocate(self.ns.lba_size * count as usize)?;
        self.io
            .next_submission()
            .to_read(self.ns.id, addr as u64, start, count);
        self.io.doorbell_submission(1)?;
        self.io.next_completion().to_read()?;
        self.io.doorbell_completion();
        Ok(addr)
    }
}

struct Namespace {
    id: u32,

    lba_count: usize,
    lba_size: usize,
}

pub fn init() -> Result<(), crate::Error> {
    unsafe { (*(&raw mut NVME)).init() }
}

pub fn read(start: u64, count: u32) -> Result<usize, crate::Error> {
    unsafe { (*(&raw mut NVME)).read_lba(start, count) }
}
