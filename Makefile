all:
	@make -C libasm -s
	@bash scripts/test_build.sh

clean:
	@make -C libasm clean 
	@bash scripts/test_clean.sh

fclean: clean
	@make -C libasm fclean

re: fclean all

subject:
	@xdg-open 'https://cdn.intra.42.fr/pdf/pdf/133629/en.subject.pdf'

syscall:
	@xdg-open 'https://github.com/torvalds/linux/blob/master/arch/x86/entry/syscalls/syscall_64.tbl'
	@xdg-open 'https://upload.wikimedia.org/wikipedia/commons/thumb/4/45/Linux_kernel_System_Call_Interface_and_glibc.svg/1280px-Linux_kernel_System_Call_Interface_and_glibc.svg.png'

.PHONY: all clean fclean re syscall subject

