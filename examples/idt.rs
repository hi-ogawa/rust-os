#![no_std]
#![feature(core_intrinsics)]

use os::asm::int3;
use os::idt::{IdtIndex, IsrArg, IDT};
use os::qemu;
use os::serial_println;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    serial_println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    // idt address
    serial_println!("IDT address {:x}", &IDT.offset());

    // lidt
    serial_println!("BEFORE `idt.load`");
    unsafe {
        IDT.load();
    }
    serial_println!("AFTER `idt.load`");

    // Set handler for breakpoint exception
    unsafe {
        IDT.set_handler(IdtIndex::Breakpoint, &breakpoint_handler);
    }

    // Trigger breakpoint exception
    serial_println!("BEFORE `int3`");
    int3();
    serial_println!("AFTER `int3`");

    qemu::exit(1);
    loop {}
}

fn breakpoint_handler(arg: &IsrArg) {
    serial_println!("BREAKPOINT: index = {}, error_code = {}", { arg.index }, {
        arg.error_code
    });
}
