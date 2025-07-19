//! Keyboard

use super::super::{Cursor, Output};

mod scancode_map;

pub fn input(byte: u8) {
    scancode_map::map(byte).output();
}
