 // This file is part of Genesis.

// Genesis is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Genesis is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with Genesis.  If not, see <http://www.gnu.org/licenses/>.

.set BOOT_START_PHYS, 0x100010 // 16 bytes for multiboot header
.set PHYS_VIRT_OFFSET, 0xffffffffC0000000

// Initial Stack
.section .boot_bss, "aw", @nobits
.align 16
boot_stack_bottom = .
.space 16 * 1024
boot_stack_top = .

// Initial Page Tables
// The first and last GB of virtual memory are mapped to the first
// GB of physical memory
.section .boot_data, "a", @progbits
.align 4096
boot_pml4:
        .quad boot_pdpt_low + 0x3
        .rept 510
        .quad 0
        .endr
        .quad boot_pdpt_high + 0x3
boot_pdpt_low:
        .quad boot_pd + 0x3
        .rept 511
        .quad 0
        .endr
boot_pdpt_high:
        .quad boot_pd + 0x3
        .rept 510
        .quad 0
        .endr
        .quad boot_pd + 0x3
boot_pd:
        index = 0
        .rept 512
          .quad (index << 21) | 0x83
          index = index + 1
        .endr

.align 8
boot_gdt:
        // NULL descriptor
        .word 0 // limit (low)
        .word 0 // base (low)
        .byte 0 // base (middle)
        .byte 0 // access
        .byte 0 // granularity
        .byte 0 // base (high)
        // Code descriptor
        .word 0 // limit (low)
        .word 0 // base (low)
        .byte 0 // base (middle)
        .byte 0x98 // access, present, not system, code - execute only
        .byte 0x20 // granularity, long mode
        .byte 0 // base (high)
        // Data descriptor
        .word 0 // limit (low)
        .word 0 // base (low)
        .byte 0 // base (middle)
        .byte 0x90 // access, present, not system
        .byte 0 // granularity
        .byte 0 // base (high)
boot_gdt_end = .

boot_gdt_desc:
        .short boot_gdt_end - boot_gdt - 1
        .quad boot_gdt

;; .align 16
;; boot_start:
;; boot_stack_top_indirect:
;;         .quad boot_stack_top - PHYS_VIRT_OFFSET
;; boot_pml4_indirect:
;;         .quad boot_pml4 - PHYS_VIRT_OFFSET
;; boot_gdt_desc32:
;;         .short boot_gdt_end - boot_gdt - 1
;;         .quad boot_gdt - PHYS_VIRT_OFFSET

.section .boot_text, "ax", @progbits
        .code32
        .align 16
// Entry point
        .global boot32
        .type boot32,@function
boot32:
        .cfi_startproc simple
        .cfi_def_cfa %esp, 0
        .cfi_undefined %eip

        cmpl $0x2BADB002, %eax // Multiboot Magic
        jne .Lno_multiboot
        mov %ebx, %edi

        // setup stack
        mov $boot_stack_top, %esp
        mov $0, %ebp

        // detect if we can cpuid
        // see if we can flip the id bit in flags
        pushf // store flags register
        pop %eax // pop it into eax
        mov %eax, %ecx // store into ecx
        xor $(1 << 21), %eax // flip the ID bit
        push %eax // store onto stack
        popf // restore flags register
        pushf // store flags register
        pop %eax // pop it into eax
        push %ecx // push ecx
        popf // restores flags to original value
        xor %eax, %ecx // should be non-zero if we flipped it
        jz .Lno_cpuid

        // we can call cpuid
        // do we have extended features cpuid?
        mov $0x80000000, %eax
        cpuid
        cmp $0x80000001, %eax
        jb .Lno_ext_features

        // we have extended features
        // do we have long mode?
        mov $0x80000001, %eax
        cpuid
        test $(1 << 29), %edx
        jz .Lno_long_mode

        // enable PAE and SSE
        mov $0x628, %eax
        mov %eax, %cr4
        // point to the pml4
        mov $boot_pml4, %eax
        mov %eax, %cr3
        // paging is setup, but not enabled
        // set long mode
        mov $0xC0000080, %ecx // EFER MSR
        mov $0x900, %eax // NXE + LME
        mov $0, %edx
        wrmsr

        mov $0x80010023, %eax // set page bit and FPU
        mov %eax, %cr0

        lgdt boot_gdt_desc

        ljmp $8, $boot64_trampoline

.Lno_multiboot:
        jmp .Lno_multiboot

.Lno_cpuid:
        jmp .Lno_cpuid

.Lno_ext_features:
        jmp .Lno_ext_features

.Lno_long_mode:
        jmp .Lno_long_mode

        .cfi_endproc
.size boot32, . - boot32

// First 64 bit code
.code64
.type boot64_trampoline,@function
boot64_trampoline:
        .cfi_startproc simple
        .cfi_def_cfa %rsp, 0
        .cfi_undefined %rip

        // jump to higher half
        lea boot64, %rax
        jmp *%rax

        .cfi_endproc
.size boot64_trampoline, . - boot64_trampoline
.text
.align 16
.type boot64,@function
boot64:
        .cfi_startproc simple
        .cfi_def_cfa %rsp, 0
        .cfi_undefined %rip
        // Reset segment selectors
        xor %ax, %ax
        mov %ax, %es
        mov %ax, %ss
        mov %ax, %ds
        mov %ax, %fs
        mov %ax, %gs

        // Call into rust
        call arch_init

.Lhang:
        cli
        hlt
        jmp .Lhang
        .cfi_endproc
.size boot64, . - boot64
