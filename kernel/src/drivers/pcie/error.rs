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
    fn output(self) {
        "PCIe ".output();
        match self {
            Error::InvalidHeaderType => "Invalid Header Type",
            Error::InvalidIndex(src) => {
                "Invalid ".output();
                src.output();
                " Index"
            }
            Error::InvalidRegisterValue(reg) => {
                "Invalid ".output();
                reg.output();
                " Value"
            }
        }
        .output();
    }
}
