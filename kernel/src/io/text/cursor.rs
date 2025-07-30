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
                    CURSOR.ptr = CURSOR.ptr.add(screen::stride()).sub(font::WIDTH - 1);
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

    pub fn out_char(c: char, right: bool) {
        if right {
            if !screen::last_is_black() {
                Self::up();
                screen::up();
            }
            unsafe { screen::right(CURSOR.x, CURSOR.y, CURSOR.ptr) };
        }

        for row in font::bitmap(c) {
            for col in 0..font::WIDTH {
                Self::out_pixel(if (row >> col) & 1 == 1 {
                    Color::White as u32
                } else {
                    Color::Black as u32
                });
            }
        }

        // Next Char
        unsafe {
            if CURSOR.x + 1 == Self::max_x() {
                CURSOR.x = 0;
                if CURSOR.y + 1 == Self::max_y() {
                    screen::up();
                    CURSOR.ptr = CURSOR
                        .ptr
                        .sub((font::HEIGHT - 1) * screen::stride())
                        .sub(Self::max_x() * font::WIDTH - 1);
                } else {
                    CURSOR.y += 1;
                    CURSOR.ptr = CURSOR
                        .ptr
                        .add(screen::MARGIN)
                        .add(screen::stride() - screen::width())
                        .add(screen::MARGIN)
                        .add(1);
                }
            } else {
                CURSOR.x += 1;
                CURSOR.ptr = CURSOR.ptr.sub((font::HEIGHT - 1) * screen::stride() - 1);
            }
        }
    }

    pub fn backspace() {
        Self::left();
        unsafe { screen::left(CURSOR.x, CURSOR.y, CURSOR.ptr) };
    }

    pub fn tab() {
        for _ in 0..4 {
            Self::space();
        }
    }

    pub fn enter() {
        unsafe {
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
        }
    }

    pub fn space() {
        if !screen::last_is_black() {
            Self::up();
            screen::up();
        }
        unsafe { screen::right(CURSOR.x, CURSOR.y, CURSOR.ptr) };
        Self::out_char(' ', false);
    }

    pub fn home() {}

    pub fn up() {
        unsafe {
            if CURSOR.y != 0 {
                CURSOR.y -= 1;
                CURSOR.ptr = CURSOR.ptr.sub(font::HEIGHT * screen::stride());
            }
        }
    }

    pub fn left() {
        unsafe {
            if CURSOR.x == 0 {
                if CURSOR.y == 0 {
                    return;
                }
                CURSOR.x = Self::max_x() - 1;
                CURSOR.y -= 1;
                CURSOR.ptr = CURSOR
                    .ptr
                    .sub(screen::MARGIN)
                    .sub(screen::stride() - screen::width())
                    .sub(screen::MARGIN)
                    .sub((font::HEIGHT - 1) * screen::stride())
                    .sub(font::WIDTH);
            } else {
                CURSOR.x -= 1;
                CURSOR.ptr = CURSOR.ptr.sub(font::WIDTH);
            }
        }
    }

    pub fn right() {
        unsafe {
            if CURSOR.x + 1 == Self::max_x() {
                if CURSOR.y + 1 == Self::max_y() {
                    return;
                }
                CURSOR.x = 0;
                CURSOR.y += 1;
                CURSOR.ptr = CURSOR
                    .ptr
                    .add(font::WIDTH)
                    .add(screen::MARGIN)
                    .add(screen::stride() - screen::width())
                    .add(screen::MARGIN)
                    .add((font::HEIGHT - 1) * screen::stride());
            } else {
                CURSOR.x += 1;
                CURSOR.ptr = CURSOR.ptr.add(font::WIDTH);
            }
        }
    }

    pub fn end() {}

    pub fn down() {
        unsafe {
            if CURSOR.y + 1 != Self::max_y() {
                CURSOR.y += 1;
                CURSOR.ptr = CURSOR.ptr.add(font::HEIGHT * screen::stride());
            }
        }
    }

    pub fn delete() {
        unsafe { screen::left(CURSOR.x, CURSOR.y, CURSOR.ptr) }
    }
}
