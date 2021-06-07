#![no_std]

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
    let buffer = 0xb8000 as *mut u16;
    let text = "Hello World!";
    let color_code: u16 = 0x2f; // (background, foreground) = (blue, white)

    for (i, c) in text.bytes().enumerate() {
        unsafe {
            *buffer.offset(i as isize) = (color_code << 8) | (c as u16);
        }
    }

    loop {}
}
