ENTRY(start)

SECTIONS {
    . = 1M;

    .boot :
    {
        /* multiboot header at the beginning */
        *(.multiboot_header)
    }

    .text :
    {
        *(.text)
    }
}
