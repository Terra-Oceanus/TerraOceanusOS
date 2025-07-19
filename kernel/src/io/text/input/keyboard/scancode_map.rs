//! Scancode Map

use super::{super::super::super::port, Cursor};

pub fn map(byte: u8) -> &'static str {
    match byte {
        0x02 => "1",
        0x03 => "2",
        0x04 => "3",
        0x05 => "4",
        0x06 => "5",
        0x07 => "6",
        0x08 => "7",
        0x09 => "8",
        0x0A => "9",
        0x0B => "0",
        0x0C => "-",
        0x0D => "=",
        0x0E => {
            Cursor::backspace();
            ""
        }
        0x0F => {
            Cursor::tab();
            ""
        }
        0x10 => "Q",
        0x11 => "W",
        0x12 => "E",
        0x13 => "R",
        0x14 => "T",
        0x15 => "Y",
        0x16 => "U",
        0x17 => "I",
        0x18 => "O",
        0x19 => "P",
        0x1A => "[",
        0x1B => "]",
        0x1C => {
            Cursor::enter();
            ""
        }
        0x1E => "A",
        0x1F => "S",
        0x20 => "D",
        0x21 => "F",
        0x22 => "G",
        0x23 => "H",
        0x24 => "J",
        0x25 => "K",
        0x26 => "L",
        0x27 => ";",
        0x28 => "\"",
        0x29 => "`",
        0x2B => "\\",
        0x2C => "Z",
        0x2D => "X",
        0x2E => "C",
        0x2F => "V",
        0x30 => "B",
        0x31 => "N",
        0x32 => "M",
        0x33 => ",",
        0x34 => ".",
        0x35 => "/",
        0x39 => " ",
        0xE0 => {
            match port::in_byte(port::PS2_DATA) {
                0x47 => Cursor::home(),
                0x48 => Cursor::up(),
                0x4B => Cursor::left(),
                0x4D => Cursor::right(),
                0x4F => Cursor::end(),
                0x50 => Cursor::down(),
                0x53 => Cursor::delete(),
                _ => {}
            };
            ""
        }
        _ => "",
    }
}
