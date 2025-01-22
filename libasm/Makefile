NAME := libasm.a

SRC_DIR := src

SRC_FILES := ft_strlen.s ft_strcmp.s ft_strcpy.s ft_write.s ft_read.s ft_strdup.s

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

clean:
	@rm -rf $(BUILD_DIR)
	@echo "libasm .o deleted"

fclean: clean
	@rm -rf $(NAME)
	@echo "libasm bin deleted"

re: fclean all

subject:
	@xdg-open 'https://cdn.intra.42.fr/pdf/pdf/133629/en.subject.pdf'

syscall:
	@xdg-open 'https://github.com/torvalds/linux/blob/master/arch/x86/entry/syscalls/syscall_64.tbl'
	@xdg-open 'https://upload.wikimedia.org/wikipedia/commons/thumb/4/45/Linux_kernel_System_Call_Interface_and_glibc.svg/1280px-Linux_kernel_System_Call_Interface_and_glibc.svg.png'

.PHONY: all clean fclean re syscall subject

