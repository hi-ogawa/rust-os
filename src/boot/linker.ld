ENTRY(start)

SECTIONS {
    . = 1M;

    .text :
    {
        *(.multiboot_header) /* header at the beginning */
        *(.text)
    }

    .rodata :
    {
        *(.rodata)
    }

    .bss :
    {
        *(.bss)
        *(.kernel_stack) /* stack at the end */
    }
}
