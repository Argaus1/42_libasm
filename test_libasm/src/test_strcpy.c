#include "libasm.h"

int	test_strcpy(char *dest, char *dest_2, const char *src, int i) {
	char	*libc = strcpy(dest, src);
	int		libc_errno = errno;
	char	*libasm = ft_strcpy(dest_2, src);
	int		libasm_errno = errno;

	printf("test %i: \"%s\" and \"%s\"", i, dest, src);
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
	char	dest[30];
	char	dest_2[30];

	bzero(dest, 30);
	bzero(dest_2, 30);
	if (DEBUG) {
		printf("\n\t%sTESTING FT_STRCPY%s\n", BWHT, RESET);
	}
	res += test_strcpy(dest, dest_2, "Normal", 0);
	res += test_strcpy(dest, dest_2, "", 1);
	// res += test_strcpy(dest, dest_2, "test", 1);
	// res += test_strcpy(dest, dest_2, "", 1);
	if (res == 2)
		printf("\t%sft_strcpy ------------------- OK%s\n", BGRN, RESET);
	else
		printf("\t%sft_strcpy ------------------- KO%s\n", BRED, RESET);
	return (0);
}