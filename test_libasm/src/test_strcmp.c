# include "libasm.h"

static int	test_strcmp(char *s1, char *s2, int i) {
	int libasm = ft_strcmp(s1, s2);
	int libasm_errno = errno;
	int libc = strcmp(s1, s2);
	int libc_errno = errno;
	printf("test %i: \"%s\" and \"%s\"", i, s1, s2);
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
	char *tests[5] = {"Classic", "", "", "classic", "classic"};

	if (DEBUG) {
		printf("\n\t%sTESTING FT_STRCMP%s\n", BWHT, RESET);
	}
	int res = 0;
	res += test_strcmp(tests[0], tests[1], 0);
	res += test_strcmp(tests[0], tests[3], 1);
	res += test_strcmp(tests[3], tests[4], 2);
	res += test_strcmp(tests[1], tests[2], 3);
	if (res == 4)
		printf("\t%sft_strcmp ------------------- OK%s\n", BGRN, RESET);
	else
		printf("\t%sft_strcmp ------------------- KO%s\n", BRED, RESET);
	return (0);
}