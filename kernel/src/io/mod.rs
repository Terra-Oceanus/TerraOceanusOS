//! IO

pub mod port;
pub mod text;

pub fn init(
    frame_buffer_base: usize,
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
}
