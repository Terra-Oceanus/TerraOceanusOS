//! Fixed ACPI Description Table

use crate::error::{ACPI, Error};

use super::{FromAddr, Header, dsdt, facs};

pub const SIGNATURE: &[u8; 4] = b"FACP";

static mut ADDR: u64 = 0;

pub fn set_config(addr: u64) {
    unsafe { ADDR = addr }
}

#[repr(C, packed)]
struct GenericAddressStructure {
    /// - 0x00 System Memory space
    /// - 0x01 System I/O space
    /// - 0x02 PCI Configuration space
    /// - 0x03 Embedded Controller
    /// - 0x04 SMBus
    /// - 0x05 SystemCMOS
    /// - 0x06 PciBarTarget
    /// - 0x07 IPMI
    /// - 0x08 General PurposeIO
    /// - 0x09 GenericSerialBus
    /// - 0x0A Platform Communications Channel (PCC)
    /// - 0x0B Platform Runtime Mechanism (PRM)
    /// - 0x0C ..= 0x7E Reserved
    /// - 0x7F Functional Fixed Hardware
    /// - 0x80 ..= 0xFF OEM Defined
    address_space_id: u8,

    /// - Reserved for data structure
    register_bit_width: u8,

    /// - Reserved for data structure
    register_bit_offset: u8,

    /// Access size | defined by Address Space ID
    /// - 0 Undefined (legacy reasons)
    /// - 1 Byte access
    /// - 2 Word access
    /// - 3 DWord access
    /// - 4 QWord access
    access_size: u8,

    address: u64,
}

#[repr(C, packed)]
struct FADT {
    header: Header,

    /// FACS
    /// - Ignored if X_FIRMWARE_CTRL is non-zero
    /// - Reserved if HARDWARE_REDUCED_ACPI flag is 1
    firmware_ctrl: u32,

    /// - Ignored if X_DSDT is non-zero
    dsdt: u32,

    reserved0: u8,
    preferred_pm_profile: u8,

    /// SCI interrupt GSI number as shareable, level, active low
    sci_int: u16,

    /// SMI command port address
    /// - Reserved for no support
    smi_cmd: u32,

    /// Value written to SMI_CMD
    /// - Reserved for no support
    acpi_enable: u8,

    /// Value written to SMI_CMD
    /// - Reserved for no support
    acpi_disable: u8,

    /// Value written to SMI_CMD to enter S4BIOS state
    ///
    /// No support if S4BIOS_F in FACS is zero
    s4bios_req: u8,

    /// Value if non-zero written to SMI_CMD to control processor performance state
    pstate_cnt: u8,

    /// PM1a Event Register Block port address required
    /// - Ignored if X_PM1a_CNT_BLK is non-zero
    pm1a_evt_blk: u32,

    /// PM1b Event Register Block port address optional
    /// - Ignored if X_PM1b_CNT_BLK is non-zero
    /// - Reserved for no support
    pm1b_evt_blk: u32,

    /// PM1a Control Register Block port address required
    /// - Ignored if X_PM1a_CNT_BLK is non-zero
    pm1a_cnt_blk: u32,

    /// PM1b Control Register Block port address optional
    /// - Ignored if X_PM1b_CNT_BLK is non-zero
    /// - Reserved for no support
    pm1b_cnt_blk: u32,

    /// PM2 Control Register Block port address optional
    /// - Ignored if X_PM2_CNT_BLK is non-zero
    /// - Reserved for no support
    pm2_cnt_blk: u32,

    /// Power Management Timer Control Register Block port address optional
    /// - Ignored if X_PM_TMR_BLK is non-zero
    /// - Reserved for no support
    pm_tmr_blk: u32,

    /// General-Purpose Event 0 Register Block port address
    /// - Ignored if X_GPE0_BLK is non-zero
    /// - Reserved for no support
    gpe0_blk: u32,

    /// General-Purpose Event 1 Register Block port address optional
    /// - Ignored if X_GPE1_BLK is non-zero
    /// - Reserved for no support
    gpe1_blk: u32,

    /// Number >= 4 of bytes decoded by PM1a_EVT_BLK & PM1b_EVT_BLK if supported
    pm1_evt_len: u8,

