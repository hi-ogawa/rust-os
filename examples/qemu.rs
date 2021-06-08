#![no_std]

use os::qemu;
use os::serial_println;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    serial_println!("before exit");
    qemu::exit(0);
    serial_println!("after exit");
    loop {}
}
