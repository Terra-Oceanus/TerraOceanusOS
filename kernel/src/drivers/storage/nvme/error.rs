//! Error

pub enum Error {
    InvalidAddress,
}
impl From<Error> for super::super::Error {
    fn from(err: Error) -> Self {
        super::super::Error::NVMe(err)
    }
}
impl From<Error> for super::super::super::Error {
    fn from(err: Error) -> Self {
        super::super::super::Error::Storage(err.into())
    }
}
impl From<Error> for crate::Error {
    fn from(err: Error) -> Self {
        crate::Error::Drivers(err.into())
    }
}
impl crate::Output for Error {
    fn output(&self) {
        "NVMe ".output();
        match self {
            Error::InvalidAddress => "Invalid Address",
        }
        .output();
    }
}
