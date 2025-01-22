#include <stdio.h>
#include <string.h>
#include <unistd.h>
#include <stdlib.h>

//COLOR
#define RED "\033[0;31m"
#define BLUE "\033[0;34m"
#define GREEN "\033[0;32m"
#define RESET "\033[0m"

int ft_strlen(char *str);

char *ft_strcpy(char *dest, char *src);

int ft_strcmp(char *s1, char *s2);

ssize_t ft_write(int fd, const void *buf, size_t count);

ssize_t ft_read(int fd, void *buf, size_t count);

char *ft_strdup(const char *s);

int main(void)
{
    char str[] = "Hello, World";
    char str2[] = "Hello, World!";
    char dest[100];
    char dest2[100];
    
    printf(RED "----------Strlen----------\n");
    printf(BLUE "OFFICIAL -> Length of string: %ld\n", strlen(str));
    printf(GREEN "NO OFFICIAL -> Length of string: %d\n\n", ft_strlen(str));

    strcpy(dest, str);
    ft_strcpy(dest2, str2);
    printf(RED "----------Strcpy----------\n");
    printf(BLUE "OFFICIAL -> Copied string: %s\n", dest);
    printf(GREEN "NO OFFICIAL -> Copied string: %s\n\n", dest2);

    printf(RED "----------Strcmp----------\n");
    printf(BLUE "OFFICIAL -> Compare strings: %d\n", strcmp(str, str2));
    printf(GREEN "NO OFFICIAL -> Compare strings: %d\n\n", ft_strcmp(str, str2));

    printf(RED "----------Write----------\n");
    write(1, BLUE "OFFICIAL -> Hello, World!\n" RESET, strlen(BLUE "OFFICIAL -> Hello, World!\n" RESET));
    ft_write(1, GREEN "NO OFFICIAL -> Hello, World!\n\n" RESET, strlen(GREEN "NO OFFICIAL -> Hello, World!\n\n" RESET));

    char buf[100];
    printf(RED "----------Read----------\n");
    read(0, buf, 100);
    printf(BLUE "OFFICIAL -> %s\n", buf);
    ft_read(0, buf, 100);
    printf(GREEN "NO OFFICIAL -> %s\n\n", buf);

    printf(RED "----------Strdup----------\n");
    char *str3 = strdup(str);
    char *str4 = ft_strdup(str2);
    printf(BLUE "OFFICIAL -> %s\n", str3);
    printf(GREEN "NO OFFICIAL -> %s\n\n", str4);

    return (0);
}
