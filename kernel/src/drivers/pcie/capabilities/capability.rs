//! Capability

pub const CAPABILITY_ID: u8 = 0x10;

#[repr(C, packed)]
struct Capability {
    header: super::Header,

    /// - Bits 0 ..= 3: Capability Version
    /// - Bits 4 ..= 7: Device/Port Type
    ///   - 0b0000: PCI Express Endpoint
    ///   - 0b0001: Legacy PCI Express Endpoint
    ///   - 0b0010 ..= 0b0011: Reserved
    ///   - 0b0100: Root Port of PCI Express Root Complex
    ///   - 0b0101: Upstream Port of PCI Express Switch
    ///   - 0b0110: Downstream Port of PCI Express Switch
    ///   - 0b0111: PCI Express to PCI/PCI-X Bridge
    ///   - 0b1000: PCI/PCI-X to PCI Express Bridge
    ///   - 0b1001: RCiEP
    ///   - 0b1010: Root Complex Event Collector
    ///   - 0b1011 ..= 0b1111: Reserved
    /// - Bit 8: Slot Implemented
    /// - Bits 9 ..= 13: Interrupt Message Number
    /// - Bit 14: Undefined
    /// - Bit 15: Flit Mode Supported
    capabilities: u16,

    /// - Bits 0 ..= 2: Max_Payload_Size Supported
    ///   - 0b000: 128 bytes max payload size
    ///   - 0b001: 256 bytes max payload size
    ///   - 0b010: 512 bytes max payload size
    ///   - 0b011: 1024 bytes max payload size
    ///   - 0b100: 2048 bytes max payload size
    ///   - 0b101: 4096 bytes max payload size
    ///   - 0b110 ..= 0b111: Reserved
    /// - Bits 3 ..= 4: Phantom Functions Supported
    ///   - 0b00: No Function Number bits
    ///   - 0b01: The most significant bit
    ///   - 0b10: The two most significant bits
    ///   - 0b11: All 3 bits
    /// - Bit 5: Extended Tag Field Supported
    ///   - 0: 5-bit Tag Requester capability supported
    ///   - 1: 8-bit Tag Requester capability supported
    /// - Bits 6 ..= 8: Endpoint L0s Acceptable Latency
    ///   - 0b000: Maximum of 64 ns
    ///   - 0b001: Maximum of 128 ns
    ///   - 0b010: Maximum of 256 ns
    ///   - 0b011: Maximum of 512 ns
    ///   - 0b100: Maximum of 1 μs
    ///   - 0b101: Maximum of 2 μs
    ///   - 0b110: Maximum of 4 μs
    ///   - 0b111: No limit
    /// - Bits 9 ..= 11: Endpoint L1 Acceptable Latency
    ///   - 0b000: Maximum of 1 μs
    ///   - 0b001: Maximum of 2 μs
    ///   - 0b010: Maximum of 4 μs
    ///   - 0b011: Maximum of 8 μs
    ///   - 0b100: Maximum of 16 μs
    ///   - 0b101: Maximum of 32 μs
    ///   - 0b110: Maximum of 64 μs
    ///   - 0b111: No limit
    /// - Bits 12 ..= 14: Undefined
    /// - Bit 15: Role-Based Error Reporting
    /// - Bit 16: ERR_COR Subclass Capable
    /// - Bit 17: Rx_MPS_Fixed
    /// - Bits 18 ..= 25: Captured Slot Power Limit Value
    /// - Bits 26 ..= 27: Captured Slot Power Limit Scale
    ///   - 0b00: 1.0x
    ///   - 0b01: 0.1x
    ///   - 0b10: 0.01x
    ///   - 0b11: 0.001x
    /// - Bit 28: Function Level Reset Capability
    /// - Bit 29: Mixed_MPS_Supported
    /// - Bit 30: TEE-IO Supported
    /// - Bit 31: Reserved
    device_capabilities: u32,

    device_control: u16,
    device_status: u16,

    link_capabilities: u32,
    link_control: u16,
    link_status: u16,

    slot_capabilities: u32,
    slot_control: u16,
    slot_status: u16,

    root_control: u16,
    root_capabilities: u16,
    root_status: u32,

    device_capabilities_2: u32,
    device_control_2: u16,

    /// Reserved
    device_status_2: u16,

    link_capabilities_2: u32,
    link_control_2: u16,
    link_status_2: u16,

    /// - Bit 0: In-Band PD Disable Supported
    /// - Bits 1 ..= 2: SCap2 OOB PD Supported
    ///   - 0b00: This field does not indicate if OOB PD is supported or not
    ///   - 0b01: Reserved
    ///   - 0b10: Not supported
    ///   - 0b11: Supported
    /// - Bits 3 ..= 31: Reserved
    slot_capabilities_2: u32,

    /// Reserved
    slot_control_2: u16,

    /// Reserved
    slot_status_2: u16,
}
