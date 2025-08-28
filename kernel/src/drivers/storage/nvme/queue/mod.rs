//! Queue

mod completion;
mod submission;

use completion::Completion;
use submission::Submission;

static mut ID_COUNTER: u16 = 0;

pub struct Queue {
    id: u16,

    pub submission: Submission,
    pub completion: Completion,
}
impl Queue {
    pub const fn null() -> Self {
        Self {
            id: 0,
            submission: Submission::null(),
            completion: Completion::null(),
        }
    }

    pub fn init(
        &mut self,
        doorbell_base: usize,
        dstrd: u8,
        submission_size: u16,
        completion_size: u16,
    ) -> Result<(usize, usize), crate::Error> {
        unsafe {
            self.id = ID_COUNTER;
            ID_COUNTER += 1;
        };
        Ok((
            self.submission.init(
                submission_size,
                doorbell_base + (2 * self.id as usize) * (1 << (2 + dstrd)),
            )?,
            self.completion.init(
                completion_size,
                doorbell_base + (2 * self.id as usize + 1) * (1 << (2 + dstrd)),
            )?,
        ))
    }

    pub fn new_cmd(&self) -> &'static mut super::command::Submission {
        self.submission.tail_cmd()
    }

    pub fn execute(&mut self) {
        self.submission.enqueue();
        self.completion.dequeue();
    }
}
