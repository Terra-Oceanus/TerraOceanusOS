//! Error

pub enum Error {
    NVMe(super::nvme::Error),
}
impl From<Error> for super::super::Error {
    fn from(err: Error) -> Self {
        super::super::Error::Storage(err)
    }
}
impl From<Error> for crate::Error {
    fn from(err: Error) -> Self {
        crate::Error::Drivers(err.into())
    }
}
impl crate::Output for Error {
    fn output(&self) {
        "Storage/".output();
        match self {
            Error::NVMe(e) => e.output(),
        }
    }
}
