.PHONY: build install clean test run

# Default target
all: build

# Build the project
build:
	@echo "Building MacDevKit CLI..."
	cargo build --release

# Install the binary to ~/.local/bin (create directory if it doesn't exist)
install: build
	@echo "Installing MacDevKit CLI..."
	mkdir -p $(HOME)/.local/bin
	cp target/release/macdevkit-cli $(HOME)/.local/bin/macdevkit
	@echo "MacDevKit CLI installed to $(HOME)/.local/bin/macdevkit"
	@echo "Make sure $(HOME)/.local/bin is in your PATH"

# Clean the project
clean:
	@echo "Cleaning build artifacts..."
	cargo clean

# Run the project
run:
	@echo "Running MacDevKit CLI..."
	cargo run

# Run tests
test:
	@echo "Running tests..."
	cargo test

# Copy the init.sh script to the project directory
copy-script:
	@echo "Copying init.sh script..."
	cp ../init.sh .
	@echo "Script copied successfully" 