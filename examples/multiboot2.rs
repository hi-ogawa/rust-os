#![no_std]

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
    serial_println!("{:?}", boot_info);

    #[cfg(not(feature = "os-test"))]
    {
        let addr = boot_info as *const _ as u32;
        serial_println!("boot_info address = 0x{:08x}", addr);
    }

    for (tag, _addr) in boot_info.tags() {
        #[cfg(not(feature = "os-test"))]
        serial_println!("0x{:08x}: {:?}", _addr, tag);

        #[cfg(feature = "os-test")]
        serial_println!("{:?}", tag);
    }

    serial_println!("{:?}", boot_info.framebuffer().unwrap());

    let memory_map = boot_info.memory_map().unwrap();
    let section_headers = boot_info.section_headers().unwrap();

    #[cfg(not(feature = "os-test"))]
    {
        for item in memory_map {
            serial_println!("{:?}", item);
        }

        for item in section_headers {
            serial_println!("{:?}", item);
        }

        serial_println!("memory map");
        for item in memory_map {
            serial_println!(
                "   address = 0x{:08x}, length = 0x{:08x}",
                { item.base_addr },
                { item.length }
            );
        }

        serial_println!("section header");
        for item in section_headers {
            serial_println!("   address = 0x{:08x}, length = 0x{:08x}", { item.addr }, {
                item.size
            });
        }

        serial_println!("usable memory");
        for (lo, hi) in boot_info.usable_memory() {
            serial_println!("   (0x{:08x}, 0x{:08x})", lo, hi);
        }

        serial_println!("occupied memory");
        for (lo, hi) in boot_info.occupied_memory() {
            serial_println!("   (0x{:08x}, 0x{:08x})", lo, hi);
        }
    }

    #[cfg(feature = "os-test")]
    {
        serial_println!("memory map (3): address = 0x{:08x}", {
            memory_map.clone().nth(3).unwrap().base_addr
        });

        serial_println!("section header (1): address = 0x{:08x}", {
            section_headers.clone().nth(1).unwrap().addr
        });
    }

    qemu::exit_success();
    loop {}
}
