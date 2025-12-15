//! Error

pub enum Error {
    InvalidAddress(&'static str),
}
impl From<Error> for super::super::Error {
    fn from(err: Error) -> Self {
        super::super::Error::Net(err)
    }
}
impl From<Error> for crate::Error {
    fn from(err: Error) -> Self {
        crate::Error::Drivers(err.into())
    }
}
impl crate::Output for Error {
    fn out(&self) {
        "/Net ".out();
        match self {
            Error::InvalidAddress(entity) => {
                entity.out();
                " Address"
            }
        }
        .out();
    }
}
