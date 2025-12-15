//! Error

pub enum Error {
    InvalidHeaderType,
    InvalidIndex(&'static str),
    InvalidRegisterValue(&'static str),
}
impl From<Error> for super::super::Error {
    fn from(err: Error) -> Self {
        super::super::Error::PCIe(err)
    }
}
impl From<Error> for crate::Error {
    fn from(err: Error) -> Self {
        crate::Error::Drivers(err.into())
    }
}
impl crate::Output for Error {
    fn out(&self) {
        "/PCIe ".out();
        match self {
            Error::InvalidHeaderType => "Header Type",
            Error::InvalidIndex(src) => {
                src.out();
                " Index"
            }
            Error::InvalidRegisterValue(reg) => {
                reg.out();
                " Value"
            }
        }
        .out();
    }
}
