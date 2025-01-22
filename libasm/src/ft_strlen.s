section .text
		global ft_strlen

ft_strlen:
		xor rax, rax

.loop:
		cmp BYTE [rdi + rax], 0
		je .out
		inc rax
		jmp .loop

.out:
		ret

section .note.GNU-stack noalloc progbits