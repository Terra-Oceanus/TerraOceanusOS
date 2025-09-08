//! Scancode Map

use super::{super::super::super::port, Cursor};

#[inline(always)]
fn upper() -> bool {
    unsafe { super::CAPS_LOCK ^ super::SHIFT }
}

pub fn map(byte: u8) {
    unsafe {
        match byte {
            0x02 => Cursor::out_char(if super::SHIFT { '!' } else { '1' }, true),
            0x03 => Cursor::out_char(if super::SHIFT { '@' } else { '2' }, true),
            0x04 => Cursor::out_char(if super::SHIFT { '#' } else { '3' }, true),
            0x05 => Cursor::out_char(if super::SHIFT { '$' } else { '4' }, true),
            0x06 => Cursor::out_char(if super::SHIFT { '%' } else { '5' }, true),
            0x07 => Cursor::out_char(if super::SHIFT { '^' } else { '6' }, true),
            0x08 => Cursor::out_char(if super::SHIFT { '&' } else { '7' }, true),
            0x09 => Cursor::out_char(if super::SHIFT { '*' } else { '8' }, true),
            0x0A => Cursor::out_char(if super::SHIFT { '(' } else { '9' }, true),
            0x0B => Cursor::out_char(if super::SHIFT { ')' } else { '0' }, true),
            0x0C => Cursor::out_char(if super::SHIFT { '_' } else { '-' }, true),
            0x0D => Cursor::out_char(if super::SHIFT { '+' } else { '=' }, true),
            0x0E => Cursor::backspace(),
            0x0F => {
                for _ in 0..4 {
                    Cursor::space();
                }
            }
            0x10 => Cursor::out_char(if upper() { 'Q' } else { 'q' }, true),
            0x11 => Cursor::out_char(if upper() { 'W' } else { 'w' }, true),
            0x12 => Cursor::out_char(if upper() { 'E' } else { 'e' }, true),
            0x13 => Cursor::out_char(if upper() { 'R' } else { 'r' }, true),
            0x14 => Cursor::out_char(if upper() { 'T' } else { 't' }, true),
            0x15 => Cursor::out_char(if upper() { 'Y' } else { 'y' }, true),
            0x16 => Cursor::out_char(if upper() { 'U' } else { 'u' }, true),
            0x17 => Cursor::out_char(if upper() { 'I' } else { 'i' }, true),
            0x18 => Cursor::out_char(if upper() { 'O' } else { 'o' }, true),
            0x19 => Cursor::out_char(if upper() { 'P' } else { 'p' }, true),
            0x1A => Cursor::out_char(if super::SHIFT { '{' } else { '[' }, true),
            0x1B => Cursor::out_char(if super::SHIFT { '}' } else { ']' }, true),
            0x1C => Cursor::enter(),
            0x1E => Cursor::out_char(if upper() { 'A' } else { 'a' }, true),
            0x1F => Cursor::out_char(if upper() { 'S' } else { 's' }, true),
            0x20 => Cursor::out_char(if upper() { 'D' } else { 'd' }, true),
            0x21 => Cursor::out_char(if upper() { 'F' } else { 'f' }, true),
            0x22 => Cursor::out_char(if upper() { 'G' } else { 'g' }, true),
            0x23 => Cursor::out_char(if upper() { 'H' } else { 'h' }, true),
            0x24 => Cursor::out_char(if upper() { 'J' } else { 'j' }, true),
            0x25 => Cursor::out_char(if upper() { 'K' } else { 'k' }, true),
            0x26 => Cursor::out_char(if upper() { 'L' } else { 'l' }, true),
            0x27 => Cursor::out_char(if super::SHIFT { ':' } else { ';' }, true),
            0x28 => Cursor::out_char(if super::SHIFT { '"' } else { '\'' }, true),
            0x29 => Cursor::out_char(if super::SHIFT { '~' } else { '`' }, true),
            0x2A => super::SHIFT = true,
            0x2B => Cursor::out_char(if super::SHIFT { '|' } else { '\\' }, true),
            0x2C => Cursor::out_char(if upper() { 'Z' } else { 'z' }, true),
            0x2D => Cursor::out_char(if upper() { 'X' } else { 'x' }, true),
            0x2E => Cursor::out_char(if upper() { 'C' } else { 'c' }, true),
            0x2F => Cursor::out_char(if upper() { 'V' } else { 'v' }, true),
            0x30 => Cursor::out_char(if upper() { 'B' } else { 'b' }, true),
            0x31 => Cursor::out_char(if upper() { 'N' } else { 'n' }, true),
            0x32 => Cursor::out_char(if upper() { 'M' } else { 'm' }, true),
            0x33 => Cursor::out_char(if super::SHIFT { '<' } else { ',' }, true),
            0x34 => Cursor::out_char(if super::SHIFT { '>' } else { '.' }, true),
            0x35 => Cursor::out_char(if super::SHIFT { '?' } else { '/' }, true),
            0x36 => super::SHIFT = true,
            0x39 => Cursor::space(),
            0x3A => super::CAPS_LOCK = !super::CAPS_LOCK,
            0xAA => super::SHIFT = false,
            0xB6 => super::SHIFT = false,
            0xE0 => match port::in_byte(port::PS2_DATA) {
                0x47 => Cursor::home(),
                0x48 => Cursor::up(),
                0x4B => Cursor::left(),
                0x4D => Cursor::right(),
                0x4F => Cursor::end(),
                0x50 => Cursor::down(),
                0x53 => Cursor::delete(),
                _ => {}
            },
            _ => {}
        }
    }
}
