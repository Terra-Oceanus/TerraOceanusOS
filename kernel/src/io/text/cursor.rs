//! Cursor

use super::output::{font, frame_buffer, screen};

const CURSOR_WIDTH: usize = 2;

static mut CURSOR: Cursor = Cursor {
    x: 0,
    y: 0,
    dx: 0,
    dy: 0,
    ptr: core::ptr::null_mut(),
};

static mut CACHE: [u8; font::HEIGHT] = [0u8; font::HEIGHT];

#[repr(u32)]
pub enum Color {
    Black,
    White = 0x00FFFFFF,
}

pub struct Cursor {
    pub x: usize,
    pub y: usize,

    dx: usize,
    dy: usize,

    pub ptr: *mut u32,
}
impl Cursor {
    pub fn init() {
        unsafe {
            CURSOR.ptr = (frame_buffer::base() as *mut u32)
                .add(screen::MARGIN * screen::stride())
                .add(screen::MARGIN);
        }
    }

    pub fn max_x() -> usize {
        (screen::width() - screen::MARGIN * 2) / font::WIDTH
    }

    pub fn max_y() -> usize {
        (screen::height() - screen::MARGIN * 2) / font::HEIGHT
    }

    fn out_pixel(color: u32) {
        unsafe {
            (CURSOR.ptr as *mut u32).write_volatile(color);

            // Next Pixel
            if CURSOR.dx + 1 == font::WIDTH {
                CURSOR.dx = 0;
                if CURSOR.dy + 1 == font::HEIGHT {
                    CURSOR.dy = 0;
                } else {
                    CURSOR.dy += 1;
                    CURSOR.ptr = CURSOR.ptr.add(screen::stride() - font::WIDTH + 1);
                }
            } else {
                CURSOR.dx += 1;
                CURSOR.ptr = CURSOR.ptr.add(1);
            }
        }
    }

    pub fn read_cache() {
        unsafe {
            let mut ptr = CURSOR.ptr;
            for y in 0..font::HEIGHT {
                for x in 0..CURSOR_WIDTH {
                    ptr.write_volatile(if (CACHE[y] >> x) & 1 == 1 {
                        Color::White
                    } else {
                        Color::Black
                    } as u32);
                    ptr = ptr.add(1);
                }
                ptr = ptr.add(screen::stride()).sub(CURSOR_WIDTH);
            }
        }
    }

    pub fn write_cache() {
        unsafe {
            let mut ptr = CURSOR.ptr;
            for y in 0..font::HEIGHT {
                CACHE[y] = 0;
                for x in 0..CURSOR_WIDTH {
                    if ptr.read_volatile() != 0 {
                        CACHE[y] |= 1 << x;
                    }
                    ptr = ptr.add(1);
                }
                ptr = ptr.add(screen::stride()).sub(CURSOR_WIDTH);
            }
        }
    }

    pub fn show() {
        unsafe {
            let mut ptr = CURSOR.ptr;
            for _ in 0..font::HEIGHT {
                for _ in 0..CURSOR_WIDTH {
                    ptr.write_volatile(Color::White as u32);
                    ptr = ptr.add(1);
                }
                ptr = ptr.add(screen::stride()).sub(CURSOR_WIDTH);
            }
        }
    }

    pub fn wrapper<F: FnOnce()>(f: F) {
        Self::read_cache();
        f();
        Self::write_cache();
        Self::show();
    }

