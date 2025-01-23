#ifndef LIBASM_H
# define LIBASM_H

# include <string.h>
# include <strings.h>
# include <stdio.h>
# include <unistd.h>
# include <stdlib.h>
# include <errno.h>
# include <fcntl.h>
# include <sys/stat.h>

# ifndef DEBUG
#  define DEBUG 1
# endif

# define BLK "\e[0;30m"
# define RED "\e[0;31m"
# define GRN "\e[0;32m"
# define YEL "\e[0;33m"
# define BLU "\e[0;34m"
# define MAG "\e[0;35m"
# define CYN "\e[0;36m"
# define WHT "\e[0;37m"

# define BWHT "\e[1;37m"
# define BRED "\e[1;31m"
# define BGRN "\e[1;32m"

# define RESET "\e[0m"

size_t	ft_strlen(const char *s);
int		ft_strcmp(const char *s1, const char *s2);
ssize_t	ft_write(int fd, const void *buf, size_t count);   
ssize_t	ft_read(int fd, const void *buf, size_t count);   
char	*ft_strdup(const char *s);
char	*ft_strcpy(char *dest, const char *src);

#endif