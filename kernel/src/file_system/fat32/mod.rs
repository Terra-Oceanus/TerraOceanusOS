//! File Allocation Table 32

use crate::memory::Memory;

mod bs;
mod directory;
mod fsinfo;

use bs::BS;
use fsinfo::FSI;

pub fn handle(start: u64) -> Result<bool, crate::Error> {
    use crate::drivers::storage::read;

    let bs = BS::get_ref(read(start, 0, size_of::<BS>())?);
    let fsi = match bs.validate() {
        0 => {
            bs.delete()?;
            return Ok(false);
        }
        sector => FSI::get_ref(read(
            start,
            sector as usize * bs.sector_bytes(),
            size_of::<FSI>(),
        )?),
    };
    if !fsi.validate() {
        return Ok(false);
    }
    Ok(true)
}
