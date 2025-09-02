//! Frame Buffer

static mut BASE: usize = 0;
static mut SIZE: usize = 0;

pub fn set_config(base: usize, size: usize) {
    unsafe {
        BASE = base;
        SIZE = size;
    }
}

pub fn base() -> usize {
    unsafe { BASE }
}

pub fn size() -> usize {
    unsafe { SIZE }
}
