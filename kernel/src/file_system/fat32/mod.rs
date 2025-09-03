//! File Allocation Table 32

mod bs;

use bs::BS;

use crate::memory::Memory;

pub fn validate(addr: usize) -> bool {
    BS::get_ref(addr).validate()
}
