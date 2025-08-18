//! Queue

mod submission;

struct Queue {
    id: u16,

    addr: u64,
    size: u16,

    doorbell: *mut u32,
}
