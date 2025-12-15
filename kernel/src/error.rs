//! Error

pub enum Error {
    ACPI(crate::acpi::Error),
    Drivers(crate::drivers::Error),
    FS(crate::fs::Error),
    Mem(crate::mem::Error),
    X86_64(crate::x86_64::Error),
}
impl crate::Output for Error {
    fn out(&self) {
        "Error: ".out();
        match self {
            Error::ACPI(e) => e.out(),
            Error::Drivers(e) => e.out(),
            Error::FS(e) => e.out(),
            Error::Mem(e) => e.out(),
            Error::X86_64(e) => e.out(),
        }
        ".\n".out();
    }
}
