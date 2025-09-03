//! Error

pub enum Error {
    InvalidAddress(&'static str),
    InvalidRegisterValue(&'static str),
    Queue(&'static str),
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
    fn out(&self) {
        "/NVMe ".out();
        match self {
            Error::InvalidAddress(entity) => {
                entity.out();
                " Address"
            }
            Error::InvalidRegisterValue(reg) => {
                reg.out();
                " Value"
            }
            Error::Queue(msg) => {
                "Queue ".out();
                msg
            }
        }
        .out();
    }
}
