# include "libasm.h"

static int	test_strlen(char *s, int i) {

	int libasm = ft_strlen(s);
	int libasm_errno = errno;
	int libc = strlen(s);
	int libc_errno = errno;
	printf("test %i: \"%s\"", i, s);
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
	char *tests[4] = {"Classic", "", "\r\t\n", "dfjdifioewfiewreiruiwqeuwi"};

	if (DEBUG) {
		printf("\n\t%sTESTING FT_STRLEN%s\n", BWHT, RESET);
	}
	int res = 0;
	for (int i = 0; i < 4; i++) {
		res += test_strlen(tests[i], i);
	}
	if (res == 4)
		printf("\t%sft_strlen ------------------- OK%s\n", BGRN, RESET);
	else
		printf("\t%sft_strlen ------------------- KO%s\n", BRED, RESET);
	return (0);
}