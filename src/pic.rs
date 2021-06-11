use crate::asm::outb;

// cf. https://wiki.osdev.org/PIC#Programming_with_the_8259_PIC

const PIC1_COMMAND_PORT: u16 = 0x20;
const PIC2_COMMAND_PORT: u16 = 0xA0;
const PIC1_DATA_PORT: u16 = PIC1_COMMAND_PORT + 1;
const PIC2_DATA_PORT: u16 = PIC2_COMMAND_PORT + 1;

const PIC_INIT: u8 = 0x11;
const PIC_8086: u8 = 0x01;
const PIC_EOI: u8 = 0x20;

const PIC1_IDT_OFFSET: u8 = 32; // PIC0 uses 32..40
const PIC2_IDT_OFFSET: u8 = 40; // PIC1 uses 40..48

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
pub enum PicIndex {
    Timer = PIC1_IDT_OFFSET + 0,
    Keyboard = PIC1_IDT_OFFSET + 1,
}

pub struct Pic {}

impl Pic {
    pub fn new() -> Self {
        Self {}
    }

    pub fn init(&mut self) {
        outb(PIC1_COMMAND_PORT, PIC_INIT);
        outb(PIC2_COMMAND_PORT, PIC_INIT);

        // Offset
        outb(PIC1_DATA_PORT, PIC1_IDT_OFFSET);
        outb(PIC2_DATA_PORT, PIC2_IDT_OFFSET);

        // Cascading
        outb(PIC1_DATA_PORT, 4);
        outb(PIC2_DATA_PORT, 2);

        // 8086/88 mode
        outb(PIC1_DATA_PORT, PIC_8086);
        outb(PIC2_DATA_PORT, PIC_8086);

        // No masking
        outb(PIC1_DATA_PORT, 0);
        outb(PIC2_DATA_PORT, 0);
    }
}

pub extern "C" fn pic1_eoi() {
    outb(PIC1_COMMAND_PORT, PIC_EOI);
}

pub extern "C" fn pic2_eoi() {
    outb(PIC2_COMMAND_PORT, PIC_EOI);
}
