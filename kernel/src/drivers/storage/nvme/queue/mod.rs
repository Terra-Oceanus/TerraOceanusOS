//! Queue

pub mod completion;
pub mod submission;

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
        submission_size: u16,
        completion_size: u16,
    ) -> Result<(u64, u64), crate::Error> {
        unsafe {
            self.id = ID_COUNTER;
            ID_COUNTER += 1;
        };
        Ok((
            self.submission.init(self.id, submission_size)?,
            self.completion.init(self.id, completion_size)?,
        ))
    }

    pub fn submit(&mut self, cmd: &mut super::command::Submission) -> &mut Self {
        self.submission.enqueue(cmd);
        self
    }

    pub fn poll(&mut self) -> &'static super::command::Completion {
        self.completion.dequeue()
    }
}
