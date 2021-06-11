#![no_std]

use core::fmt::Write;
use os::qemu;
use os::uart::SerialPort;

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
    let mut serial = SerialPort::new(0x3F8);
    serial.init();
    write!(serial, "Hello {}", "World!\n").unwrap();
    qemu::exit_success();
    loop {}
}
