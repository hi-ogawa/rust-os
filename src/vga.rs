use crate::lazy_static;
use crate::util::{address_cast_mut, Volatile};
use core::fmt;

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

const BUFFER_WIDTH: usize = 80;
const BUFFER_HEIGHT: usize = 25;
type Buffer = [[Volatile<u16>; BUFFER_WIDTH]; BUFFER_HEIGHT];

pub fn make_code(character: u8, foreground: Color, background: Color) -> u16 {
    ((background as u16) << 12) | ((foreground as u16) << 8) | (character as u16)
}

pub struct Writer {
    buffer: &'static mut Buffer,
    foreground: Color,
    background: Color,
    x: usize,
    y: usize,
}

impl Writer {
    pub fn new(buffer: &'static mut Buffer, foreground: Color, background: Color) -> Self {
        Self {
            buffer,
            foreground,
            background,
            x: 0,
            y: 0,
        }
    }

    pub unsafe fn from_address(address: usize, foreground: Color, background: Color) -> Self {
        Writer::new(address_cast_mut(address), foreground, background)
    }

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

    fn read_byte_at(&mut self, x: usize, y: usize) -> u8 {
        self.buffer[y][x].read() as u8
    }

    fn write_byte_at(&mut self, c: u8, x: usize, y: usize) {
        let code = make_code(c, self.foreground, self.background);
        self.buffer[y][x].write(code);
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

lazy_static! {
    pub static mut WRITER : Writer = {
        let mut writer = Writer::from_address(0xb8000, Color::Gray, Color::Black);
        writer.clear();
        writer
    };
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
