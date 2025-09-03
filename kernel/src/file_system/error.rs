//! Error

pub enum Error {
    InvalidFileSystem,
}
impl crate::Output for Error {
    fn out(&self) {
        "/FileSystem".out();
        match self {
            Error::InvalidFileSystem => " Type",
        }
        .out();
    }
}
