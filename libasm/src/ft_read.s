section .text
	global ft_read
    extern __errno_location

ft_read:
    mov rax, 0
    test rsi, rsi
    jz .error_null
    syscall
    cmp rax, 0
    jl .error   ; if rax is less than 0
    ret

.error:
    mov r8, rax
    neg r8
    call __errno_location   ; now we have the address of errno in rax
    mov [rax], r8
    mov rax, -1
    ret

.error_null:
    call __errno_location
    mov dword [rax], 14     ; EFAULT (bad address) or EINVAL (alignment) 22 ???
    mov rax, -1
    ret

section .note.GNU-stack noalloc progbits