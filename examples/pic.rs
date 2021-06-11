#![no_std]
#![feature(naked_functions)]
#![feature(asm)]

use os::asm::{int3, sti};
use os::idt::{Idt, IdtIndex, IsrArg};
use os::make_isr;
use os::pic::{pic1_eoi, Pic, PicIndex};
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
    let mut pic = Pic::new();

    idt.load();
    pic.init();

    // Set handlers
    idt.set_handler(IdtIndex::Breakpoint, make_isr!(breakpoint_handler));
    idt.set_irq_handler(PicIndex::Timer as u8, make_isr!(timer_handler));

    // Enable interrupt
    sti();

    // Trigger breakpoint exception
    int3();

    loop {}
}

extern "C" fn breakpoint_handler(arg: &IsrArg) {
    serial_println!("BREAKPOINT: error_code = {}", { arg.error_code });
}

static mut TIMER_COUNTER: usize = 1;

extern "C" fn timer_handler(arg: &IsrArg) {
    serial_println!(
        "TIMER: error_code = {}, counter = {}",
        { arg.error_code },
        TIMER_COUNTER
    );
    unsafe {
        TIMER_COUNTER += 1;
        if TIMER_COUNTER == 10 {
            qemu::exit_success();
        }
    }
    pic1_eoi();
}
