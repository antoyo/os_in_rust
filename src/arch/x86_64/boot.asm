global start

section .text
bits 32
start:
    mov dword [0xB8000], 0x2F4B2F4F  ; Print OK to screen
    hlt
