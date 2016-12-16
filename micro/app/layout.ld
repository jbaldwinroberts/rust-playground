ENTRY(_reset);

MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 512K
  RAM : ORIGIN = 0x20000000, LENGTH = 128k
}

SECTIONS
{
  .text :
  {
    /* Vector table */
    LONG(ORIGIN(RAM) + LENGTH(RAM))
    LONG(_reset + 1);
    KEEP(*(.rodata._EXCEPTIONS));

    /* Reset handler */
    _reset = .;
    *(.text._reset)

    *(.text*)
  } > FLASH

  /DISCARD/ :
  {
    *(.ARM.exidx*)
    *(.note.gnu.build-id*)
  }
}
