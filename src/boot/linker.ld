ENTRY(start)

SECTIONS {
    . = 1M;

    .text :
    {
        *(.multiboot2_header) /* boot header at the beginning */
        *(.text .text.*)
    }

    .rodata : {
        *(.rodata .rodata.*)
    }

    .data.rel.ro : {
        *(.data.rel.ro.local*) *(.data.rel.ro .data.rel.ro.*)
    }

    .bss :
    {
        *(.bss .bss.*)
        *(.kernel_stack) /* stack at the end */
    }
}
