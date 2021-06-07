#![no_std]

use os::println;
use os::serial_println;
use os::uart;
use os::vga;

//
// Panic handler
//
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

//
// Main
//
#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    vga::clear_screen();
    println!("hello vga");

    uart::serial_init();
    serial_println!("hello serial");

    loop {}
}
