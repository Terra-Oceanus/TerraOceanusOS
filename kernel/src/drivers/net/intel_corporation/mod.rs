//! Intel Corporation

mod i82574;

pub fn init(pcie: &'static mut crate::drivers::pcie::Type0) {
    match pcie.header.device_id() {
        0x10D3 => i82574::init(pcie),
        _ => {}
    }
}
