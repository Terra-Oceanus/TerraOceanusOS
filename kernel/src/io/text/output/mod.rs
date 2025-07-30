//! Output

use crate::Error;

use super::Cursor;

pub mod font;
pub mod frame_buffer;
pub mod screen;

pub trait Output {
    fn output(&self) {}
}
impl Output for usize {
    fn output(&self) {
        if *self == 0 {
            '0'.output();
        } else {
            // 1 << 64: 20-digit
            let mut buffer = [0u8; 20];
            let mut n = *self;
            let mut i = buffer.len();
            while n > 0 {
                i -= 1;
                buffer[i] = b'0' + (n % 10) as u8;
                n /= 10;
            }
            for &byte in &buffer[i..] {
                (byte as char).output();
            }
        }
    }
}
impl Output for u8 {
    fn output(&self) {
        "0b".output();

        let mut zero = true;
        for i in (0..8).rev() {
            let bit = ((self >> i) & 1) as u8;
            if zero {
                if bit == 0 && i != 0 {
                    continue;
                }
                zero = false;
            }
            if bit == 0 {
                '0'.output();
            } else {
                '1'.output();
            }
        }

        if zero {
            '0'.output();
        }
    }
}
impl Output for u64 {
    fn output(&self) {
        "0x".output();

        let mut zero = true;
        for i in (0..16).rev() {
            let nibble = ((self >> (i * 4)) & 0xF) as u8;
            if zero {
                if nibble == 0 && i != 0 {
                    continue;
                }
                zero = false;
            }
            if nibble < 10 {
                ((b'0' + nibble) as char).output();
            } else {
                ((b'A' + (nibble - 10)) as char).output();
            }
        }

        if zero {
            '0'.output();
        }
    }
}
impl Output for char {
    fn output(&self) {
        if self.is_ascii_control() {
            match self {
                '\t' => Cursor::tab(),
                '\n' => Cursor::enter(),
                _ => {}
            }
        } else {
            Cursor::out_char(*self, true);
        }
    }
}
impl Output for &str {
    fn output(&self) {
        for c in self.chars() {
            c.output();
        }
    }
}
impl Output for Error {
    fn output(&self) {
        match self {
            Error::InvalidSignature => "\nACPI Error: Invalid Signature\n",
            Error::InvalidChecksum => "\nACPI Error: Invalid Checksum\n",
            Error::InvalidRevision => "\nACPI Error: Invalid Revision\n",
            Error::InvalidLength => "\nACPI Error: Invalid Length\n",
            Error::InvalidReserved => "\nACPI Error: Invalid Reserved\n",
            Error::MaxCountReached => "\nI/O APIC Error: Max Count Reached\n",
            Error::InvalidGSIIndex => "\nI/O APIC Error: Invalid GSI Index\n",
        }
        .output();
    }
}

pub fn init(
    frame_buffer_base: u64,
    screen_width: usize,
    screen_height: usize,
    screen_stride: usize,
) {
    frame_buffer::set_config(frame_buffer_base, 4 * screen_stride * screen_height);
    screen::set_config(screen_width, screen_height, screen_stride);
    screen::clear();
}
