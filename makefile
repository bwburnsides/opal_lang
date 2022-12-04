SRC_DIR := src
BIN_DIR := bin

EXE := $(BIN_DIR)/opal
SRC := $(wildcard $(SRC_DIR)/*.c)
OBJ := $(SRC:$(SRC_DIR)/%.c=$(BIN_DIR)/%.o)

.PHONY: all clean

all: $(EXE)

$(EXE): $(OBJ) | $(BIN_DIR)
	gcc $^ -o $@ -g

$(BIN_DIR)/%.o: $(SRC_DIR)/%.c | $(BIN_DIR)
	gcc -c $< -o $@ -g -Wmissing-field-initializers

$(BIN_DIR):
	mkdir $@

clean:
	rmdir /s /q bin

-include $(OBJ:.o=.d)