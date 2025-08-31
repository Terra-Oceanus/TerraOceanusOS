//! Identify Controller Data Structure

use crate::Memory;

use super::super::super::super::Error;

impl super::super::super::Submission {
    /// - CNS: 0x00
    pub fn to_identify_namespace_data_structure(&mut self, addr: usize, id: u32) {
        self.to_identify(addr);
        self.nsid = id;
    }
}

#[repr(C, packed)]
pub struct Data {
    /// Namespace Size
    nsze: u64,

    /// Namespace Capacity
    ncap: u64,

    /// Namespace Utilization
    nuse: u64,

    /// Namespace Features
    /// - Bit 0: THINP for Thin Provisioning
    /// - Bit 1: NSABP for Namespace Supported Atomic Boundary & Power
    /// - Bit 2: DAE for Deallocated Error
    /// - Bit 3: UIDREUSE for UID Reuse
    /// - Bits 4 ..= 5: OPTPERF for Optional Write Performance
    /// - Bit 6: MAM for Multiple Atomicity Mode
    /// - Bit 7: OPTRPERF for Optional Read Performance
    nsfeat: u8,

    /// Number of LBA Formats
    nlbaf: u8,

    /// Formatted LBA Size
    /// - Bits 0 ..= 3: FIDXL for Format Index Lower
    /// - Bit 4: MTELBA for Metadata Transferred as Extended LBA
    /// - Bits 5 ..= 6: FIDXU for Format Index Upper
    ///   - Ignored if NLBAF + NULBAF <= 16
    /// - Bit 7: Reserved
    flbas: u8,

    /// Metadata Capabilities
    /// - Bit 0: MTELBAS for Metadata Transferred as Extended LBA Support
    /// - Bit 1: MTSBS for Metadata Transferred as Separate Buffer Support
    mc: u8,

    /// End-to-end Data Protection Capabilities
    /// - Bit 0: PIT1S for Protection Information Type 1 Supported
    /// - Bit 1: PIT2S for Protection Information Type 2 Supported
    /// - Bit 2: PIT3S for Protection Information Type 3 Supported
    /// - Bit 3: PIIFB for Protection Information In First Bytes
    /// - Bit 4: PIILB for Protection Information In Last Bytes
    /// - Bits 5 ..= 7: Reserved
    dpc: u8,

    /// End-to-end Data Protection Type Settings
    /// - Bits 0 ..= 2: PIT for Protection Information Type
    ///   - 0b000: Not enabled
    ///   - 0b001: Type 1 enabled
    ///   - 0b010: Type 2 enabled
    ///   - 0b011: Type 3 enabled
    ///   - 0b100 ..= 0b111: Reserved
    /// - Bit 3: PIP for Protection Information Position
    /// - Bits 4 ..= 7: Reserved
    dps: u8,

    /// Namespace Multi-path I/O and Namespace Sharing Capabilities
    nmic: u8,

    /// Reservation Capabilities
    rescap: u8,

    /// Format Progress Indicator
    fpi: u8,

    /// Deallocate Logical Block Features
    /// - Bits 0 ..= 2: DRB for Deallocation Read Behavior
    ///   - 0b000: Not reported
    ///   - 0b001: All bytes cleared to 0x0
    ///   - 0b010: All bytes set to 0xFF
    ///   - 0b011 ..= 0b111: Reserved
    /// - Bit 3: WZDS for Write Zeroes Deallocation Support
    /// - Bit 4: GDS for Guard Deallocation Status
    /// - Bits 5 ..= 7: Reserved
    dlfeat: u8,

    /// Namespace Atomic Write Unit Normal
    nawun: u16,

    /// Namespace Atomic Write Unit Power Fail
    nawupf: u16,

    /// Namespace Atomic Compare & Write Unit
    nacwu: u16,

    /// Namespace Atomic Boundary Size Normal
    nabsn: u16,

    /// Namespace Atomic Boundary Offset
    nabo: u16,

    /// Namespace Atomic Boundary Size Power Fail
    nabspf: u16,

    /// Namespace Optimal I/O Boundary
    noiob: u16,

    /// NVM Capacity
    nvmcap: u128,

    /// Namespace Preferred Write Granularity
    npwg: u16,

    /// Namespace Preferred Write Alignment
    npwa: u16,

    /// Namespace Preferred Deallocate Granularity
    npdg: u16,

    /// Namespace Preferred Deallocate Alignment
    npda: u16,

    /// Namespace Optimal Write Size
    nows: u16,

    /// Maximum Single Source Range Length
    mssrl: u16,

    /// Maximum Copy Length
    mcl: u32,

    /// Maximum Source Range Count
    msrc: u8,

    /// Key Per I/O Status
    /// - Bit 0: KPIOENS for Key Per I/O Enabled in Namespace
    /// - Bit 1: KPIOSNS for Key Per I/O Supported in Namespace
    kpios: u8,

    /// Number of Unique Attribute LBA Formats
    nulbaf: u8,

    reserved0: u8,

    /// Key Per I/O Data Access Alignment and Granularity
    kpiodaag: u32,

    reserved1: u32,

    /// ANA Group Identifier
    anagrpid: u32,

    reserved2: [u8; 3],

    /// Namespace Attributes
    nsattr: u8,

    /// NVM Set Identifier
    nvmsetid: u16,

    /// Endurance Group Identifier
    endgid: u16,

    /// Namespace Globally Unique Identifier
    nguid: u128,

    /// IEEE Extended Unique Identifier
    eui64: u64,

    /// Logical Block Address
    lba: [LBAFormatData; 64],

    /// Vendor Specific
    vs: [u8; 3712],
}
impl Memory for Data {}
impl Data {
    pub fn handle(&self) -> Result<(usize, usize), Error> {
        let flbas = self.flbas;
        let lba = &self.lba[((flbas & 0b1111)
            | if self.nlbaf + self.nulbaf <= 16 {
                0
            } else {
                (flbas & 0b1100000) >> 1
            }) as usize];
        if lba.ms() != 0 {
            return Err(Error::InvalidRegisterValue("LBAF.MS"));
        }
        Ok((self.ncap as usize, 1 << lba.lbads()))
    }
}

#[repr(C, packed)]
struct LBAFormatData {
    /// - Bits 0 ..= 15: MS for Metadata Size
    ///   - 0: Not supported
    /// - Bits 16 ..= 23: LBADS for LBA Data Size
    ///   - 0: Self not available
    ///   - 1 ..= 8: Not supported
    /// - Bits 24 ..= 25: RP for Relative Performance
    ///   - 0b00: Best
    ///   - 0b01: Better
    ///   - 0b10: Good
    ///   - 0b11: Degraded
    /// - Bits 26 ..= 31: Reserved
    dword0: u32,
}
impl LBAFormatData {
    fn ms(&self) -> u32 {
        self.dword0 & 0xFFFF
    }

    fn lbads(&self) -> usize {
        ((self.dword0 >> 16) & 0xFF) as usize
    }
}
