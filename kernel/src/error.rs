//! Error

pub enum Error {
    ACPI(crate::acpi::Error),
    Drivers(crate::drivers::Error),
    FileSystem(crate::file_system::Error),
    Memory(crate::memory::Error),
    X86_64(crate::x86_64::Error),
}
impl crate::Output for Error {
    fn out(&self) {
        "Error: ".out();
        match self {
            Error::ACPI(e) => e.out(),
            Error::Drivers(e) => e.out(),
            Error::FileSystem(e) => e.out(),
            Error::Memory(e) => e.out(),
            Error::X86_64(e) => e.out(),
        }
        ".\n".out();
    }
}
