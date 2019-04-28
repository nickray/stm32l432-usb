/* Linker script for building examples for the STM32L432KC */
MEMORY
{
  FLASH (rx) : ORIGIN = 0x08000000, LENGTH = 256K
  RAM  (rwx) : ORIGIN = 0x20000000, LENGTH =  48K
  SRAM2 (rw) : ORIGIN = 0x10000000, LENGTH =  16K
}
