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

pub fn poll() -> &'static super::Completion {
    unsafe { (*(&raw mut QUEUE)).completion.dequeue() }
}

pub fn execute(cmd: &mut super::Submission) -> &'static super::Completion {
    submit(cmd);
    poll()
}
