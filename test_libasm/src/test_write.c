#include "libasm.h"

static int	test_write(int fd, char *buf, int i, int len) {
	int libc = write(fd, buf, len);
	int libc_errno = errno;
	int libasm = ft_write(fd, buf, len);
	int libasm_errno = errno;
	printf("test %i: \"%s\" in fd %d on %d bytes", i, buf, fd, len);
	if (DEBUG) {
		printf("\t%s-->\tlibasm = ret %d & errno %d\tlibc = ret %d & errno %d%s\n", (libasm == libc && libasm_errno == libc_errno ? GRN : RED), libasm, libasm_errno, libc, libc_errno, RESET);
	}
	else
		printf("\t");
	libasm == libc && libasm_errno == libc_errno ? printf("%s\t\tOK\n", BGRN) : printf("%s\t\tKO\n", BRED);
	printf("%s", RESET);
	return (libasm == libc && libasm_errno == libc_errno);
}

int main(void) {
	char	*tests[5] = {"Classic", "", "\r\t", "dfjdifioewfiewreiruiwqeuwi", NULL};
	int fd = open("test_write_basic.txt", O_CREAT | O_WRONLY | O_TRUNC, 0644);

	int res = 0;
	if (DEBUG) {
		printf("\n\t%sTESTING FT_WRITE%s\n", BWHT, RESET);
	}
	res += test_write(fd, tests[0], 0, strlen(tests[0]));
	res += test_write(fd, tests[1], 1, 100);
	res += test_write(fd, "zouzou", 2, 0);
	res += test_write(-3, tests[3], 3, strlen(tests[3]));
	res += test_write(fd, tests[0], 4, strlen(tests[0]));
	res += test_write(fd, tests[0], 5, -3);
	close(fd);
	chmod("test_write_basic.txt", S_IRUSR | S_IRGRP | S_IROTH);
	fd = open("test_write_basic.txt", O_CREAT | O_WRONLY | O_TRUNC, 0644);
	res += test_write(fd, tests[0], 6, strlen(tests[0]));
	close(fd);
	unlink("test_write_basic.txt");
	if (res == 7)
		printf("\t%sft_write ------------------- OK%s\n", BGRN, RESET);
	else
		printf("\t%sft_write ------------------- KO%s\n", BRED, RESET);
	return (0);
}