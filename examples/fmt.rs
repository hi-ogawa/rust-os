#![no_std]

use os::println;
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
    println!("{} / {} = {}", 1, 3, 1.0 / 3.0);
    loop {}
}
