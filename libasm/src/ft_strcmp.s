	section .text
		global ft_strcmp

ft_strcmp:
		xor rax, rax

.loop:
		mov al, byte [rdi]
		mov bl, byte [rsi]
		cmp al, bl
		jne .diff
		test al, al
		je .out
		test bl, bl
		je .out
		inc rdi
		inc rsi
		jmp .loop

.diff:
		movzx rax, al
		movzx rbx, bl
		sub rax, rbx

.out:
		ret

section .note.GNU-stack noalloc progbits