    /// Number >= 2 of bytes decoded by PM1a_CNT_BLK & PM1b_CNT_BLK if supported
    pm1_cnt_len: u8,

    /// Number >= 1 of bytes decoded by PM2_CNT_BLK
    /// - Reserved for no support
    pm2_cnt_len: u8,

    /// Number == 4 of bytes decoded by PM_TMR_BLK
    /// - Reserved for no support
    pm_tmr_len: u8,

    /// Length == +n * 2 of register given by X_GPE0_BLK if non-zero | GPE0_BLK
    gpe0_blk_len: u8,

    /// Length == +n * 2 of register given by X_GPE1_BLK if non-zero | GPE1_BLK
    gpe1_blk_len: u8,

    /// Offset of GPE1 events
    gpe1_base: u8,

    /// Value written to SMI_CMD to support _CST object & C States Changed notification
    /// - Reserved for no support
    cst_cnt: u8,

    /// Worst-case hardware latency in microseconds to enter/exit a C2 state
    /// - \> 100 for no support
    p_lvl2_lat: u16,

    /// Worst-case hardware latency in microseconds to enter/exit a C3 state
    /// - \> 1000 for no support
    p_lvl3_lat: u16,

    /// Deprecated
    flush_size: u16,

    /// Deprecated
    flush_stride: u16,

    /// Bit offset of duty cycle setting in P_CNT
    duty_offset: u8,

    /// Bit width of duty cycle setting in P_CNT
    /// - Reserved for no support
    duty_width: u8,

    /// Index of day of month alarm in RTC CMOS RAM
    /// - Required if MON_ALRM is supported
    /// - Reserved for no support
    day_alrm: u8,

    /// Index of month of year alarm in RTC CMOS RAM
    /// - Reserved for no support
    mon_alrm: u8,

    /// Index of century of data in RTC CMOS RAM
    /// - Reserved for no support
    century: u8,

    /// IA-PC Boot Architecture Flags
    iapc_boot_arch: u16,

    reserved1: u8,

    /// Fixed feature flags
    /// - Bit 0: WBINVD == 1
    /// - Bit 1: WBINVD_FLUSH
    ///   - Support system sleping states
    /// - Bit 2: PROC_C1
    ///   - 1: C1 power state supportd
    /// - Bit 3: P_LVL2_UP
    ///   - 0: C2 power state on uniprocessor system
    ///   - 1: C2 power state on multiprocessor system
    /// - Bit 4: PWR_BUTTON
    ///   - 0: Power button as fixed feature programming model
    ///   - 1: Power button as control method device | No power button
    /// - Bit 5: SLP_BUTTON
    ///   - 0: Sleep button as fixed feature programming model
    ///   - 1: Sleep button as control method device | No power button
    /// - Bit 6: FIX_RTC
    ///   - 0: Support RTC wake status in fixed register space
    ///   - 1: No support RTC wake status in fixed register space
    /// - Bit 7: RTC_S4
    ///   - Whether RTC alarm function can wake system from S4 state
    /// - Bit 8: TMR_VAL_EXT
    ///   - 0: TMR_VAL as 24-bit
    ///   - 1: TMR_VAL as 32-bit
    /// - Bit 9: DCK_CAP
    ///   - 0: No support for docking
    ///   - 1: Support for docking
    /// - Bit 10: RESET_REG_SUP
    ///   - 1: Support for reset via FADT RESET_REG
    /// - Bit 11: SEALED_CASE
    ///   - System Type Attribute
    ///   - 1: No internal expansion capabilities & sealed case
    /// - Bit 12: HEADLESS
    ///   - System Type Attribute
    ///   - 1: No monitor | keyboard/mouse
    /// - Bit 13: CPU_SW_SLP
    ///   - 1: Instruction required after writing SLP_TYPx register
    /// - Bit 14: PCI_EXP_WAK
    ///   - 1: Support PCIEXP_WAKE_STS bit in PM1 Status register & PCIEXP_WAKE_EN bit in PM1 Enable register
    /// - Bit 15: USE_PLATFORM_CLOCK
    /// - Bit 16: S4_RTC_STS_VALID
    /// - Bit 17: REMOTE_POWER_ON_CAPABLE
    /// - Bit 18: FORCE_APIC_CLUSTER_MODEL
    /// - Bit 19: FORCE_APIC_PHYSICAL_DESTINATION_MODE
    /// - Bit 20: HW_REDUCED_ACPI
    /// - Bit 21: LOW_POWER_S0_IDLE_CAPABLE
    /// - Bits 22 ..= 23: PERSISTENT_CPU_CACHES
    /// - Bits 24 ..= 31: Reserved
    flags: u32,

