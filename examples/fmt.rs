#![no_std]

use os::qemu;
use os::serial_println;

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
    serial_println!("{} / {} = {}", 1, 3, 1.0 / 3.0);
    qemu::exit_success();
    loop {}
}
