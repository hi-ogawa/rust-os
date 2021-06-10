use crate::asm::lidt;
use crate::lazy_static;

// cf. https://wiki.osdev.org/IDT

const IDT_SIZE: usize = 256;

#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
pub enum IdtIndex {
    DivideByZero = 0,
    Breakpoint = 3,
    DoubleFault = 8,
    GeneralProtectionFault = 13,
    PageFault = 14,
}

#[repr(C)]
#[repr(packed)]
#[derive(Debug, Copy, Clone, Default)]
pub struct IdtEntry {
    pub offset1: u16,
    pub selector: u16,
    pub ist: u8,
    pub type_attr: u8,
    pub offset2: u16,
    pub offset3: u32,
    pub zero: u32,
}

#[repr(C)]
#[repr(packed)]
#[derive(Debug, Copy, Clone, Default)]
pub struct IdtInfo {
    pub size: u16,
    pub offset: u64,
}

#[repr(C)]
#[repr(packed)]
#[derive(Debug, Copy, Clone, Default)]
pub struct IsrArg {
    // Saved registers
    r15: u64,
    r14: u64,
    r13: u64,
    r12: u64,
    r11: u64,
    r10: u64,
    r9: u64,
    r8: u64,
    rbp: u64,
    rdi: u64,
    rsi: u64,
    rbx: u64,
    rdx: u64,
    rcx: u64,
    rax: u64,
    rsp: u64,
    // Some interrupts pushes additional error information (e.g. page fault),
    // Otherwise "zero" is manually pushed
    pub error_code: u64,
    // TODO: More useful stuff on the stack
}

type IdtHandler = extern "C" fn();
type IdtEntries = [IdtEntry; IDT_SIZE];

pub struct Idt {
    info: IdtInfo,
    entries: IdtEntries,
}

impl Idt {
    pub fn new() -> Self {
        Self {
            info: IdtInfo::default(),
            entries: [IdtEntry::default(); IDT_SIZE],
        }
    }

    // Compare with the debug output of `qemu -d int`
    pub fn offset(&self) -> u64 {
        &self.entries as *const _ as u64
    }

    pub fn load(&mut self) {
        self.info.size = (core::mem::size_of::<IdtEntries>() as u16) - 1;
        self.info.offset = self.offset();
        lidt(&self.info as *const _);
    }

    pub fn set_handler(&mut self, index: IdtIndex, handler: IdtHandler) {
        let ptr = handler as *const char as u64; // Tricky raw function pointer extraction
        self.entries[index as usize] = IdtEntry {
            offset1: ptr as u16,
            offset2: (ptr >> 16) as u16,
            offset3: (ptr >> 32) as u32,
            type_attr: 0x8F, // = flag (present + trap gate) (TODO: 0x8E for interrupt gate?)
            selector: 8,     // = first segment (gdt64.code in boot.asm)
            ist: 0,          // TODO: separate stack to prevent triple fault?
            zero: 0,
        };
    }
}

// Duplicate is necessary since "naked functions must contain a single asm block"
#[macro_export]
macro_rules! make_isr {
    ($function:ident) => {{
        #[naked]
        extern "C" fn wrapper() {
            unsafe {
                asm!(
                    "
                    # Clear interrupt
                    cli

                    # Allocate error code manually
                    push 0

                    # Save registers
                    push rsp; push rax; push rcx; push rdx; push rbx; push rsi; push rdi; push rbp
                    push r8; push r9; push r10; push r11; push r12; push r13; push r14; push r15

                    # System V calling convention
                    # Make 1st argument a pointer to stack, which is all the data pushed so far (registers, index, error_code)
                    mov rdi, rsp
                    call {0}

                    # Restore registers
                    pop r15; pop r14; pop r13; pop r12; pop r11; pop r10; pop r9; pop r8
                    pop rbp; pop rdi; pop rsi; pop rbx; pop rdx; pop rcx; pop rax; pop rsp

                    # Deallocate error code
                    add rsp, 8
                    iretq
                    ",
                    sym $function,
                    options(noreturn)
                );
            }
        }
        wrapper
    }};

    // Only difference is "Allocate error code manually"
    ($function:ident, has_error_code) => {{
        #[naked]
        extern "C" fn wrapper() {
            unsafe {
                asm!(
                    "
                    # Clear interrupt
                    cli

                    # DO NOT allocate error code
                    # push 0

                    # Save registers
                    push rsp; push rax; push rcx; push rdx; push rbx; push rsi; push rdi; push rbp
                    push r8; push r9; push r10; push r11; push r12; push r13; push r14; push r15

                    # System V calling convention
                    # Make 1st argument a pointer to stack, which is all the data pushed so far (registers, index, error_code)
                    mov rdi, rsp
                    call {0}

                    # Restore registers
                    pop r15; pop r14; pop r13; pop r12; pop r11; pop r10; pop r9; pop r8
                    pop rbp; pop rdi; pop rsi; pop rbx; pop rdx; pop rcx; pop rax; pop rsp

                    # Deallocate error code
                    add rsp, 8
                    iretq
                    ",
                    sym $function,
                    options(noreturn)
                );
            }
        }
        wrapper
    }};
}

lazy_static! {
    // TODO:
    //   This doesn't work
    //     { let mut idt = Idt::new(); idt.load(); idt }
    //   since idt.offset() changes during lazy_static.
    //   probably this is because of a bunch of undefined behaviours happening in my lazy static.
    pub static mut IDT : Idt = Idt::new();
}
