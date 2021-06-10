use crate::asm::outb;

// qemu will exit with (value << 1) | 1
// cf. https://github.com/qemu/qemu/blob/master/hw/misc/debugexit.c

const EXIT_PORT: u16 = 0x501;
const EXIT_SUCCESS: u8 = (123 - 1) / 2;
const EXIT_FAIL: u8 = (213 - 1) / 2;

pub fn exit(value: u8) {
    outb(EXIT_PORT, value);
}

pub fn exit_success() {
    exit(EXIT_SUCCESS);
}

pub fn exit_fail() {
    exit(EXIT_FAIL);
}
