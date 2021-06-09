use crate::asm::outl;

// cf. https://github.com/qemu/qemu/blob/master/hw/misc/debugexit.c

const EXIT_PORT: u16 = 0x501;

pub fn exit(value: u32) {
    // qemu will exit with (value << 1) | 1
    outl(EXIT_PORT, value);
}
