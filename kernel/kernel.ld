ENTRY(_start)

SECTIONS {
    . = 0x100000;

    .text ALIGN(0x1000) : {
        *(.text._start)
        *(.text*)
    }

    .rodata ALIGN(0x1000) : { *(.rodata*) }

    .data ALIGN(0x1000) : {
        *(.data*)
        *(.got*)
    }
    
    .bss ALIGN(0x1000) : { *(.bss*) }
}