//! Error

pub enum Error {
    InvalidFileSystem(usize, usize),
}
impl From<Error> for crate::Error {
    fn from(err: Error) -> Self {
        crate::Error::FileSystem(err)
    }
}
impl crate::Output for Error {
    fn out(&self) {
        "FileSystem".out();
        match self {
            Error::InvalidFileSystem(start, end) => {
                " Type from LBA ".out();
                start.out();
                " to LBA ".out();
                end.out();
            }
        }
    }
}
