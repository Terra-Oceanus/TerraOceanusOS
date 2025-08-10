//! Error

pub enum Error {
    InvalidAddress,
    InvalidChecksum,
    InvalidLength,
    InvalidReserved,
    InvalidRevision,
    InvalidSignature,
}
impl From<Error> for crate::Error {
    fn from(err: Error) -> Self {
        crate::Error::ACPI(err)
    }
}
impl crate::Output for Error {
    fn output(&self) {
        "ACPI ".output();
        match self {
            Error::InvalidAddress => "Invalid Address",
            Error::InvalidChecksum => "Invalid Checksum",
            Error::InvalidLength => "Invalid Length",
            Error::InvalidReserved => "Invalid Reserved",
            Error::InvalidRevision => "Invalid Revision",
            Error::InvalidSignature => "Invalid Signature",
        }
        .output();
    }
}
