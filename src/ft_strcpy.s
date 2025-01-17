		section .text
		global ft_strcpy

ft_strcpy:
		xor rax, rax

.loop:
		cmp byte [rsi + rax], 0
		je .out
		mov bl, byte [rsi + rax]
		mov byte [rdi + rax], bl
		inc rax
		jmp .loop

.out:
		mov byte [rdi + rax], 0
		mov rax, rdi
		ret 
