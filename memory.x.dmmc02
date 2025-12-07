/* Compatible with STM32H723xG. (RM0468) */
/* - D1     domain connect to a 64-bit wide bus -> align to 8 bytes. */
/* - D2 D3  domain connect to a 32-bit wide bus -> align to 4 bytes. */

MEMORY
{
    /* STM32H723xG             */
    FLASH : ORIGIN = 0x08000000, LENGTH = 1M

    /* Instruction TCM (D1) */
    /* can be modified via the TCM_AXI_SHARED[1,0] register */
    ITCM  : ORIGIN = 0x00000000, LENGTH = 64K + 0K

    /* Data TCM (D1) */
    DTCM  : ORIGIN = 0x20000000, LENGTH = 128K

    /* AXI SRAM (D1) */
    /* can be modified via the TCM_AXI_SHARED[1,0] register */
    AXISRAM : ORIGIN = 0x24000000, LENGTH = 128K + 192K

    /* AHB SRAM1 (D2) */
    SRAM1 : ORIGIN = 0x30000000, LENGTH = 16K
    /* AHB SRAM2 (D2) */
    SRAM2 : ORIGIN = 0x30004000, LENGTH = 16K
    /* AHB SRAM4 (D3) */
    SRAM4 : ORIGIN = 0x38000000, LENGTH = 16K

    /* Backup SRAM (D3) */
    BSRAM : ORIGIN = 0x38800000, LENGTH = 4K
}

REGION_ALIAS(RAM, DTCM);

SECTIONS
{
    .itcm : ALIGN(8)
    {
        __siitcm = LOADADDR(.itcm);
        . = ALIGN(8);
        __sitcm = .;
        *(.itcm .itcm.*);
        . = ALIGN(8);
        __eitcm = .;
    } > ITCM AT > FLASH

    .axisram : ALIGN(8)
    {
        __siaxisram = LOADADDR(.axisram);
        . = ALIGN(8);
        __saxisram = .;
        *(.axisram .axisram.*);
        . = ALIGN(8);
        __eaxisram = .;
    } > AXISRAM AT > FLASH


} INSERT AFTER .rodata;

SECTIONS
{
    .sram1 (NOLOAD) : ALIGN(4)
    {
        . = ALIGN(4);
        *(.sram1 .sram1.*);
        . = ALIGN(4);
    } > SRAM1

    .sram2 (NOLOAD) : ALIGN(4)
    {
        . = ALIGN(4);
        *(.sram2 .sram2.*);
        . = ALIGN(4);
    } > SRAM2

    .sram4 (NOLOAD) : ALIGN(4)
    {
        . = ALIGN(4);
        *(.sram4 .sram4.*);
        . = ALIGN(4);
    } > SRAM4

    .bsram (NOLOAD) : ALIGN(4)
    {
        . = ALIGN(4);
        *(.bsram .bsram.*);
        . = ALIGN(4);
    } > BSRAM
}
