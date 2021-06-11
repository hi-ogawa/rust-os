#![no_std]
#![feature(naked_functions)]
#![feature(asm)]

use os::asm::{hlt, inb, sti};
use os::idt::{Idt, IsrArg};
use os::keyboard;
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

    idt.set_irq_handler(PicIndex::Timer as u8, make_isr!(timer_handler));
    idt.set_irq_handler(PicIndex::Keyboard as u8, make_isr!(keyboard_handler));

    // Enable interrupt
    sti();

    hlt();

    loop {}
}

extern "C" fn timer_handler(_arg: &IsrArg) {
    // no-op
    pic1_eoi();
}

extern "C" fn keyboard_handler(_arg: &IsrArg) {
    let code = inb(keyboard::PORT);
    let release = code & 0x80 != 0;
    let byte = keyboard::TABLE[(code & !0x80) as usize];
    let ascii = core::ascii::escape_default(byte);
    serial_println!(
        "KEYBOARD: code = {}, byte = {}, ascii = {}, release = {}",
        code,
        byte,
        ascii,
        release,
    );
    pic1_eoi();

    if byte == b'q' {
        qemu::exit_success();
    }
}
