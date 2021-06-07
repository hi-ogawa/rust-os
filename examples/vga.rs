#![no_std]

use core::fmt::Write;
use os::vga::{Color, Writer};

//
// Panic handler
//
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

//
// Main
//
#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    let mut writer = Writer::new(0xb8000 as *mut u16, Color::White, Color::Blue);
    writer.clear();
    write!(writer, "Hello {}", "World!").unwrap();
    loop {}
}
