#![no_std]

use os::qemu;
use os::serial_println;

//
// Panic handler
//
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("{}", info);
    qemu::exit_success();
    loop {}
}

//
// Main
//
#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    assert_eq!(1 + 2, 0);
    loop {}
}
