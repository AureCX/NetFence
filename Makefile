NAME = netfence
BIN_NAME = NetFence

ifeq ($(OS),Windows_NT)
    EXE = .exe
else
    EXE =
endif

all: $(NAME)

$(NAME):
	cargo build
ifeq ($(OS),Windows_NT)
	cmd /C "move /Y target\debug\$(NAME)$(EXE) $(BIN_NAME)$(EXE)"
else
	mv ./target/debug/$(NAME)$(EXE) ./$(BIN_NAME)$(EXE)
endif

clean:
	cargo clean

fclean: clean
ifeq ($(OS),Windows_NT)
	cmd /C "if exist $(BIN_NAME)$(EXE) del /Q $(BIN_NAME)$(EXE)"
else
	rm -f ./$(BIN_NAME)$(EXE)
endif

re: fclean all

.PHONY: all $(NAME) clean fclean re