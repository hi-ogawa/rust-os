#![no_std]

#[macro_use]
mod vga;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    vga::clear_screen();
    println!("Hello World!");
    loop {}
}
