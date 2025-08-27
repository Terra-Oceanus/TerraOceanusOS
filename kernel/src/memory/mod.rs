//! Memory

mod error;
pub mod physical;

pub use error::Error;

const PAGE_SIZE: usize = 0x1000;

pub fn init(entry: usize, descriptor_size: usize, descriptor_count: usize) -> Result<(), Error> {
    physical::init(entry, descriptor_size, descriptor_count)
}

pub trait Memory: Sized {
    fn get_ref(addr: usize) -> &'static Self {
        unsafe { &*(addr as *const _) }
    }

    fn get_mut(addr: usize) -> &'static mut Self {
        unsafe { &mut *(addr as *mut _) }
    }

    fn delete(&self) -> Result<(), Error> {
        physical::deallocate(self as *const _ as usize)
    }
}
