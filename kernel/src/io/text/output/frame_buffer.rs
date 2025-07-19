//! Frame Buffer

static mut BASE: u64 = 0;
static mut SIZE: usize = 0;

pub fn set_config(base: u64, size: usize) {
    unsafe {
        BASE = base;
        SIZE = size;
    }
}

pub fn base() -> u64 {
    unsafe { BASE }
}

pub fn size() -> usize {
    unsafe { SIZE }
}
