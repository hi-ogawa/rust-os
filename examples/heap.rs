#![no_std]
#![feature(naked_functions)]
#![feature(asm)]
#![feature(alloc_error_handler)]

use os::asm::read_cr2;
use os::idt::{IdtIndex, IsrArg, IDT};
use os::make_isr;
use os::memory::heap_allocator::MutexSimpleHeapAllocator;
use os::memory::paging::{map_page_range, virtual_to_page};
use os::memory::SimpleFrameAllocator;
use os::memory::VirtualAddress;
use os::multiboot2::BootInfo;
use os::qemu;
use os::serial_println;

extern crate alloc;
use alloc::vec::Vec;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    serial_println!("{}", info);
    qemu::exit_fail();
    loop {}
}

#[no_mangle]
pub extern "C" fn kernel_main(boot_info: &BootInfo) -> ! {
    // Set page fault handler
    IDT.lock().load();
    IDT.lock().set_handler(
        IdtIndex::PageFault,
        make_isr!(page_fault_handler, has_error_code),
    );

    // Initialize heap memory
    let mut allocator =
        SimpleFrameAllocator::new(boot_info.usable_memory(), boot_info.occupied_memory());
    map_page_range(
        virtual_to_page(HEAP_START),
        virtual_to_page(HEAP_END),
        &mut allocator,
    );

    // Instantiate vector
    #[cfg(not(heap_fail))]
    let xs: Vec<usize> = (0..100).collect();

    #[cfg(heap_fail)]
    let xs: Vec<usize> = (0..10000).collect();

    serial_println!("sum = {}", xs.iter().sum::<usize>());

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

//
// Rust heap allocator interface
//

const HEAP_START: VirtualAddress = 0x0100_0000;
const HEAP_END: VirtualAddress = HEAP_START + (1 << 14); // 16KB = 4 pages

#[global_allocator]
static HEAP_ALLOCATOR: MutexSimpleHeapAllocator =
    MutexSimpleHeapAllocator::new(HEAP_START as usize, HEAP_END as usize);

#[alloc_error_handler]
pub fn alloc_error_handler(layout: core::alloc::Layout) -> ! {
    serial_println!("alloc_error: {:?}", layout);
    qemu::exit_success();
    loop {}
}
