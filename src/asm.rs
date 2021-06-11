// cf.
// - https://doc.rust-lang.org/nightly/unstable-book/library-features/llvm-asm.html
// - https://llvm.org/docs/LangRef.html#inline-assembler-expressions

//
// in/out (cf. https://wiki.osdev.org/Inline_Assembly/Examples#I.2FO_access)
//

pub fn inb(port: u16) -> u8 {
    let value: u8;
    unsafe {
        llvm_asm!("in %dx, %al" : "={al}" (value) : "{dx}" (port) : : "volatile");
    }
    value
}
pub fn outb(port: u16, value: u8) {
    unsafe {
        llvm_asm!("out %al, %dx" : : "{al}" (value), "{dx}" (port) : : "volatile");
    }
}
pub fn inw(port: u16) -> u16 {
    let value: u16;
    unsafe {
        llvm_asm!("in %dx, %ax" : "={ax}" (value) : "{dx}" (port) : : "volatile");
    }
    value
}
pub fn outw(port: u16, value: u16) {
    unsafe {
        llvm_asm!("out %ax, %dx" : : "{ax}" (value), "{dx}" (port) : : "volatile");
    }
}
pub fn inl(port: u16) -> u32 {
    let value: u32;
    unsafe {
        llvm_asm!("in %dx, %eax" : "={eax}" (value) : "{dx}" (port) : : "volatile");
    }
    value
}
pub fn outl(port: u16, value: u32) {
    unsafe {
        llvm_asm!("out %eax, %dx" : : "{eax}" (value), "{dx}" (port) : : "volatile");
    }
}

// lidt
pub fn lidt<T>(ptr: *const T) {
    // HACK: Use u128 to refer to idt pointer which is 10 bytes
    let ptr = ptr as *const u128;
    unsafe {
        llvm_asm!("lidt $0" : : "m"(*ptr));
    }
}

// int3
pub fn int3() {
    unsafe {
        llvm_asm!("int3");
    }
}

// sti
pub fn sti() {
    unsafe {
        llvm_asm!("sti");
    }
}

pub fn hlt() {
    unsafe {
        llvm_asm!("hlt");
    }
}

// cr2
pub fn read_cr2() -> u64 {
    let value: u64;
    unsafe {
        llvm_asm!("mov %cr2, $0" : "=r"(value));
    }
    value
}
