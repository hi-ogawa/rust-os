#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  loop {}
}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
  let text = b"Hello World!";
  let color_code: u8 = 0x2f;

  let ptr = 0xb8000 as *mut u8;

  for (i, &c) in text.iter().enumerate() {
    let i = i as isize;
    unsafe {
      *ptr.offset(2 * i) = c;
      *ptr.offset(2 * i + 1) = color_code;
    }
  }

  loop {}
}
