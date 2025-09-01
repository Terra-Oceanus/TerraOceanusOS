//! Math

pub trait Math {
    fn log2(self) -> Self;
}
impl Math for usize {
    fn log2(self) -> Self {
        ((usize::BITS - 1) - self.leading_zeros()) as usize
    }
}

pub trait Checksum {
    fn checksum(&self, size: usize) -> bool {
        unsafe {
            core::slice::from_raw_parts(self as *const Self as *const u8, size)
                .iter()
                .copied()
                .fold(0u8, u8::wrapping_add)
                == 0
        }
    }
}
