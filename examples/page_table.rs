#![no_std]

use os::memory::paging::{get_child_table, get_p4_table, virtual_to_physical, PRESENT};
use os::multiboot2::BootInfo;
use os::qemu;
use os::serial_println;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    serial_println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn kernel_main(_boot_info: &BootInfo) -> ! {
    let p4 = get_p4_table();
    serial_println!("present p4 entry");
    for (i, &entry) in p4.iter().enumerate() {
        if entry & PRESENT != 0 {
            serial_println!("   {}", i);
        }
    }

    let p3 = get_child_table(p4, 0).unwrap();
    serial_println!("present p3 entry");
    for (i, &entry) in p3.iter().enumerate() {
        if entry & PRESENT != 0 {
            serial_println!("   {}", i);
        }
    }

    let p2 = get_child_table(p3, 0).unwrap();
    serial_println!("present p2 entry (% 100)");
    for (i, &entry) in p2.iter().enumerate() {
        if i % 100 == 0 && entry & PRESENT != 0 {
            serial_println!("   {}", i);
        }
    }

    let p1 = get_child_table(p2, 0).unwrap();
    serial_println!("present p1 entry (% 100)");
    for (i, &entry) in p1.iter().enumerate() {
        if i % 100 == 0 && entry & PRESENT != 0 {
            serial_println!("   {}", i);
        }
    }

    serial_println!("virtual_to_physical");
    serial_println!("   0           ==> {:?}", virtual_to_physical(0));
    serial_println!("   1 << 30 - 1 ==> {:?}", virtual_to_physical(1 << 30 - 1));
    serial_println!("   1 << 30     ==> {:?}", virtual_to_physical(1 << 30));
    serial_println!("   0xdeadbeaf  ==> {:?}", virtual_to_physical(0xdeadbeaf));

    qemu::exit_success();
    loop {}
}
