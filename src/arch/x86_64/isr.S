 .macro PUSHQ_CFI reg
        pushq \reg
        .cfi_adjust_cfa_offset 8
        .cfi_rel_offset \reg, 0
.endm

.macro POPQ_CFI reg
        popq \reg
        .cfi_adjust_cfa_offset -8
        .cfi_restore \reg
.endm

.macro INTERRUPT_ENTRY num, has_error_code
        .align 8
        .global int\num
        int\num:
        .if \has_error_code == 0
        pushq $0
        .endif
        pushq $\num
        jmp int_common
.endm

.macro INTERRUPT_ERROR_ENTRY num
        INTERRUPT_ENTRY \num, 1
.endm

.macro INTERRUPT_NOERROR_ENTRY num
        INTERRUPT_ENTRY \num, 0
.endm

.cfi_sections .eh_frame, .debug_frame

.global int_common
int_common:
        .cfi_startproc simple
        .cfi_signal_frame
        .cfi_def_cfa %rsp, 0
        .cfi_undefined %rip
        .cfi_undefined %rsp
        .cfi_undefined %rbp
        PUSHQ_CFI %rax
        PUSHQ_CFI %rbx
        PUSHQ_CFI %rcx
        PUSHQ_CFI %rdx
        PUSHQ_CFI %rsi
        PUSHQ_CFI %rdi
        PUSHQ_CFI %rbp
        PUSHQ_CFI %r8
        PUSHQ_CFI %r9
        PUSHQ_CFI %r10
        PUSHQ_CFI %r11
        PUSHQ_CFI %r12
        PUSHQ_CFI %r13
        PUSHQ_CFI %r14
        PUSHQ_CFI %r15
        mov 120(%rsp), %rdi
        mov %rsp, %rsi
        call interrupt_handler
        POPQ_CFI %r15
        POPQ_CFI %r14
        POPQ_CFI %r13
        POPQ_CFI %r12
        POPQ_CFI %r11
        POPQ_CFI %r10
        POPQ_CFI %r9
        POPQ_CFI %r8
        POPQ_CFI %rbp
        POPQ_CFI %rdi
        POPQ_CFI %rsi
        POPQ_CFI %rdx
        POPQ_CFI %rcx
        POPQ_CFI %rbx
        POPQ_CFI %rax
        add $8, %rsp
        iretq
        .cfi_endproc

INTERRUPT_NOERROR_ENTRY 0
INTERRUPT_NOERROR_ENTRY 1
INTERRUPT_NOERROR_ENTRY 2
INTERRUPT_NOERROR_ENTRY 3
INTERRUPT_NOERROR_ENTRY 4
INTERRUPT_NOERROR_ENTRY 5
INTERRUPT_NOERROR_ENTRY 6
INTERRUPT_NOERROR_ENTRY 7
INTERRUPT_ERROR_ENTRY 8
INTERRUPT_NOERROR_ENTRY 9
INTERRUPT_ERROR_ENTRY 10
INTERRUPT_ERROR_ENTRY 11
INTERRUPT_ERROR_ENTRY 12
INTERRUPT_ERROR_ENTRY 13
INTERRUPT_ERROR_ENTRY 14
INTERRUPT_NOERROR_ENTRY 15
INTERRUPT_NOERROR_ENTRY 16
INTERRUPT_ERROR_ENTRY 17
INTERRUPT_NOERROR_ENTRY 18
INTERRUPT_NOERROR_ENTRY 19
INTERRUPT_NOERROR_ENTRY 20

.altmacro
num = 32
.rept 256 - 32
        INTERRUPT_NOERROR_ENTRY %num
        num = num + 1
.endr
