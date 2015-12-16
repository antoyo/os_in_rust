section .multiboot_header
header_start:
    dd 0xE85250D6                   ; Magic number (Multiboot 2)
    dd 0                            ; Architecture 0 (protected mode i386)
    dd header_end - header_start    ; Header length
    ; Checksum
    dd 0x100000000 - (0xE85250D6 + 0 + (header_end - header_start))

    ; Multiboot tags

    ; End tag
    dw 0    ; Type
    dw 0    ; Flags
    dd 8    ; Size
header_end:
