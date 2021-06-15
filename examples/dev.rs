#![no_std]
#![feature(asm)]
#![feature(naked_functions)]

use os::asm::int3;
use os::idt::{IdtIndex, IsrArg, IDT};
use os::make_isr;
use os::println;
use os::qemu;
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

    IDT.lock().load();
    IDT.lock()
        .set_handler(IdtIndex::Breakpoint, make_isr!(breakpoint_handler));

    serial_println!("BEFORE int3");
    int3();
    serial_println!("AFTER int3");

    qemu::exit_success();
    loop {}
}

extern "C" fn breakpoint_handler(arg: &IsrArg) {
    serial_println!("BREAKPOINT: error_code = {}", { arg.error_code });
}
