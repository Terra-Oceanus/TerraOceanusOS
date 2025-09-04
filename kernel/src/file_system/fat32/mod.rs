//! File Allocation Table 32

use crate::memory::Memory;

mod bs;
mod directory;
mod fsinfo;

use bs::BS;
use fsinfo::FSI;

pub fn handle(start: u64, addr: usize) -> Result<bool, crate::Error> {
    let bs = BS::get_ref(addr);
    let fsi = match bs.validate() {
        0 => {
            bs.delete()?;
            return Ok(false);
        }
        sector => FSI::get_ref(crate::drivers::storage::read(start + sector as u64, 1, 0)?),
    };
    if !fsi.validate() {
        return Ok(false);
    }
    Ok(true)
}
