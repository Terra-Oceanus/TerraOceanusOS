//! Model Specific Registers

use core::arch::asm;

/// - Bits 0 ..= 7: Reserved
/// - Bit 8: BSP Flag
/// - Bit 9: Reserved
/// - Bit 10: Enable x2APIC mode
/// - Bit 11: APIC Global Enable
/// - Bits 12 ..= (MAXPHYADDR -1): APIC Base
/// - Bits MAXPHYADDR ..= 63: Reserved
pub const IA32_APIC_BASE: u32 = 0x1B;

/// - Bits 0 ..= 63: TSC-deadline Value
pub const IA32_TSC_DEADLINE: u32 = 0x6E0;

#[inline(always)]
pub fn read(msr: u32) -> u64 {
    let low: u32;
    let high: u32;
    unsafe {
        asm!(
            "rdmsr",
            in("ecx") msr,
            lateout("eax") low,
            lateout("edx") high,
        )
    };
    (high as u64) << 32 | low as u64
}

#[inline(always)]
pub fn write(msr: u32, value: u64) {
    let low = value as u32;
    let high = (value >> 32) as u32;
    unsafe {
        asm!(
            "wrmsr",
            in("ecx") msr,
            in("eax") low,
            in("edx") high,
        )
    };
}
