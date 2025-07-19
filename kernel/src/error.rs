//! Error

pub enum Error {
    // ACPI
    InvalidSignature,
    InvalidChecksum,
    InvalidRevision,
    InvalidLength,
    InvalidReserved,

    // I/O APIC
    MaxCountReached,
    InvalidGSIIndex,
}
