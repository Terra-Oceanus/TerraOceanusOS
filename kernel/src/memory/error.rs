//! Error

pub enum Error {
    InvalidAllocationSize,
    InvalidIndex,

    OutOfMemory,
}
impl From<Error> for crate::Error {
    fn from(err: Error) -> Self {
        crate::Error::Memory(err)
    }
}
impl crate::Output for Error {
    fn out(&self) {
        "Memory ".out();
        match self {
            Error::InvalidAllocationSize => "Invalid Allocation Size",
            Error::InvalidIndex => "Invalid Index",
            Error::OutOfMemory => "Out of Memory",
        }
        .out();
    }
}
