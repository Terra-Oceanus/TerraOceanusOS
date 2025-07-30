//! Keyboard

use super::super::Cursor;

mod scancode_map;

static mut CAPS_LOCK: bool = false;
static mut SHIFT: bool = false;

pub fn input(byte: u8) {
    Cursor::wrapper(|| scancode_map::map(byte));
}
