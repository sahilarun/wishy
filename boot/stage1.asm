[BITS 16]
[ORG 0x7C00]

start:
    cli
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov sp, 0x7C00
    sti

    mov [boot_drive], dl

    mov si, msg_loading
    call print_string

    mov ah, 0x02
    mov al, 16
    mov ch, 0
    mov cl, 2
    mov dh, 0
    mov dl, [boot_drive]
    mov bx, 0x7E00
    int 0x13
    jc disk_error

    ; Pass boot drive to stage2 in DL (not memory location)
    mov dl, [boot_drive]
    jmp 0x7E00

disk_error:
    mov si, msg_error
    call print_string
    hlt

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

boot_drive: db 0
msg_loading: db 'Loading stage2...', 13, 10, 0
msg_error: db 'Disk read error', 13, 10, 0

times 510-($-$$) db 0
dw 0xAA55
