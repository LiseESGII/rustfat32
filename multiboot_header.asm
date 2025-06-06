section .multiboot_header
align 8
header_start:
    dd 0xe85250d6          ; magic number (multiboot2)
    dd 0                   ; architecture (0 = i386)
    dd header_end - header_start ; header length
    dd 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start)) ; checksum

    ; Required end tag
    dw 0                   ; type
    dw 0                   ; flags
    dd 8                   ; size
header_end:
