//! IO

use crate::init_end;

pub mod port;
pub mod text;

use text::Output;

pub fn init(
    frame_buffer_base: u64,
    screen_width: usize,
    screen_height: usize,
    screen_stride: usize,
) {
    text::init(
        frame_buffer_base,
        screen_width,
        screen_height,
        screen_stride,
    );
    init_end!();
}
