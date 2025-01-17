NAME := libasm.a

SRC_DIR := src

SRC_FILES := ft_strlen.s

SRC := $(SRC_FILES:%.s=$(SRC_DIR)/%.s)

BUILD_DIR := .build

OBJ := $(SRC:$(SRC_DIR)/%.s=$(BUILD_DIR)/%.o)


NASM := nasm
NASMFLAGS := -f elf64

AR := ar
ARFLAGS := -rcs

DIR_DUP = mkdir -p $(@D)

TEST_DIR := test_libasm

all: $(NAME)

$(NAME): $(OBJ)
	@$(AR) $(ARFLAGS) $(NAME) $(OBJ)
	@echo "libasm ready"

$(BUILD_DIR)/%.o: $(SRC_DIR)/%.s
	@$(DIR_DUP)
	@$(NASM) $(NASMFLAGS) -o $@ $<

test:
	@cp $(NAME) $(TEST_DIR)/
	@echo "libasm.a copied into $(TEST_DIR)"
	@bash scripts/test_build.sh
	@echo "test exec ready"


clean:
	@rm -rf $(BUILD_DIR)
	@echo "libasm .o deleted"

fclean: clean
	@rm -rf $(NAME)
	@echo "libasm bin deleted"

test_clean:
	@bash scripts/test_clean.sh
	@echo $(TEST_DIR) cleaned

re: fclean all

test_re: test_clean test

subject:
	@xdg-open 'https://cdn.intra.42.fr/pdf/pdf/148904/en.subject.pdf'

.PHONY: all clean fclean re test test_clean test_re

