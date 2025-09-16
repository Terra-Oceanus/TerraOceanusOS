//! Error

pub enum Error {
    Net(super::net::Error),
    PCIe(super::pcie::Error),
    Storage(super::storage::Error),
}
impl From<Error> for crate::Error {
    fn from(err: Error) -> Self {
        crate::Error::Drivers(err)
    }
}
impl crate::Output for Error {
    fn out(&self) {
        "Drivers".out();
        match self {
            Error::Net(e) => e.out(),
            Error::PCIe(e) => e.out(),
            Error::Storage(e) => e.out(),
        }
    }
}
