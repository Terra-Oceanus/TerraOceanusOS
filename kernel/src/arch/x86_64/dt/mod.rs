//! Descriptor Table

pub mod gdt;
pub mod idt;

#[repr(C, packed)]
struct Descriptor {
    size: u16,
    offset: u64,
}
impl Descriptor {
    fn new<T>(addr: usize) -> Self {
        Self {
            size: size_of::<T>() as u16 - 1,
            offset: addr as u64,
        }
    }
}

pub fn init() {
    gdt::init();
    idt::init();
}
