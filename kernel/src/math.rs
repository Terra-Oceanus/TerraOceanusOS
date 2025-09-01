//! Math

pub trait Math {
    fn log2(self) -> Self;
}
impl Math for usize {
    fn log2(self) -> Self {
        ((usize::BITS - 1) - self.leading_zeros()) as usize
    }
}
