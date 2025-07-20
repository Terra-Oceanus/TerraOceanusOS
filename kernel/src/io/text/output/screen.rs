//! Screen

use super::{super::cursor::Color, Cursor, font, frame_buffer};

pub const MARGIN: usize = 8;

static mut WIDTH: usize = 0;
static mut HEIGHT: usize = 0;
static mut STRIDE: usize = 0;

pub fn set_config(width: usize, height: usize, stride: usize) {
    unsafe {
        WIDTH = width;
        HEIGHT = height;
        STRIDE = stride;
    }
}

pub fn width() -> usize {
    unsafe { WIDTH }
}

pub fn height() -> usize {
    unsafe { HEIGHT }
}

pub fn stride() -> usize {
    unsafe { STRIDE }
}

pub fn clear() {
    let mut ptr = frame_buffer::base() as *mut u8;
    let size = frame_buffer::size();
    for _ in 0..size {
        unsafe {
            ptr.write_volatile(0x00);
            ptr = ptr.add(1);
        }
    }
}

pub fn up() {
    unsafe {
        let mut ptr = (frame_buffer::base() as *mut u32)
            .add(MARGIN * STRIDE)
            .add(MARGIN);
        for _ in MARGIN..(HEIGHT - MARGIN) {
            for _ in MARGIN..(WIDTH - MARGIN) {
                ptr.write_volatile(ptr.add((font::HEIGHT * STRIDE) as usize).read_volatile());
                ptr = ptr.add(1);
            }
            ptr = ptr.add(MARGIN).add(STRIDE - WIDTH).add(MARGIN);
        }
    }
}

pub fn left(x: usize, y: usize, mut ptr: *mut u32) {
    for i in y..Cursor::max_y() {
        for j in 0..font::HEIGHT {
            unsafe {
                for _ in (if i == y { x * font::WIDTH } else { 0 })
                    ..((Cursor::max_x() - 1) * font::WIDTH)
                {
                    ptr.write_volatile(ptr.add(font::WIDTH).read_volatile());
                    ptr = ptr.add(1);
                }
                for _ in 0..font::WIDTH {
                    ptr.write_volatile(if i + 1 == Cursor::max_y() {
                        Color::Black as u32
                    } else {
                        ptr.sub((Cursor::max_x() - 1) * font::WIDTH)
                            .add(font::HEIGHT * STRIDE)
                            .read_volatile()
                    });
                    ptr = ptr.add(1);
                }
                ptr = ptr.add(MARGIN).add(STRIDE - WIDTH).add(MARGIN);
                if i == y && j + 1 != font::HEIGHT {
                    ptr = ptr.add(x * font::WIDTH);
                }
            }
        }
    }
}
