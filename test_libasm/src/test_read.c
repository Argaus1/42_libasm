#include "libasm.h"

static int	test_read(int fd, char *buf, int i, int len) {
	int libc = read(fd, buf, len);
	int libc_errno = errno;
	int libasm = ft_read(fd, buf, len);
	int libasm_errno = errno;
	printf("test %i: in fd %d on %d bytes", i, fd, len);
	if (DEBUG) {
		printf("\t%s-->\tlibasm = ret %d & errno %d\tlibc = ret %d & errno %d%s\n", (libasm == libc && libasm_errno == libc_errno ? GRN : RED), libasm, libasm_errno, libc, libc_errno, RESET);
	}
	else
		printf("\t");
	libasm == libc && libasm_errno == libc_errno ? printf("%s\t\tOK\n", BGRN) : printf("%s\t\tKO\n", BRED);
	printf("%s", RESET);
	if (buf) { bzero(buf, strlen(buf)); }
	return (libasm == libc && libasm_errno == libc_errno);
}

int main(void) {
	char	buf[30];
	bzero(buf, 30);
	void	*buf_null = NULL;
	char	*buf_th = "";

	char	*file = "Makefile";
	int fd = open(file, O_RDONLY);

	int res = 0;
	if (DEBUG) {
		printf("\n\t%sTESTING FT_READ%s\n", BWHT, RESET);
	}
	res += test_read(fd, buf, 0, 3);
	res += test_read(fd, buf, 1, 0);
	res += test_read(fd, buf, 2, 2);
	res += test_read(-3, buf, 3, 2);
	res += test_read(fd, buf_null, 4, 4);
	res += test_read(fd, buf_th, 5, 2);
	close(fd);
	chmod(file, S_IRUSR | S_IRGRP | S_IROTH);
	fd = open(file, O_RDONLY);
	if (fd < 0)
		close(fd);
	res += test_read(fd, buf, 6, 2);
	res += test_read(0, buf, 7, 2);
	res += test_read(fd, buf, 8, -10);
	close(fd);
	if (res == 9)
		printf("\t%sft_read ------------------- OK%s\n", BGRN, RESET);
	else
		printf("\t%sft_read ------------------- KO%s\n", BRED, RESET);
	chmod(file, S_IRWXU);
	return (0);
}