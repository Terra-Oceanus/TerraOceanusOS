//! 82574 GbE Controller

use core::hint::spin_loop;

use crate::drivers::pcie;

static mut DEVICE: Device = Device::null();

struct Device {
    addr: usize,
}
impl Device {
    /// Read/Write
    /// Device Control Register
    /// - Bit 0: FD for Full Duplex
    ///   - 0: Half Duplex
    ///   - 1: Full Duplex
    /// - Bit 1: Reserved
    /// - Bit 2: GIO Master Disable
    /// - Bits 3 ..= 4: Reserved
    /// - Bit 5: ASDE for Auto-Speed Detection Enable
    /// - Bit 6: SLU for Set Link Up
    /// - Bit 7: Reserved
    /// - Bits 8 ..= 9: SPEED
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

    /// Read/Write
    /// EEPROM Read Register
    /// - Bit 0: START
    /// - Bit 1: DONE
    /// - Bits 2 ..= 15: ADDR
    /// - Bits 16 ..= 31: DATA
    const EERD: usize = 0x14;

    /// Read/Write
    /// MDI Control Register
    /// - Bits 0 ..= 15: DATA
    /// - Bits 16 ..= 20: REGADD for PHY Register Address
    /// - Bits 21 ..= 25: PHYADD for PHY Address
    ///   - 1: Gigabit PHY
    ///   - 2: PCIe PHY
    /// - Bits 26 ..= 27: OP for Op-Code
    ///   - 0b00: Reserved
    ///   - 0b01: MDI Write
    ///   - 0b10: MDI Read
    ///   - 0b11: Reserved
    /// - Bit 28: R for Ready
    /// - Bit 29: I for Interrupt Enable
    /// - Bit 30: E for Error
    /// - Bit 31: Reserved
    const MDIC: usize = 0x20;

    /// Read/Write
    /// Interrupt Mask Set/Read Register
    /// - Bit 0: TXDW for Transmit Descriptor Written Back
    /// - Bit 1: TXQE for Transmit Queue Empty
    /// - Bit 2: LSC for Link Status Change
    /// - Bit 3: Reserved
    /// - Bit 4: RXDMT0 for Receive Descriptor Minimum Threshold Hit
    /// - Bit 5: Reserved
    /// - Bit 6: RXO for Receiver Overrun
    /// - Bit 7: RXT0 for Receiver Timer
    /// - Bit 8: Reserved
    /// - Bit 9: MDAC for MDIO Access Complete
    /// - Bits 10 ..= 14: Reserved
    /// - Bit 15: TXD_LOW for Transmit Descriptor Low Threshold Hit
    /// - Bit 16: SRPD for Small Receive Packet Detection
    /// - Bit 17: ACK for Receive ACK Frame Detection
    /// - Bit 18: MNG for Manageability Event
    /// - Bit 19: Reserved
    /// - Bit 20: RxQ0 for Receive Queue 0
    /// - Bit 21: Rxq1 for Receive Queue 1
    /// - Bit 22: TxQ0 for Transmit Queue 0
    /// - Bit 23: TxQ1 for Transmit Queue 1
    /// - Bit 24: Other
    /// - Bits 25 ..= 31: Reserved
    const IMS: usize = 0xD0;

    /// Write
    /// Interrupt Mask Clear Register
    /// - Bit 0: TXDW for Transmit Descriptor Written Back
    /// - Bit 1: TXQE for Transmit Queue Empty
    /// - Bit 2: LSC for Link Status Change
    /// - Bit 3: Reserved
    /// - Bit 4: RXDMT0 for Receive Descriptor Minimum Threshold Hit
    /// - Bit 5: Reserved
    /// - Bit 6: RXO for Receiver Overrun
    /// - Bit 7: RXT0 for Receiver Timer
    /// - Bit 8: Reserved
    /// - Bit 9: MDAC for MDIO Access Complete
    /// - Bits 10 ..= 14: Reserved
    /// - Bit 15: TXD_LOW for Transmit Descriptor Low Threshold Hit
    /// - Bit 16: SRPD for Small Receive Packet Detection
    /// - Bit 17: ACK for Receive ACK Frame Detection
    /// - Bit 18: MNG for Manageability Event
    /// - Bit 19: Reserved
    /// - Bit 20: RxQ0 for Receive Queue 0
    /// - Bit 21: Rxq1 for Receive Queue 1
    /// - Bit 22: TxQ0 for Transmit Queue 0
    /// - Bit 23: TxQ1 for Transmit Queue 1
    /// - Bit 24: Other
    /// - Bits 25 ..= 31: Reserved
    const IMC: usize = 0xD8;

    /// Read/Write
    /// Receive Address Low
    const RAL: usize = 0x5400;

    /// Read/Write
    /// Receive Address High
    /// - Bits 0 ..= 15: RAH
    /// - Bits 16 ..= 17: ASEL for Address Select
    ///   - 0b00: Destination
    ///   - 0b01: Source
    ///   - 0b10 ..= 0b11: Reserved
    /// - Bits 18 ..= 30: Reseerved
    /// - Bit 31: AV for Address Valid
    const RAH: usize = 0x5404;

    /// Read/Write
    /// 3GIO Control Register
    const GCR: usize = 0x5B00;

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

        self.write(Self::IMC, 0xFFFF_FFFF);
        self.write(Self::CTRL, (1 << 3) | (1 << 6) | (1 << 20) | (1 << 26));
        while (self.read(Self::CTRL) >> 26) & 1 != 0 {
            spin_loop();
        }
        self.write(Self::IMC, 0xFFFF_FFFF);
        self.write(Self::GCR, self.read(Self::GCR) | (1 << 22));
    }
}

pub fn init(pcie: &'static mut pcie::Type0) {
    unsafe { (*(&raw mut DEVICE)).init(pcie) }
}
