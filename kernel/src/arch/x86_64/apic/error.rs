//! Error

pub enum Error {
    IOAPIC(super::ioapic::Error),
}
impl From<Error> for super::super::Error {
    fn from(err: Error) -> Self {
        super::super::Error::APIC(err)
    }
}
impl From<Error> for crate::Error {
    fn from(err: Error) -> Self {
        crate::Error::X86_64(err.into())
    }
}
impl crate::Output for Error {
    fn out(&self) {
        "APIC/".out();
        match self {
            Error::IOAPIC(e) => e.out(),
        }
    }
}
