//! Output

use super::Cursor;

pub mod font;
pub mod frame_buffer;
pub mod screen;

pub trait Output {
    fn out(&self);

    fn nibble_to_hex_char(nibble: u8) -> char {
        if nibble < 10 {
            (b'0' + nibble) as char
        } else {
            (b'A' + (nibble - 10)) as char
        }
    }

    fn byte_to_hex_str(n: u8) {
        Self::nibble_to_hex_char(n >> 4).out();
        Self::nibble_to_hex_char(n & 0xF).out();
    }
}
impl Output for usize {
    fn out(&self) {
        if *self == 0 {
            '0'.out();
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
                (byte as char).out();
            }
        }
    }
}
impl Output for u8 {
    fn out(&self) {
        "0b".out();

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
                '0'.out();
            } else {
                '1'.out();
            }
        }

        if zero {
            '0'.out();
        }
    }
}
impl Output for u64 {
    fn out(&self) {
        "0x".out();

        let mut zero = true;
        for i in (0..8).rev() {
            let byte = (self >> (i * 8)) as u8;
            if zero {
                if byte == 0 && i != 0 {
                    continue;
                }
                zero = false;
            }
            Self::byte_to_hex_str(byte);
        }

        if zero {
            '0'.out();
        }
    }
}
impl Output for bool {
    fn out(&self) {
        if *self { "True" } else { "False" }.out()
    }
}
impl Output for char {
    fn out(&self) {
        if self.is_ascii_control() {
            match self {
                '\t' => {
                    for _ in 0..4 {
                        Cursor::right();
                    }
                }
                '\n' => Cursor::enter(),
                _ => {}
            }
        } else {
            Cursor::out_char(*self, false);
        }
    }
}
impl Output for &str {
    fn out(&self) {
        for c in self.chars() {
            c.out();
        }
    }
}
impl Output for [u8] {
    fn out(&self) {
        self.iter().for_each(|&c| (c as char).out());
    }
}

pub fn init(
    frame_buffer_base: usize,
    screen_width: usize,
    screen_height: usize,
    screen_stride: usize,
) {
    frame_buffer::set_config(frame_buffer_base, 4 * screen_stride * screen_height);
    screen::set_config(screen_width, screen_height, screen_stride);
}
