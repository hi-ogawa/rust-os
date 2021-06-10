#![no_std]
#![feature(core_intrinsics)]
#![feature(naked_functions)]
#![feature(asm)]

use os::asm::{int3, read_cr2};
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

    // idt address
    serial_println!("IDT address {:x}", idt.offset());

    // lidt
    serial_println!("BEFORE `idt.load`");
    idt.load();
    serial_println!("AFTER `idt.load`");

    // Set handler for breakpoint exception
    idt.set_handler(IdtIndex::Breakpoint, make_isr!(breakpoint_handler));
    idt.set_handler(
        IdtIndex::PageFault,
        make_isr!(page_fault_handler, has_error_code),
    );

    // Trigger breakpoint exception twice
    serial_println!("BEFORE `int3` (1)");
    int3();
    serial_println!("AFTER `int3` (1)");

    serial_println!("BEFORE `int3` (2)");
    int3();
    serial_println!("AFTER `int3` (2)");

    // Trigger page fault
    serial_println!("BEFORE page fault");
    unsafe {
        *(0xdeadbeaf as *mut u64) = 0;
    }
    serial_println!("AFTER page fault");

    qemu::exit(1);
    loop {}
}

static mut COUNTER: usize = 1;

extern "C" fn breakpoint_handler(arg: &IsrArg) {
    serial_println!(
        "BREAKPOINT: error_code = {}, counter = {}",
        { arg.error_code },
        COUNTER
    );
    unsafe {
        COUNTER += 1;
    }
}

extern "C" fn page_fault_handler(arg: &IsrArg) {
    serial_println!(
        "PAGE_FAULT: error_code = {}, cr2 = 0x{:08x}",
        { arg.error_code },
        read_cr2()
    );
    panic!("Cannot return from page fault");
}
