use core::fmt;
use core::ptr;

// See the first row of https://en.wikipedia.org/wiki/File:VGA_palette_with_black_borders.svg
#[repr(u8)]
#[derive(Debug, Copy, Clone, Hash)]
#[allow(dead_code)]
pub enum Color {
    Black = 0,
    Blue,
    Green,
    Cyan,
    Red,
    Magenta,
    Brown,
    Gray,
    DarkGray,
    LightBlue,
    LightGreen,
    LightCyan,
    LightRed,
    LightMagenta,
    Yellow,
    White,
}

const BUFFER_WIDTH: isize = 80;
const BUFFER_HEIGHT: isize = 25;
const BUFFER_ADDRESS: usize = 0xb8000;

pub fn make_code(character: u8, foreground: Color, background: Color) -> u16 {
    ((background as u16) << 12) | ((foreground as u16) << 8) | (character as u16)
}

pub struct Writer {
    x: isize,
    y: isize,
    data: *mut u16,
    foreground: Color,
    background: Color,
}

impl Writer {
    pub fn clear(&mut self) {
        for x in 0..BUFFER_WIDTH {
            for y in 0..BUFFER_HEIGHT {
                self.write_byte_at(b' ', x, y);
            }
        }
        self.x = 0;
        self.y = 0;
    }

    pub fn scroll(&mut self) {
        for x in 0..BUFFER_WIDTH {
            for y in 0..(BUFFER_HEIGHT - 1) {
                let c = self.read_byte_at(x, y + 1);
                self.write_byte_at(c, x, y);
            }
        }
        self.y -= 1;
    }

    fn read_byte_at(&mut self, x: isize, y: isize) -> u8 {
        assert!(0 <= x && x < BUFFER_WIDTH);
        assert!(0 <= y && y < BUFFER_HEIGHT);
        let code = unsafe { ptr::read_volatile(self.data.offset(BUFFER_WIDTH * y + x)) };
        code as u8
    }

    fn write_byte_at(&mut self, c: u8, x: isize, y: isize) {
        assert!(0 <= x && x < BUFFER_WIDTH);
        assert!(0 <= y && y < BUFFER_HEIGHT);
        let code = make_code(c, self.foreground, self.background);
        unsafe {
            ptr::write_volatile(self.data.offset(BUFFER_WIDTH * y + x), code);
        }
    }

    pub fn newline(&mut self) {
        while self.x < BUFFER_WIDTH {
            self.write_byte_at(b' ', self.x, self.y);
            self.x += 1;
        }
        self.x = 0;
        self.y += 1;
        if self.y == BUFFER_HEIGHT {
            self.scroll();
        }
    }

    pub fn write_byte(&mut self, c: u8) {
        if c == b'\n' {
            self.newline();
        } else {
            if self.x == BUFFER_WIDTH {
                self.newline();
            }
            self.write_byte_at(c, self.x, self.y);
            self.x += 1;
        }
    }

    pub fn write_string<'a>(&mut self, s: &'a str) {
        for c in s.bytes() {
            let c = if c < 128 { c } else { 254 }; // Check ascii codepoint (otherwise use box shape)
            self.write_byte(c);
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

// Global instance as mutable static
pub static mut WRITER: Writer = Writer {
    x: 0,
    y: 0,
    data: BUFFER_ADDRESS as *mut u16,
    foreground: Color::Gray,
    background: Color::Black,
};

pub fn clear_screen() {
    unsafe { WRITER.clear() }
}

#[macro_export]
macro_rules! print {
  ($($arg:tt)*) => ({
      use core::fmt::Write;
      unsafe {
        $crate::vga::WRITER.write_fmt(format_args!($($arg)*)).unwrap();
      }
  });
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
