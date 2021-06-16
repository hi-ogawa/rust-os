#![no_std]

use os::memory::{FrameAllocator, SimpleFrameAllocator};
use os::multiboot2::BootInfo;
use os::qemu;
use os::serial_println;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    serial_println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn kernel_main(boot_info: &BootInfo) -> ! {
    #[cfg(not(os_test))]
    {
        serial_println!("usable memory");
        for (lo, hi) in boot_info.usable_memory() {
            serial_println!("   (0x{:08x}, 0x{:08x})", lo, hi);
        }

        serial_println!("occupied memory");
        for (lo, hi) in boot_info.occupied_memory() {
            serial_println!("   (0x{:08x}, 0x{:08x})", lo, hi);
        }
    }

    let mut frame_allocator =
        SimpleFrameAllocator::new(boot_info.usable_memory(), boot_info.occupied_memory());
    while let Some(index) = frame_allocator.allocate() {
        if index % 1000 == 0 {
            serial_println!("{}", index);
        }
    }

    qemu::exit_success();
    loop {}
}