    pub fn out_char(c: char) {
        for row in font::bitmap(c) {
            for col in 0..font::WIDTH {
                let color: u32 = if (row >> col) & 1 == 1 {
                    Color::White as u32
                } else {
                    Color::Black as u32
                };
                Self::out_pixel(color);
            }
        }

        // Next Char
        unsafe {
            if CURSOR.x + 1 == Self::max_x() {
                CURSOR.x = 0;
                if CURSOR.y + 1 == Self::max_y() {
                    screen::up();
                    CURSOR.ptr = CURSOR.ptr.sub((Self::max_x() - 1) * font::WIDTH);
                } else {
                    CURSOR.y += 1;
                    CURSOR.ptr = CURSOR
                        .ptr
                        .add(screen::MARGIN + screen::stride() - screen::width() + screen::MARGIN);
                }
                CURSOR.ptr = CURSOR.ptr.add(1);
            } else {
                CURSOR.x += 1;
                CURSOR.ptr = CURSOR
                    .ptr
                    .sub(font::HEIGHT * screen::stride())
                    .add(screen::stride())
                    .add(1);
            }
        }
        Self::write_cache();
    }

    pub fn backspace() {
        Self::wrapper(|| unsafe {
            if CURSOR.x == 0 {
                if CURSOR.y == 0 {
                    return;
                }
                CURSOR.x = Self::max_x() - 1;
                CURSOR.y -= 1;
                CURSOR.ptr = CURSOR
                    .ptr
                    .sub(font::HEIGHT * screen::stride())
                    .add((Self::max_x() - 1) * font::WIDTH);
            } else {
                CURSOR.x -= 1;
                CURSOR.ptr = CURSOR.ptr.sub(font::WIDTH);
            }
            screen::left(CURSOR.x, CURSOR.y, CURSOR.ptr);
        });
    }

    pub fn tab() {
        Self::wrapper(|| unsafe {
            CURSOR.ptr = CURSOR.ptr.add(font::WIDTH * 4);
            CURSOR.x += 4;
        });
    }

    pub fn enter() {
        Self::wrapper(|| unsafe {
            if CURSOR.y + 1 == Self::max_y() {
                CURSOR.ptr = CURSOR.ptr.sub(CURSOR.x * font::WIDTH);
                screen::up();
            } else {
                CURSOR.ptr = CURSOR
                    .ptr
                    .sub(CURSOR.x * font::WIDTH)
                    .add(font::HEIGHT * screen::stride());
                CURSOR.y += 1;
            }
            CURSOR.x = 0;
        });
    }

    pub fn space() {
        Self::wrapper(|| unsafe {
            screen::left(CURSOR.x, CURSOR.y, CURSOR.ptr);
        });
    }

    pub fn home() {}

    pub fn up() {
        Self::wrapper(|| unsafe {
            if CURSOR.y != 0 {
                CURSOR.y -= 1;
                CURSOR.ptr = CURSOR.ptr.sub(font::HEIGHT * screen::stride());
            }
        });
    }

    pub fn left() {
        Self::wrapper(|| unsafe {
            if CURSOR.x == 0 {
                if CURSOR.y == 0 {
                    return;
                }
                CURSOR.x = Self::max_x();
                CURSOR.y -= 1;
                CURSOR.ptr = CURSOR
                    .ptr
                    .sub(screen::MARGIN + screen::stride() - screen::width() + screen::MARGIN)
                    .sub(font::HEIGHT * screen::stride())
                    .add(screen::stride());
            }
            CURSOR.x -= 1;
            CURSOR.ptr = CURSOR.ptr.sub(font::WIDTH);
        });
    }

    pub fn right() {
        Self::wrapper(|| unsafe {
            if CURSOR.y != 0 {
                CURSOR.y -= 1;
                CURSOR.ptr = CURSOR.ptr.sub(font::HEIGHT * screen::stride());
            }
        });
    }

    pub fn end() {}

    pub fn down() {
        Self::wrapper(|| unsafe {
            if CURSOR.y + 1 != Self::max_y() {
                CURSOR.y += 1;
                CURSOR.ptr = CURSOR.ptr.add(font::HEIGHT * screen::stride());
            }
        });
    }

    pub fn delete() {
        Self::wrapper(|| unsafe { screen::left(CURSOR.x, CURSOR.y, CURSOR.ptr) });
    }
}
