//! Error

pub enum Error {
    InvalidGSIIndex,

    MaxCountReached,
}
impl From<Error> for super::super::Error {
    fn from(err: Error) -> Self {
        super::super::Error::IOAPIC(err)
    }
}
impl From<Error> for super::super::super::Error {
    fn from(err: Error) -> Self {
        super::super::super::Error::APIC(err.into())
    }
}
impl From<Error> for crate::Error {
    fn from(err: Error) -> Self {
        crate::Error::X86_64(err.into())
    }
}
impl crate::Output for Error {
    fn out(&self) {
        "IOAPIC ".out();
        match self {
            Error::InvalidGSIIndex => "Invalid GSI Index",
            Error::MaxCountReached => "Max Count Reached",
        }
        .out();
    }
}
