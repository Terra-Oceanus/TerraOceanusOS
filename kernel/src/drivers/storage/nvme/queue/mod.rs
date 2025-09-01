//! Queue

mod completion;
mod submission;

use completion::Completion;
use submission::Submission;

static mut ID_COUNTER: u16 = 0;

pub struct Queue {
    id: u16,

    submission: Submission,
    completion: Completion,
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

    pub fn id(&self) -> u16 {
        self.id
    }

    pub fn next_submission(&mut self) -> &'static mut super::command::Submission {
        self.submission.next_cmd()
    }

    pub fn doorbell_submission(&mut self, n: u16) -> Result<(), super::Error> {
        self.submission.doorbell(n)
    }

    pub fn next_completion(&mut self) -> &'static mut super::command::Completion {
        self.completion.next_cmd()
    }

    pub fn doorbell_completion(&mut self) {
        self.completion.doorbell()
    }
}
