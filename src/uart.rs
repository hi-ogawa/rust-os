use crate::asm::{inb, outb};
use crate::lazy_static;
use crate::util::Mutex;
use core::fmt;

// cf. https://wiki.osdev.org/Serial_Ports

pub struct SerialPort {
    port: u16,
}

impl SerialPort {
    pub fn new(port: u16) -> Self {
        Self { port }
    }

    pub fn init(&mut self) {
        let p = self.port;

        // Disable interrupts
        outb(p + 1, 0);

        // Set baudrate divisor = 3
        outb(p + 3, 0b1000_0000); // Set DLAB mode
        outb(p + 0, 3); // (lo)
        outb(p + 1, 0); // (hi)

        // Set protocol (8 bits, no parity, one stop bit) and unset DLAB mode
        outb(p + 3, 0b0000_0011);

        // and more ...
        outb(p + 2, 0xC7); // Enable FIFO, clear them, with 14-byte threshold
        outb(p + 4, 0x0B); // IRQs enabled, RTS/DSR set

        // Test loopback
        outb(p + 4, 0x1E); // Loopback mode
        outb(p + 0, 0xAE); // Send 0xAE
        assert_eq!(inb(p + 0), 0xAE); // Receive 0xAE

        // Set normal mode
        outb(p + 4, 0x0F);
    }

    pub fn write_byte(&mut self, value: u8) {
        while (inb(self.port + 5) & 0b0010_0000) == 0 {} // Wait until the transmission buffer is empty
        outb(self.port, value);
    }

    pub fn write_string<'a>(&mut self, s: &'a str) {
        for c in s.bytes() {
            self.write_byte(c);
        }
    }
}

impl fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

lazy_static! {
    pub static ref SERIAL: Mutex<SerialPort> = {
        let mut serial = SerialPort { port: 0x3F8 };
        serial.init();
        Mutex::new(serial)
    };
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        $crate::uart::SERIAL.lock().write_fmt(format_args!($($arg)*)).unwrap();
    });
}

#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($($arg:tt)*) => ($crate::serial_print!("{}\n", format_args!($($arg)*)));
}
