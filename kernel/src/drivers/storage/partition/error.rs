//! Error

pub enum Error {
    InvalidGPT(&'static str),
    InvalidMBR(&'static str),
}
impl From<Error> for super::super::Error {
    fn from(err: Error) -> Self {
        super::super::Error::Partition(err)
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
        "/Partition ".out();
        match self {
            Error::InvalidGPT(part) => {
                "GPT ".out();
                part
            }
            Error::InvalidMBR(part) => {
                "Protective MBR ".out();
                part
            }
        }
        .out();
    }
}
