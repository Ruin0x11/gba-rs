ENTRY(_start)

MEMORY {
    rom      : ORIGIN = 0x08000000, LENGTH = 32M
    iwram    : ORIGIN = 0x03000000, LENGTH = 32K
    ewram    : ORIGIN = 0x02000000, LENGTH = 256K
}

__text_start    = ORIGIN(rom);
__eheap_end     = ORIGIN(ewram) + LENGTH(ewram);
__iwram_start   = ORIGIN(iwram);
__iwram_top     = ORIGIN(iwram) + LENGTH(iwram);
__ewram_start   = ORIGIN(ewram);
__sp_irq        = __iwram_top - 0x060;
__sp_usr        = __sp_irq - 0x0a0;
__irq_flags     = 0x03007ff8;

SECTIONS
{
/******************************* TEXT *******************************/

    . = __text_start;
    .init :
    {
        KEEP (*(.init))
        . = ALIGN(4);
    } >rom =0xFF

    .text :
    {
        *(EXCLUDE_FILE (*.iwram*) .text)
        . = ALIGN(4);
    } >rom =0xFF

    __text_end = .;

    .rodata :
    {
        *(.rodata)
        . = ALIGN(4);
    } >rom =0xFF
    .ARM.extab   : { *(.ARM.extab* .gnu.linkonce.armextab.*) } >rom
    __exidx_start = .;
    .ARM.exidx   : { *(.ARM.exidx* .gnu.linkonce.armexidx.*) } >rom
    __exidx_end = .;

/****************************** IWRAM *******************************/

    __iwram_lma = .;

    .iwram /* __iwram_start */ : AT(__iwram_lma)
    {
        __iwram_start__ = ABSOLUTE(.) ;
        *(.iwram)
        . = ALIGN(4);
        __iwram_end__ = ABSOLUTE(.) ;
    } >iwram = 0xff

    .got :
    {
        *(.got.plt)
        *(.got)
    } >iwram = 0

    .bss ALIGN(4) (NOLOAD) :
    {
        __bss_start = ABSOLUTE(.);
        __bss_start__ = ABSOLUTE(.);
        *(.bss*)
        *(COMMON)
        . = ALIGN(4);
        __bss_end__ = ABSOLUTE(.);

    } /*AT*/>iwram

/******************************* DATA *******************************/

    __data_lma = __iwram_lma + SIZEOF(.iwram);

    .data ALIGN(4) : AT(__data_lma)
    {
        __data_start__ = ABSOLUTE(.);
        *(.data)
        *(.data.*)
        . = ALIGN(4);
    } >iwram = 0xff

    __data_end__  =  ABSOLUTE(.);

/*
    __iwram_overlay_lma = __jcr_lma + SIZEOF(.jcr);

    __iwram_overlay_start = . ;

    OVERLAY ALIGN(4) : NOCROSSREFS AT (__iwram_overlay_lma)
    {
        .iwram0 { *(.iwram0) . = ALIGN(4); }
        .iwram1 { *(.iwram1) . = ALIGN(4); }
        .iwram2 { *(.iwram2) . = ALIGN(4); }
        .iwram3 { *(.iwram3) . = ALIGN(4); }
        .iwram4 { *(.iwram4) . = ALIGN(4); }
        .iwram5 { *(.iwram5) . = ALIGN(4); }
        .iwram6 { *(.iwram6) . = ALIGN(4); }
        .iwram7 { *(.iwram7) . = ALIGN(4); }
        .iwram8 { *(.iwram8) . = ALIGN(4); }
        .iwram9 { *(.iwram9) . = ALIGN(4); }
    } >iwram =0xff

    __iwram_overlay_end = . ;
    __ewram_lma = __iwram_overlay_lma + (__iwram_overlay_end - __iwram_overlay_start) ;
*/

/****************************** EWRAM *******************************/

    __iheap_start = . ;

    __ewram_lma = __data_lma + SIZEOF(.data);

    .ewram /* __ewram_start */ : AT (__ewram_lma)
    {
        *(.ewram)
        . = ALIGN(4);
        __ewram_end = ABSOLUTE(.);
    }>ewram = 0xff

    .sbss ALIGN(4)(NOLOAD):
    {
        __sbss_start__ = ABSOLUTE(.);
         *(.sbss)
         . = ALIGN(4);
        __sbss_end__  = ABSOLUTE(.);
        __end__ = ABSOLUTE(.);
        __eheap_start = ABSOLUTE(.);
    } >ewram

/******************************* PAD ********************************/

    __pad_lma = __ewram_lma + SIZEOF(.ewram);

    .pad ALIGN(4) : AT (__pad_lma)
    {
        LONG(0x52416b64)
        LONG(0x4d)
        . = ALIGN(4);
    } = 0xff

/******************************* END ********************************/

    __rom_end__ = __pad_lma + SIZEOF(.pad);
}

ASSERT(SIZEOF(.init) > 0, ".init section was not linked")
