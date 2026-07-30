OUTPUT_FORMAT(elf64-x86-64)
ENTRY(kmain)

PHDRS
{
    text   PT_LOAD;
    rodata PT_LOAD;
    data   PT_LOAD;
}

SECTIONS
{
    _kernel_start = .;

    . = 0xffffffff80000000;

    .text : {
        *(.text .text.*)
    } :text

    . = ALIGN(CONSTANT(MAXPAGESIZE));

    .rodata : {
        *(.rodata .rodata.*)
    } :rodata

    . = ALIGN(CONSTANT(MAXPAGESIZE));

    .data : {
        *(.data .data.*)

        /*
         * Limine сканирует эти секции и находит запросы ядра.
         */
        KEEP(*(.requests_start_marker))
        KEEP(*(.requests))
        KEEP(*(.requests_end_marker))
    } :data

    .bss : {
        *(.bss .bss.*)
        *(COMMON)
    } :data

    /*
     * Эти секции раннему ядру пока не требуются.
     */
    /DISCARD/ : {
        *(.eh_frame*)
        *(.note .note.*)
    }

    _kernel_end = .;
}
