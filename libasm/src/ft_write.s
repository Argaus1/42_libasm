section .text
	global ft_write
    extern __errno_location

ft_write:
    mov rax, 1
    test rsi, rsi 
    jz .error
    syscall
    cmp rax, 0
    jl .error   ; if rax is less than 0
    ret

.error:
    mov r8, rax
    call __errno_location   ; now we have the address of errno in rax
    mov [rax], r8
    mov rax, -1
    ret

section .note.GNU-stack noalloc progbits