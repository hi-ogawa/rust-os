// This example doesn't work anymore since we switched to multiboot2

#![no_std]

use os::multiboot::BootInfo;
use os::qemu;
use os::serial_println;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    serial_println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn kernel_main(boot_info: &BootInfo) -> ! {
    serial_println!("sizeof(BootInfo) = {}", core::mem::size_of::<BootInfo>());
    serial_println!("address = 0x{:08x}", boot_info as *const _ as u32);
    serial_println!("flags = 0b{:b}", { boot_info.flags });

    assert!(boot_info.flags & (1 << 5) != 0); // ELF section
    assert!(boot_info.flags & (1 << 6) != 0); // mmap
    assert!(boot_info.flags & (1 << 12) != 0); // framebuffer

    serial_println!(
        "framebuffer = (0x{:x}, {}, {})",
        { boot_info.framebuffer_addr },
        { boot_info.framebuffer_width },
        { boot_info.framebuffer_height }
    );

    serial_println!("mmap_addr = 0x{:08x}", { boot_info.mmap_addr });
    serial_println!("mmap_length = 0x{:08x}", { boot_info.mmap_length });
    for mmap in boot_info.memory_maps() {
        serial_println!(
            "mmap: addr = 0x{:08x}, length = 0x{:08x}, type = {}",
            { mmap.addr },
            { mmap.length },
            { mmap.type_ }
        );
    }

    if cfg!(feature = "os-test") {
        let header = boot_info.section_headers().nth(1).unwrap();
        serial_println!("section: addr = 0x{:08x}", { header.addr });
    } else {
        for header in boot_info.section_headers() {
            serial_println!(
                "section: addr = 0x{:08x}, size = 0x{:08x}",
                { header.addr },
                { header.size }
            );
        }

        let kernel_start = boot_info
            .section_headers()
            .filter(|sh| sh.size > 0)
            .map(|sh| sh.addr)
            .min()
            .unwrap();
        let kernel_end = boot_info
            .section_headers()
            .filter(|sh| sh.size > 0)
            .map(|sh| sh.addr + sh.size)
            .max()
            .unwrap();
        serial_println!("kernel: [0x{:08x}, 0x{:08x}]", kernel_start, kernel_end);
    }

    let (boot_info_start, boot_info_end) = boot_info.memory_usage();
    serial_println!(
        "boot_info: [0x{:08x}, 0x{:08x}]",
        boot_info_start,
        boot_info_end
    );

    qemu::exit_success();
    loop {}
}
