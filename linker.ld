ENTRY(_start)

SECTIONS
{
    . = 1M;

    .multiboot_header :
    {
        KEEP(*(.multiboot_header))
    }

    .text : {
        *(.text .text.*)
    }
    .rodata : { *(.rodata .rodata.*) }
    .data : { *(.data .data.*) }
    .bss : { *(.bss .bss.*) *(COMMON) }
}
