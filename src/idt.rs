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
    // Manually push index to choose handler
    pub index: u64,
    // Some interrupts pushes additional error information (e.g. page fault),
    // Otherwise "zero" is manually pushed
    pub error_code: u64,
}

type IdtHandler = &'static dyn Fn(&IsrArg);

fn default_handler(_isr_arg: &IsrArg) {
    unreachable!();
}

type IdtEntries = [IdtEntry; IDT_SIZE];
type IdtHandlers = [IdtHandler; IDT_SIZE];

pub struct Idt {
    info: IdtInfo,
    entries: IdtEntries,
    handlers: IdtHandlers,
}

impl Idt {
    pub fn new() -> Self {
        Self {
            info: IdtInfo::default(),
            entries: [IdtEntry::default(); IDT_SIZE],
            handlers: [&default_handler; IDT_SIZE],
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
        let index = index as usize;
        let mut entry = &mut self.entries[index];
        let address = unsafe { isr_offsets[index] };
        entry.offset1 = address as u16;
        entry.offset2 = (address >> 16) as u16;
        entry.offset3 = (address >> 32) as u32;
        entry.type_attr = 0x8F; // = flag (present + trap gate) (TODO: 0x8E for interrupt gate)
        entry.selector = 8; // = first segment (gdt64.code in boot.asm)
        entry.ist = 0;
        self.handlers[index] = handler;
    }

    pub fn unset_handler(&mut self, index: IdtIndex) {
        let i = index as usize;
        self.entries[i] = IdtEntry::default();
        self.handlers[i] = &default_handler;
    }
}

lazy_static! {
    pub static mut IDT : Idt = Idt::new();
}

// Defined in boot.asm
extern "C" {
    static isr_offsets: [u64; IDT_SIZE];
}

// Called by `isr_common` in boot.asm
#[no_mangle]
pub extern "C" fn isr_main(arg: *const IsrArg) {
    let arg: &IsrArg = unsafe { &*arg };
    unsafe {
        IDT.handlers[arg.index as usize](arg);
    }
}
