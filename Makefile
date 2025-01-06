NAME := libasm.a

SRC_DIR := src

SRC_FILES := ft_strlen.c

SRC := $(SRC_FILES:%.c=$(SRC_DIR)/%.c)

BUILD_DIR := .build

OBJ := $(SRC:$(SRC_DIR)/%.c=$(BUILD_DIR)/%.o)

DEPS := $(OBJ:.o=.d)

CC := cc

CFLAGS := -Wall -Wextra -Werror
PREPFLAGS := -MMD -MP -I headers
AR := ar
ARFLAGS := -r -c -s

DIR_DUP = mkdir -p $(@D)

all: $(NAME)

$(NAME): $(OBJ)
	@$(AR) $(ARFLAGS) $(NAME) $(OBJ)
	@echo "libft ready"

$(BUILD_DIR)/%.o: $(SRC_DIR)/%.c
	@$(DIR_DUP)
	@$(CC) $(CFLAGS) $(PREPFLAGS) -c -o $@ $<

-include $(DEPS)

clean:
	@rm -rf $(BUILD_DIR)
	@echo "libft .o deleted"

fclean: clean
	@rm -rf $(NAME)
	@echo "libft bin deleted"

re: fclean all

.PHONY: all clean fclean re
