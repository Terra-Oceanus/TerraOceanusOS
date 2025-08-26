//! Trait

pub trait FromAddr: Sized {
    fn get_ref(addr: u64) -> &'static Self {
        unsafe { &*(addr as *const _) }
    }

    fn get_mut(addr: u64) -> &'static mut Self {
        unsafe { &mut *(addr as *mut _) }
    }
}
