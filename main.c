#include <stdio.h>
#include <string.h>
#include <unistd.h>
#include <stdlib.h>
#include <errno.h>
#include <fcntl.h>

//COLOR
#define RED "\033[0;31m"
#define BLUE "\033[0;34m"
#define GREEN "\033[0;32m"
#define RESET "\033[0m"

//int ft_strlen(char *str);

//char *ft_strcpy(char *dest, char *src);

//int ft_strcmp(char *s1, char *s2);

ssize_t ft_write(int fd, const void *buf, size_t count);

//ssize_t ft_read(int fd, void *buf, size_t count);

//char *ft_strdup(const char *s);

int main(void)
{
	int fd = open("test", O_RDWR);
    printf("%zd\n", ft_write(1, NULL, 7));
    printf("Error number: %d\n", errno);
    printf("%zd\n", write(1, NULL, 7));
    printf("Error number: %d\n", errno);
	close(fd);
    return (0);
}
