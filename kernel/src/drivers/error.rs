//! Error

pub enum Error {
    PCIe(super::pcie::Error),
    Storage(super::storage::Error),
}
impl From<Error> for crate::Error {
    fn from(err: Error) -> Self {
        crate::Error::Drivers(err)
    }
}
impl crate::Output for Error {
    fn output(self) {
        "Drivers/".output();
        match self {
            Error::PCIe(e) => e.output(),
            Error::Storage(e) => e.output(),
        }
    }
}
