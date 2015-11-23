/*
  This file is part of Genesis.

  Genesis is free software: you can redistribute it and/or modify
  it under the terms of the GNU Affero General Public License as published by
  the Free Software Foundation, either version 3 of the License, or
  (at your option) any later version.

  Genesis is distributed in the hope that it will be useful,
  but WITHOUT ANY WARRANTY; without even the implied warranty of
  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
  GNU Affero General Public License for more details.

  You should have received a copy of the GNU Affero General Public License
  along with Genesis.  If not, see <http://www.gnu.org/licenses/>.
*/

ENTRY(boot32)
OUTPUT_FORMAT(elf64-x86-64)

mb_magic = 0x1badb002;
mb_flags = 0x3;
mb_checksum = -(mb_magic + mb_flags);

KERNEL_LMA = 1 << 20;
KERNEL_PHYS_MAP = 0xFFFFFFFFC0000000;

SECTIONS {
         /* Lower half */
         . = KERNEL_LMA;

         .mb_header : {
           LONG(mb_magic)
           LONG(mb_flags)
           LONG(mb_checksum)
         }

         .boot_data : {
           *(.boot_data*)
         }

         .boot_text : {
           *(.boot_text*)
         }

         /* Higher half */
         . = . + KERNEL_PHYS_MAP;

         .text ALIGN(4K) : AT(ADDR(.text) - KERNEL_PHYS_MAP) {
           btext = .;
           *(.text*)
         }

         .rodata ALIGN(4K) : AT(ADDR(.rodata) - KERNEL_PHYS_MAP) {
           etext = .;
           bro = .;
           *(.rodata*)
         }

         .data ALIGN(4K) : AT(ADDR(.data) - KERNEL_PHYS_MAP) {
           ero = .;
           bdata = .;
           *(.data*)
         }

         .bss ALIGN(4K) : AT(ADDR(.bss) - KERNEL_PHYS_MAP) {
           *(COMMON)
           *(.bss*)
           *(.gnu.linkonce.b*)
         }

         kend = ALIGN(4K);
         edata = .;

         .debug_abbrev 0 : { KEEP(*(.debug_abbrev)) }
         .debug_arranges 0 : { KEEP(*(.debug_arranges)) }
         .debug_frame 0 : { KEEP(*(.debug_frame)) }
         .debug_info 0 : { KEEP(*(.debug_info)) }
         .debug_line 0 : { KEEP(*(.debug_line)) }
         .debug_loc 0 : { KEEP(*(.debug_loc)) }
         .debug_macinfo 0 : { KEEP(*(.debug_macinfo)) }
         .debug_pubnames 0 : { KEEP(*(.debug_pubnames)) }
         .debug_pubtypes 0 : { KEEP(*(.debug_pubtypes)) }
         .debug_ranges 0 : { KEEP(*(.debug_ranges)) }
         .debug_str 0 : { KEEP(*(.debug_str))}

         /DISCARD/ : {
           *(.note*)
           *(.eh_frame*)
         }
}
