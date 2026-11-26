ENTRY(_start)

SECTIONS
{
    . = 0x80000000;

    .text : {
        KEEP(*(.text.entry))   /* _start goes first */
        *(.text .text.*)
    }

    .rodata : {
        . = ALIGN(8);
        *(.rodata .rodata.*)
    }

    .data : {
        . = ALIGN(8);
        _data_start = .;
        *(.data .data.*)
        _data_end = .;
    }

    .bss : {
        . = ALIGN(8);
        _bss_start = .;
        *(.bss .bss.*)
        _bss_end = .;
    }

    . = ALIGN(16);
    . += 32768;  /* 32 KiB stack */
    _stack_top = .;
}
