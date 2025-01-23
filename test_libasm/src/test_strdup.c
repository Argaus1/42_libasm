#include "libasm.h"

static int	test_strdup(char *s, int i) {
	char	*libc = strdup(s);
	int		libc_errno = errno;
	char	*libasm = ft_strdup(s);
	int		libasm_errno = errno;

	printf("test %i: \"%s\"", i, s);
	if (DEBUG) {
		printf("\t%s-->\tlibasm = ret %s & errno %d\tlibc = ret %s & errno %d%s\n", (!strcmp(libc, libasm) && libasm_errno == libc_errno ? GRN : RED), libasm, libasm_errno, libc, libc_errno, RESET);
	}
	else
		printf("\t");
	!strcmp(libc, libasm) && libasm_errno == libc_errno ? printf("%s\t\tOK\n", BGRN) : printf("%s\t\tKO\n", BRED);
	printf("%s", RESET);
	return (!strcmp(libc, libasm) && libasm_errno == libc_errno);
}

int main(void) {
	int	res = 0;

	if (DEBUG) {
		printf("\n\t%sTESTING FT_STRDUP%s\n", BWHT, RESET);
	}
	res += test_strdup("Normal", 0);
	res += test_strdup("", 1);
	if (res == 2)
		printf("\t%sft_strdup ------------------- OK%s\n", BGRN, RESET);
	else
		printf("\t%sft_strdup ------------------- KO%s\n", BRED, RESET);
	return (0);
}