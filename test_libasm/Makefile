SRC_DIR := src/
INCLUDES := -I includes
ALL = strcmp strlen write read
ALL	:= $(addprefix test_, $(addsuffix .c, $(ALL)))
ALL	:= $(addprefix $(SRC_DIR), $(ALL))

OBJ = $(ALL:.c=.o)
DEPS := $(OBJ:.o=.d)

CC := clang
CFLAGS := -Wall -Wextra -Werror -Wno-unused-variable -Wno-unused-parameter -g
ifdef DEBUG
    CFLAGS += -DDEBUG=$(DEBUG)
endif

PREPFLAGS := -MMD -MP $(INCLUDES)
#DIR_DUP = mkdir -p $(@D)

all: $(ALL)
	@make -C ../libasm
	@for src in $(ALL); do \
		exe=$$(basename $$src .c); \
		$(CC) $(CFLAGS) $(PREPFLAGS) -o $$exe $$src ../libasm/libasm.a && ./$$exe; \
		rm -f $$exe; \
		rm -f *.d; \
	done

%.o: %.c
	@$(DIR_DUP)
	$(CC) $(CFLAGS) $(PREPFLAGS) -c $< -o $@

oclean:
	rm -rf $(OBJ) $(DIR_DEP)

clean: oclean
	make clean -C ../libasm
	rm -f *.txt

fclean: clean
	make fclean -C ../libasm

re: fclean all

.PHONY: all clean oclean fclean re