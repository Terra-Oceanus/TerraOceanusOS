//! Error

pub enum Error {
    NVMe(super::nvme::Error),
    Partition(super::partition::Error),
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
    fn out(&self) {
        "/Storage".out();
        match self {
            Error::NVMe(e) => e.out(),
            Error::Partition(e) => e.out(),
        }
    }
}
