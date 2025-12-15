//! Error

pub enum Error {
    InvalidAddress([u8; 4]),
    InvalidChecksum([u8; 4]),
    InvalidLength([u8; 4]),
    InvalidRevision([u8; 4]),
    InvalidSignature([u8; 4]),
}
impl From<Error> for crate::Error {
    fn from(err: Error) -> Self {
        crate::Error::ACPI(err)
    }
}
impl crate::Output for Error {
    fn out(&self) {
        "ACPI ".out();
        match self {
            Error::InvalidAddress(signature) => {
                signature.out();
                " Address"
            }
            Error::InvalidChecksum(signature) => {
                signature.out();
                " Checksum"
            }
            Error::InvalidLength(signature) => {
                signature.out();
                " Length"
            }
            Error::InvalidRevision(signature) => {
                signature.out();
                " Revision"
            }
            Error::InvalidSignature(signature) => {
                signature.out();
                " Signature"
            }
        }
        .out();
    }
}
