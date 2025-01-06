section .data
	msg db "Hello, World", 0xA ; msg = global varible for a string, db = define and allocate bytes
	len equ $ - msg

section .text
	global _start

_start:
	mov x8, 64
	mov x0, 1
	ldr x1, =msg

