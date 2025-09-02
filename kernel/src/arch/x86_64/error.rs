//! Error

pub enum Error {
    APIC(super::apic::Error),
}
impl From<Error> for crate::Error {
    fn from(err: Error) -> Self {
        crate::Error::X86_64(err)
    }
}
impl crate::Output for Error {
    fn output(self) {
        "x86-64/".output();
        match self {
            Error::APIC(e) => e.output(),
        }
    }
}
