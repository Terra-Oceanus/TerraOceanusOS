//! Error

pub enum Error {
    ACPI(crate::acpi::Error),
    Drivers(crate::drivers::Error),
    Memory(crate::memory::Error),
    X86_64(crate::x86_64::Error),
}
impl crate::Output for Error {
    fn output(self) {
        "Error: ".output();
        match self {
            Error::ACPI(e) => e.output(),
            Error::Drivers(e) => e.output(),
            Error::Memory(e) => e.output(),
            Error::X86_64(e) => e.output(),
        }
        ".\n".output();
    }
}
