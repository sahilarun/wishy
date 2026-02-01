[BITS 16]
[ORG 0x7E00]

stage2_start:
    mov [boot_drive], dl
    
    ; Print "A20"
    mov si, msg_a20
    call print_string
    call enable_a20
    
    ; Print "Loading"
    mov si, msg_loading
    call print_string
    call load_kernel
    
    ; Print "GDT"
    mov si, msg_gdt
    call print_string
    call setup_gdt
    
    ; Print "Protected"
    mov si, msg_protected
    call print_string
    call enter_protected_mode

enable_a20:
    in al, 0x92
    or al, 2
    out 0x92, al
    ret



load_kernel:
    ; Load kernel starting at sector 18 (0x12)
    ; Kernel is at physical sector 18 on disk
    mov ax, 0x1000
    mov es, ax
    xor bx, bx
    
    mov dl, [boot_drive]
    mov ah, 0x02             ; read
    mov al, 127              ; sectors (max for one call)
    mov ch, 0                ; cylinder 0
    mov cl, 19               ; sector 19 (sector 18 in 1-based = 19)
    mov dh, 0                ; head 0
    int 0x13
    jc kernel_error
    
    mov si, msg_kernel_ok
    call print_string
    ret

boot_drive: db 0

kernel_error:
    mov si, msg_kernel_err
    call print_string
    hlt

setup_gdt:
    cli
    lgdt [gdt_descriptor]
    ret

enter_protected_mode:
    mov eax, cr0
    or eax, 1
    mov cr0, eax
    jmp 0x08:protected_mode_start

[BITS 32]
protected_mode_start:
    mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax
    mov esp, 0x90000

    call 0x10000
    
    cli
    hlt

[BITS 16]
print_string:
    pusha
    mov ah, 0x0E
.loop:
    lodsb
    test al, al
    jz .done
    int 0x10
    jmp .loop
.done:
    popa
    ret

gdt_start:
    dq 0

gdt_code:
    dw 0xFFFF
    dw 0
    db 0
    db 10011010b
    db 11001111b
    db 0

gdt_data:
    dw 0xFFFF
    dw 0
    db 0
    db 10010010b
    db 11001111b
    db 0

gdt_end:

gdt_descriptor:
    dw gdt_end - gdt_start - 1
    dd gdt_start

msg_stage2: db 'Stage2 loaded', 13, 10, 0
msg_a20: db 'A20...', 0
msg_loading: db 'Loading...', 0
msg_gdt: db 'GDT...', 0
msg_protected: db 'Protected...', 0
msg_kernel_ok: db 'OK', 13, 10, 0
msg_kernel_err: db 'Kernel load failed', 13, 10, 0

times 8192-($-$$) db 0
