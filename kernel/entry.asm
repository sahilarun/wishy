[BITS 32]
[EXTERN kmain]
[GLOBAL _start]

section .text
_start:
    mov esp, kernel_stack_top
    
    push ebx
    push eax
    
    call kmain
    
    cli
.hang:
    hlt
    jmp .hang

section .bss
align 16
kernel_stack_bottom:
    resb 16384
kernel_stack_top:
