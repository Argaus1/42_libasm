section .text
    global ft_read
    ;extern __errno_location

ft_read:
    mov rax, 0
    syscall
    cmp rax, 0
    jl .error
    ret

.error:
    mov r8, rax
    ;call __errno_location
    mov [rax], r8
    mov rax, -1
    ret

section .note.GNU-stack noalloc progbits