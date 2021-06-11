#![no_std]
#![feature(naked_functions)]
#![feature(asm)]
#![feature(core_intrinsics)]

use os::idt::{Idt, IdtIndex, IsrArg};
use os::make_isr;
use os::qemu;
use os::serial_println;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    serial_println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    let mut idt = Idt::new();
    idt.load();
    idt.set_handler(
        IdtIndex::DoubleFault,
        make_isr!(double_fault_handler, has_error_code),
    );

    // Divide by zero
    let x: isize = "1".parse().unwrap();
    let y: isize = "0".parse().unwrap();
    let z = unsafe { core::intrinsics::unchecked_div(x, y) };
    serial_println!("1 / 0 = {}", z);

    loop {}
}

extern "C" fn double_fault_handler(arg: &IsrArg) {
    serial_println!("DOUBLE_FAULT: error_code = {}", { arg.error_code },);
    qemu::exit_success();
}