    /// Address_Space_ID valid for
    /// - System I/O space
    /// - System Memory space
    /// - PCI Configuration space (bus #0)
    ///
    /// Register_Bit_Width == 8
    ///
    /// Register_Bit_Offset == 0
    reset_reg: GenericAddressStructure,

    /// Value written to RESET_REG to reset system
    reset_value: u8,

    /// ARM Boot Architecture Flags
    arm_boot_arch: u16,

    /// Minor in Major.Minor form
    /// - Bits 0 ..= 3: Minor version
    /// - Bits 4 ..= 7: Errata version
    fadt_minor_version: u8,

    /// FACS
    /// - Reserved if HARDWARE_REDUCED_ACPI flag is 1
    x_firmware_ctrl: u64,

    x_dsdt: u64,

    /// PM1a Event Register Block port address required
    x_pm1a_evt_blk: GenericAddressStructure,

    /// PM1b Event Register Block port address optional
    /// - Reserved for no support
    x_pm1b_evt_blk: GenericAddressStructure,

    /// PM1a Control Register Block port address required
    x_pm1a_cnt_blk: GenericAddressStructure,

    /// PM1b Control Register Block port address optional
    /// - Reserved for no support
    x_pm1b_cnt_blk: GenericAddressStructure,

    /// PM2 Control Register Block port address optional
    /// - Reserved for no support
    x_pm2_cnt_blk: GenericAddressStructure,

    /// Power Management Timer Control Register Block port address optional
    /// - Reserved for no support
    x_pm_tmr_blk: GenericAddressStructure,

    /// General-Purpose Event 0 Register Block port address optional
    /// - Reserved for no support
    x_gpe0_blk: GenericAddressStructure,

    /// General-Purpose Event 1 Register Block port address optional
    /// - Reserved for no support
    x_gpe1_blk: GenericAddressStructure,

    /// Address_Space_ID valid for
    /// - System I/O space
    /// - System Memory space
    /// - PCI Configuration space (bus #0)
    ///
    /// Register_Bit_Width == 8
    ///
    /// Register_Bit_Offset == 0
    sleep_control_reg: GenericAddressStructure,

    /// Address_Space_ID valid for
    /// - System I/O space
    /// - System Memory space
    /// - PCI Configuration space (bus #0)
    ///
    /// Register_Bit_Width == 8
    ///
    /// Register_Bit_Offset == 0
    sleep_status_reg: GenericAddressStructure,

    hypervisor_vendor_identity: u64,
}
impl FADT {
    fn init(&self) -> Result<(), Error> {
        self.header.init(*SIGNATURE)?;
        facs::set_config(match self.x_firmware_ctrl {
            0 => self.firmware_ctrl as u64,
            addr => addr,
        });
        dsdt::set_config(match self.x_dsdt {
            0 => self.dsdt as u64,
            addr => addr,
        });
        Ok(())
    }

    fn preferred_pm_profile_to_str(&self) -> &'static str {
        match self.preferred_pm_profile {
            0 => "Unspecified",
            1 => "Desktop",
            2 => "Mobile",
            3 => "Workstation",
            4 => "Enterprise Server",
            5 => "SOHO Server",
            6 => "Appliance PC",
            7 => "Performance Server",
            8 => "Tablet",
            _ => "Reserved",
        }
    }
}

pub fn init() -> Result<(), Error> {
    unsafe {
        if ADDR == 0 {
            return Err(Error::ACPI(ACPI::InvalidAddress));
        }
        FADT::get_ref(ADDR).init()
    }
}
