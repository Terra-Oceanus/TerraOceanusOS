//! File Allocation Table 32

use crate::{io::text::Output, memory::Memory};

mod bs;
mod directory;
mod fsinfo;

use bs::BS;
use directory::Entry;
use fsinfo::FSI;

pub fn handle(start: u64) -> Result<bool, crate::Error> {
    use crate::drivers::storage::read;

    let bs = BS::get_ref(read(start, 0, size_of::<BS>())?);
    let fsi = match bs.validate() {
        0 => return Ok(false),
        sector => FSI::get_ref(read(
            start,
            sector as usize * bs.sector_bytes(),
            size_of::<FSI>(),
        )?),
    };
    let entry_per_cluster = bs.cluster_bytes() / size_of::<Entry>();
    let root = match fsi.validate() {
        true => unsafe {
            core::slice::from_raw_parts(
                read(start, bs.root_offset(), bs.cluster_bytes())? as *const Entry,
                entry_per_cluster,
            )
        },
        false => return Ok(false),
    };
    "FAT32\n".out();
    for entry in root {
        let first_cluster = match entry.name()[0] {
            0x00 => break,
            0xE5 => continue,
            _ => entry.first_cluster(),
        };
    }
    Ok(true)
}
