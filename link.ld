ENTRY(_entry)
OUTPUT_FORMAT(binary)

SECTIONS
{
    .text 0x0 :
    {
        *(.text._entry)
        *(.data*)
        *(.rodata*)
        *(.text*)

    }

    .bss :
    {
        *(COMMON)
        *(.bss*)
    }

    _backdoor_end = .;

    /DISCARD/ : { *(.dynsym) *(.gnu.hash) *(.dynamic) *(.hash) *(.eh_frame_hdr) *(.dynstr) *(.eh_frame) *(.rela.dyn) }
}
