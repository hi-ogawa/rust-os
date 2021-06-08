#![no_std]

use os::println;
use os::serial_println;

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
    println!("hello vga");
    serial_println!("hello serial");
    loop {}
}
