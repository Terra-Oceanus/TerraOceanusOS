//! Trait

pub trait FromAddr: Sized {
    fn get_ref(addr: u64) -> &'static Self {
        unsafe { &*(addr as *const Self) }
    }
}
