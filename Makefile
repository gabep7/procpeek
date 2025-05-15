PREFIX ?= /usr/local
BINDIR = $(PREFIX)/bin
TARGET_NAME = procpeek
CARGO_BUILD_TARGET_DIR = target/release

all: build

build:
	@echo "Building $(TARGET_NAME) with Cargo..."
	@cargo build --release
	@echo "$(TARGET_NAME) successfully built in $(CARGO_BUILD_TARGET_DIR)/"

install: build
	@echo "Installing $(TARGET_NAME) to $(DESTDIR)$(BINDIR)/..."
	@mkdir -p "$(DESTDIR)$(BINDIR)"
	@cp "$(CARGO_BUILD_TARGET_DIR)/$(TARGET_NAME)" "$(DESTDIR)$(BINDIR)/$(TARGET_NAME)"
	@chmod 755 "$(DESTDIR)$(BINDIR)/$(TARGET_NAME)"
	@echo "$(TARGET_NAME) installed to $(DESTDIR)$(BINDIR)/$(TARGET_NAME)"
	@echo "Ensure $(DESTDIR)$(BINDIR) is in your PATH."
	@echo "You may need to run this with superuser privileges (e.g., sudo make install)"

uninstall:
	@echo "Uninstalling $(TARGET_NAME) from $(BINDIR)/$(TARGET_NAME)..."
	@rm -f "$(DESTDIR)$(BINDIR)/$(TARGET_NAME)"
	@echo "$(TARGET_NAME) uninstalled."
	@echo "You may need to run this with superuser privileges (e.g., sudo make uninstall)"

clean:
	@echo "Cleaning build artifacts..."
	@cargo clean
	@echo "Clean complete."

.PHONY: all build install uninstall clean
