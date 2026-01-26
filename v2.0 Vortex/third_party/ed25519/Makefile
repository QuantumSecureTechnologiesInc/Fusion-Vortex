# Makefile for Ed25519 Signature Library
# Builds production-ready Ed25519 implementation
# Supports MSVC, GCC, Clang

CC ?= gcc
CFLAGS = -O2 -Wall -Wextra -fPIC
LDFLAGS =

# Source files
SRCS = ed25519_field.c ed25519_sha512.c ed25519_scalar.c ed25519_group.c ed25519_api_complete.c
OBJS = $(SRCS:.c=.o)
HEADERS = ed25519.h ed25519_core.h ed25519_field.h

# Targets
LIB = libed25519.a
SHARED_LIB = libed25519.so
TEST_PROG = test_ed25519
TEST_SRC = test_ed25519.c

# Platform detection
UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S),Linux)
    PLATFORM = linux
    SHARED_LIB = libed25519.so
endif
ifeq ($(UNAME_S),Darwin)
    PLATFORM = macos
    SHARED_LIB = libed25519.dylib
    LDFLAGS += -dynamiclib
endif
ifeq ($(OS),Windows_NT)
    PLATFORM = windows
    SHARED_LIB = ed25519.dll
endif

# Default target
all: $(LIB) $(TEST_PROG)

# Build static library
$(LIB): $(OBJS)
	@echo "[AR] Building $@..."
	ar rcs $@ $(OBJS)
	@echo "[OK] Static library built: $@"

# Build shared library
shared: $(OBJS)
	@echo "[LD] Building $@..."
	$(CC) -shared $(LDFLAGS) -o $(SHARED_LIB) $(OBJS)
	@echo "[OK] Shared library built: $(SHARED_LIB)"

# Compile object files
%.o: %.c $(HEADERS)
	@echo "[CC] Compiling $<..."
	$(CC) $(CFLAGS) -c $< -o $@

# Build test program
$(TEST_PROG): $(OBJS) $(TEST_SRC)
	@echo "[CC] Building test program..."
	$(CC) $(CFLAGS) -o $(TEST_PROG) $(OBJS) $(TEST_SRC)
	@echo "[OK] Test program built: $(TEST_PROG)"

# Run tests
test: $(TEST_PROG)
	@echo "[RUN] Running tests..."
	./$(TEST_PROG)

# Clean build artifacts
clean:
	@echo "[CLEAN] Removing build artifacts..."
	rm -f $(OBJS) $(LIB) $(SHARED_LIB) $(TEST_PROG) *.o

# Install (static library)
install: $(LIB)
	@echo "[INSTALL] Installing library..."
	mkdir -p /usr/local/lib /usr/local/include
	install -m 644 $(LIB) /usr/local/lib/
	install -m 644 ed25519.h /usr/local/include/
	install -m 644 ed25519_core.h /usr/local/include/
	install -m 644 ed25519_field.h /usr/local/include/
	@echo "[OK] Installation complete"

# Uninstall
uninstall:
	@echo "[UNINSTALL] Removing installed files..."
	rm -f /usr/local/lib/$(LIB)
	rm -f /usr/local/include/ed25519.h
	rm -f /usr/local/include/ed25519_core.h
	rm -f /usr/local/include/ed25519_field.h
	@echo "[OK] Uninstallation complete"

# Help target
help:
	@echo "Ed25519 Signature Library - Build Targets"
	@echo "=========================================="
	@echo "  all       - Build static library and test program (default)"
	@echo "  shared    - Build shared library"
	@echo "  test      - Build and run test suite"
	@echo "  clean     - Remove all build artifacts"
	@echo "  install   - Install library to /usr/local"
	@echo "  uninstall - Remove installed library"
	@echo "  help      - Show this help message"
	@echo ""
	@echo "Environment Variables:"
	@echo "  CC        - C compiler (default: gcc)"
	@echo "  CFLAGS    - C compiler flags (default: -O2 -Wall -Wextra -fPIC)"

.PHONY: all shared test clean install uninstall help
