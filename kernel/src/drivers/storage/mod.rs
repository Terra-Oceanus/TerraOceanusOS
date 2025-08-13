//! Storage

pub mod nvme;

pub fn init() {
    nvme::init();
}