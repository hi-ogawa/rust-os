// cf.
// - https://doc.rust-lang.org/nightly/unstable-book/library-features/llvm-asm.html
// - https://llvm.org/docs/LangRef.html#inline-assembler-expressions

//
// inb/outb (cf. https://wiki.osdev.org/Inline_Assembly/Examples#I.2FO_access)
//
pub fn inb(port: u16) -> u8 {
    let value: u8;
    unsafe {
        llvm_asm!(
          "inb %dx, %al"
          : "={al}" (value)
          : "N{dx}" (port)
          :
          : "volatile"
        )
    }
    value
}

pub fn outb(port: u16, value: u8) {
    unsafe {
        llvm_asm!(
          "outb %al, %dx"
          :
          : "{al}" (value), "N{dx}" (port)
          :
          : "volatile"
        );
    }
}
