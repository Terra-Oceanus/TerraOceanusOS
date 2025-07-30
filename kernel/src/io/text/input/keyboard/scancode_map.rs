//! Scancode Map

use super::{super::super::super::port, Cursor};

pub fn map(byte: u8) {
    unsafe {
        match byte {
            0x02 => Cursor::out_char(if super::SHIFT { '!' } else { '1' }),
            0x03 => Cursor::out_char(if super::SHIFT { '@' } else { '2' }),
            0x04 => Cursor::out_char(if super::SHIFT { '#' } else { '3' }),
            0x05 => Cursor::out_char(if super::SHIFT { '$' } else { '4' }),
            0x06 => Cursor::out_char(if super::SHIFT { '%' } else { '5' }),
            0x07 => Cursor::out_char(if super::SHIFT { '^' } else { '6' }),
            0x08 => Cursor::out_char(if super::SHIFT { '&' } else { '7' }),
            0x09 => Cursor::out_char(if super::SHIFT { '*' } else { '8' }),
            0x0A => Cursor::out_char(if super::SHIFT { '(' } else { '9' }),
            0x0B => Cursor::out_char(if super::SHIFT { ')' } else { '0' }),
            0x0C => Cursor::out_char(if super::SHIFT { '_' } else { '-' }),
            0x0D => Cursor::out_char(if super::SHIFT { '+' } else { '=' }),
            0x0E => Cursor::backspace(),
            0x0F => Cursor::tab(),
            0x10 => Cursor::out_char(if super::CAPS_LOCK ^ super::SHIFT {
                'Q'
            } else {
                'q'
            }),
            0x11 => Cursor::out_char(if super::CAPS_LOCK ^ super::SHIFT {
                'W'
            } else {
                'w'
            }),
            0x12 => Cursor::out_char(if super::CAPS_LOCK ^ super::SHIFT {
                'E'
            } else {
                'e'
            }),
            0x13 => Cursor::out_char(if super::CAPS_LOCK ^ super::SHIFT {
                'R'
            } else {
                'r'
            }),
            0x14 => Cursor::out_char(if super::CAPS_LOCK ^ super::SHIFT {
                'T'
            } else {
                't'
            }),
            0x15 => Cursor::out_char(if super::CAPS_LOCK ^ super::SHIFT {
                'Y'
            } else {
                'y'
            }),
            0x16 => Cursor::out_char(if super::CAPS_LOCK ^ super::SHIFT {
                'U'
            } else {
                'u'
            }),
            0x17 => Cursor::out_char(if super::CAPS_LOCK ^ super::SHIFT {
                'I'
            } else {
                'i'
            }),
            0x18 => Cursor::out_char(if super::CAPS_LOCK ^ super::SHIFT {
                'O'
            } else {
                'o'
            }),
            0x19 => Cursor::out_char(if super::CAPS_LOCK ^ super::SHIFT {
                'P'
            } else {
                'p'
            }),
            0x1A => Cursor::out_char(if super::SHIFT { '{' } else { '[' }),
            0x1B => Cursor::out_char(if super::SHIFT { '}' } else { ']' }),
            0x1C => Cursor::enter(),
            0x1E => Cursor::out_char(if super::CAPS_LOCK ^ super::SHIFT {
                'A'
            } else {
                'a'
            }),
            0x1F => Cursor::out_char(if super::CAPS_LOCK ^ super::SHIFT {
                'S'
            } else {
                's'
            }),
            0x20 => Cursor::out_char(if super::CAPS_LOCK ^ super::SHIFT {
                'D'
            } else {
                'd'
            }),
            0x21 => Cursor::out_char(if super::CAPS_LOCK ^ super::SHIFT {
                'F'
            } else {
                'f'
            }),
            0x22 => Cursor::out_char(if super::CAPS_LOCK ^ super::SHIFT {
                'G'
            } else {
                'g'
            }),
            0x23 => Cursor::out_char(if super::CAPS_LOCK ^ super::SHIFT {
                'H'
            } else {
                'h'
            }),
            0x24 => Cursor::out_char(if super::CAPS_LOCK ^ super::SHIFT {
                'J'
            } else {
                'j'
            }),
            0x25 => Cursor::out_char(if super::CAPS_LOCK ^ super::SHIFT {
                'K'
            } else {
                'k'
            }),
            0x26 => Cursor::out_char(if super::CAPS_LOCK ^ super::SHIFT {
                'L'
            } else {
                'l'
            }),
            0x27 => Cursor::out_char(if super::SHIFT { ':' } else { ';' }),
            0x28 => Cursor::out_char(if super::SHIFT { '"' } else { '\'' }),
            0x29 => Cursor::out_char(if super::SHIFT { '~' } else { '`' }),
            0x2A => super::SHIFT = true,
            0x2B => Cursor::out_char(if super::SHIFT { '|' } else { '\\' }),
            0x2C => Cursor::out_char(if super::CAPS_LOCK ^ super::SHIFT {
                'Z'
            } else {
                'z'
            }),
            0x2D => Cursor::out_char(if super::CAPS_LOCK ^ super::SHIFT {
                'X'
            } else {
                'x'
            }),
            0x2E => Cursor::out_char(if super::CAPS_LOCK ^ super::SHIFT {
                'C'
            } else {
                'c'
            }),
            0x2F => Cursor::out_char(if super::CAPS_LOCK ^ super::SHIFT {
                'V'
            } else {
                'v'
            }),
            0x30 => Cursor::out_char(if super::CAPS_LOCK ^ super::SHIFT {
                'B'
            } else {
                'b'
            }),
            0x31 => Cursor::out_char(if super::CAPS_LOCK ^ super::SHIFT {
                'N'
            } else {
                'n'
            }),
            0x32 => Cursor::out_char(if super::CAPS_LOCK ^ super::SHIFT {
                'M'
            } else {
                'm'
            }),
            0x33 => Cursor::out_char(if super::SHIFT { '<' } else { ',' }),
            0x34 => Cursor::out_char(if super::SHIFT { '>' } else { '.' }),
            0x35 => Cursor::out_char(if super::SHIFT { '?' } else { '/' }),
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
