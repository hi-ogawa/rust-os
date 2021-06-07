//
// Crate configuration
//
#![no_std]
#![feature(llvm_asm)]

//
// Modules
//
mod asm;
#[macro_use]
mod vga;
#[macro_use]
mod uart;

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
    println!("Hello World!");

    uart::serial_init();
    serial_println!("Hello World!");

    loop {}
}
