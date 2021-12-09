BITS 32

section .text
    global _exit
    global _start_window_draw
    global _end_window_draw
    global _define_window
    global _display_message
    global _wait_event
    global _pressed_key

_exit:
    mov eax, -1
    int 0x40

_start_window_draw:
    mov eax, 0xc
    mov ebx, 1
    int 0x40
    ret

_end_window_draw:
    mov eax, 0xc
    mov ebx, 2
    int 0x40
    ret

_define_window:
    push edi
    push ebx
    xor eax, eax
    mov ebx, dword [esp + 0x14]
    mov ecx, dword [esp + 0x18]
    mov edx, dword [esp + 0x1c]
    mov edi, dword [esp + 0x20]
    mov ebx, 2
    int 0x40
    pop ebx
    pop edi
    ret

_display_message:
    push edi
    push esi
    push ebx
    mov eax, 4
    mov ebx, dword [esp + 0x14]
    mov ecx, dword [esp + 0x18]
    mov edx, dword [esp + 0x1c]
    mov esi, dword [esp + 0x20]
    mov edi, dword [esp + 0x24]
    mov ebx, 2
    int 0x40
    pop ebx
    pop esi
    pop edi
    ret

_wait_event:
    mov eax, 0xa
    int 0x40
    ret

_pressed_key:
    mov eax, 2
    int 0x40
    ret
