NAME := libasm.a

SRC_DIR := src

SRC_FILES := ft_strlen.s

SRC := $(SRC_FILES:%.s=$(SRC_DIR)/%.s)

BUILD_DIR := .build

OBJ := $(SRC:$(SRC_DIR)/%.s=$(BUILD_DIR)/%.o)

#DEPS := $(OBJ:.o=.d)

NASM := nasm
NASMFLAGS := -f elf64

#CFLAGS := -Wall -Wextra -Werror
#PREPFLAGS := -MMD -MP -I headers
AR := ar
ARFLAGS := -rcs

DIR_DUP = mkdir -p $(@D)

all: $(NAME)

$(NAME): $(OBJ)
	@$(AR) $(ARFLAGS) $(NAME) $(OBJ)
	@cp $(NAME) test_libasm/
	@echo "libasm ready and copied"

$(BUILD_DIR)/%.o: $(SRC_DIR)/%.s
	@$(DIR_DUP)
	@$(NASM) $(NASMFLAGS) -o $@ $<

#-include $(DEPS)

clean:
	@rm -rf $(BUILD_DIR)
	@echo "libasm .o deleted"

fclean: clean
	@rm -rf $(NAME)
	@rm test_libasm/$(NAME)
	@echo "libasm bin deleted"
	@echo "libasm bin in rust project deleted"

re: fclean all

subject:
	@xdg-open 'https://cdn.intra.42.fr/pdf/pdf/148904/en.subject.pdf'

.PHONY: all clean fclean re

