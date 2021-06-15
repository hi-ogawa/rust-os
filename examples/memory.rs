#![no_std]
#![feature(naked_functions)]
#![feature(asm)]

use os::asm::{flush_tlb, read_cr2};
use os::idt::{IdtIndex, IsrArg, IDT};
use os::make_isr;
use os::memory::paging::{map_page_to_frame, unmap_page, virtual_to_page, virtual_to_physical};
use os::memory::{FrameAllocator, SimpleFrameAllocator};
use os::multiboot2::BootInfo;
use os::qemu;
use os::serial_println;
use os::util::{address_cast, address_cast_mut, Volatile};

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    serial_println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn kernel_main(boot_info: &BootInfo) -> ! {
    //
    // Exception handler
    //
    unsafe {
        IDT.load();
        IDT.set_handler(
            IdtIndex::PageFault,
            make_isr!(page_fault_handler, has_error_code),
        );
    }

    //
    // Paging manipulation
    //

    let address = 0xdeadbeaf;
    let page = virtual_to_page(address);

    serial_println!("BEFORE map_page_to_frame");
    serial_println!(
        "virtual_to_physical(0xdeadbeaf) = {:?}",
        virtual_to_physical(address)
    );

    let mut allocator =
        SimpleFrameAllocator::new(boot_info.usable_memory(), boot_info.occupied_memory());
    let frame = allocator.allocate().unwrap();
    map_page_to_frame(page, frame, &mut allocator);

    serial_println!("AFTER map_page_to_frame");
    serial_println!(
        "virtual_to_physical(0xdeadbeaf) = {:?}",
        virtual_to_physical(address)
    );

    serial_println!("WRITE AND READ");
    unsafe {
        address_cast_mut::<Volatile<u8>>(address as usize).write(1);
    }
    serial_println!(
        "*0xdeadbeaf = {}",
        address_cast::<Volatile<u8>>(address as usize).read()
    );

    serial_println!("BEFORE unmap_page");
    unmap_page(page);
    serial_println!("AFTER unmap_page");
    flush_tlb();

    serial_println!("WRITE");
    unsafe {
        address_cast_mut::<Volatile<u8>>(address as usize).write(2);
    }

    qemu::exit_success();
    loop {}
}

extern "C" fn page_fault_handler(arg: &IsrArg) {
    serial_println!(
        "PAGE_FAULT: error_code = {}, cr2 = 0x{:08x}",
        { arg.error_code },
        read_cr2()
    );
    qemu::exit_success();
}
