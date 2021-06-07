#![no_std]

use os;

//
// Panic handler
//
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os::serial_println!("{}", info);
    loop {}
}

//
// Main
//
#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    os::uart::serial_init();
    assert_eq!(1 + 2, 0);
    loop {}
}
