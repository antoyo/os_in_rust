ENTRY(start)

SECTIONS {
    . = 1M;

    .boot :
    {
        /* multiboot header at the beginning */
        KEEP(*(.multiboot_header))
    }

    .text :
    {
        *(.text)
    }
}
