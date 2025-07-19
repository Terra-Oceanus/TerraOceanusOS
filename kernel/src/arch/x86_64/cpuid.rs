//! CPUID

use core::arch::asm;

#[repr(u8)]
pub enum Leaf {
    /// - EAX: Maximum Input Value for Basic CPUID Information
    /// - EBX: Genu
    /// - ECX: ntel
    /// - EDX: ineI
    BasicCPUIDInformation0,

    /// - EAX: Version Information
    ///   - Type
    ///   - Family
    ///   - Model
    ///   - Stepping ID
    /// - EBX:
    ///   - Bits 0 ..= 7: Brand Index
    ///   - Bits 8 ..= 15: CLFLUSH instruction cache line size
    ///   - Bits 16 ..= 23: Maximum number of addressable IDs for logical processors
    ///   - Bits 24 ..= 31: Initial APIC ID
    /// - ECX:
    /// - EDX:
    BasicCPUIDInformation1,
}

pub fn cpuid(leaf: Leaf) -> (u32, u32, u32, u32) {
    let eax: u32;
    let ebx: u32;
    let ecx: u32;
    let edx: u32;
    unsafe {
        asm!(
            "cpuid",
            "mov {0:e}, ebx",
            lateout(reg) ebx,
            inlateout("eax") leaf as u32 => eax,
            lateout("ecx") ecx,
            lateout("edx") edx,
        )
    };
    (eax, ebx, ecx, edx)
}
