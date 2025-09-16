//! 82574 GbE Controller

use crate::drivers::pcie;

static mut DEVICE: Device = Device::null();

struct Device {
    addr: usize,
}
impl Device {
    /// Device Control
    /// - Bit 0: FD for Full Duplex
    ///   - 0: Half Duplex
    ///   - 1: Full Duplex
    /// - Bit 1: Reserved
    /// - Bit 2: GIO Master Disable
    /// - Bits 3 ..= 4: Reserved
    /// - Bit 5: ASDE for Auto-Speed Detection Enable
    /// - Bit 6: SLU for Set Link Up
    /// - Bit 7: Reserved
    /// - Bits 8 ..= 9: SPEED for Speed Selection
    ///   - 0b00: 10 Mb/s
    ///   - 0b01: 100 Mb/s
    ///   - 0b10: 1000 Mb/s
    ///   - 0b11: Reserved
    /// - Bit 10: Reserved
    /// - Bit 11: FRCSPD for Force Speed
    /// - Bit 12: FRCDPLX for Force Duplex
    /// - Bits 13 ..= 19: Reserved
    /// - Bit 20: ADVD3WUC for D3Cold WakeUp Capability Advertisement Enable
    /// - Bits 21 ..= 25: Reserved
    /// - Bit 26: RST for Device Reset
    /// - Bit 27: RFCE for Receive Flow Control Enable
    /// - Bit 28: TFCE for Transmit Flow Control Enable
    /// - Bit 29: Reserved
    /// - Bit 30: VME for VLAN Mode Enable
    /// - Bit 31: PHY_RST for PHY Reset
    ///   - 0: Normal
    ///   - 1: Asserted
    const CTRL: usize = 0x0;

    const fn null() -> Self {
        Self { addr: 0 }
    }

    fn read(&self, offset: usize) -> u32 {
        unsafe { ((self.addr + offset) as *const u32).read_volatile() }
    }

    fn write(&self, offset: usize, val: u32) {
        unsafe { ((self.addr + offset) as *mut u32).write_volatile(val) }
    }

    fn init(&mut self, pcie: &'static mut pcie::Type0) {
        pcie.header.set_memory_space(true);
        pcie.header.set_bus_master(true);

        self.addr = pcie.bar(0);
    }
}

pub fn init(pcie: &'static mut pcie::Type0) {
    unsafe { (*(&raw mut DEVICE)).init(pcie) }
}
