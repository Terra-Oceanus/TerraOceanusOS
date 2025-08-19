//! Admin

use super::super::queue::Queue;

mod identify;

pub const ENTRY_COUNT: u16 = 32;

static mut QUEUE: Queue = Queue::null();

pub fn init() -> Result<(u64, u64), crate::Error> {
    unsafe { (*(&raw mut QUEUE)).init(ENTRY_COUNT, ENTRY_COUNT) }
}

pub fn submit(cmd: &mut super::Submission) {
    unsafe { (*(&raw mut QUEUE)).submission.enqueue(cmd) };
}
