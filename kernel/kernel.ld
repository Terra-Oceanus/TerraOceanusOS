ENTRY(_start)

PHDRS {
    r__ PT_LOAD FLAGS(4);
    r_x PT_LOAD FLAGS(5);
    rw_ PT_LOAD FLAGS(6);
}

SECTIONS {
    . = 0x100000;

    .text ALIGN(0x1000) : {
        *(.text._start)
        *(.text*)
    } : r_x

    .rodata ALIGN(0x1000) : {
        *(.rodata*)
    } : r__

    .data ALIGN(0x1000) : {
        *(.data*)
        *(.got*)
    } : rw_
    
    .bss : {
        *(.bss*)
        *(COMMON)
    } : rw_
}