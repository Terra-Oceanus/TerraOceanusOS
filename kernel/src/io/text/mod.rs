//! Text

mod cursor;
mod input;
mod output;

pub use cursor::Cursor;
pub use input::keyboard;
pub use output::{Output, screen};

pub fn init(
    frame_buffer_base: usize,
    screen_width: usize,
    screen_height: usize,
    screen_stride: usize,
) {
    output::init(
        frame_buffer_base,
        screen_width,
        screen_height,
        screen_stride,
    );
    Cursor::init();
}
