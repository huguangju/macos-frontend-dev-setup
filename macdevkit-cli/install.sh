#!/bin/bash

# Colors for better readability
GREEN='\033[0;32m'
RED='\033[0;31m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

echo -e "${CYAN}MacDevKit CLI Installer${NC}"
echo -e "${CYAN}======================${NC}"

# Check if Rust and Cargo are installed
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: Rust and Cargo are required but not installed.${NC}"
    echo -e "Please install Rust from https://rustup.rs/ and try again."
    exit 1
fi

# Check if init.sh exists
if [ ! -f "../init.sh" ] && [ ! -f "init.sh" ]; then
    echo -e "${RED}Error: init.sh not found.${NC}"
    echo -e "Please make sure init.sh is in the parent directory or current directory."
    exit 1
fi

# Copy init.sh if needed
if [ ! -f "init.sh" ] && [ -f "../init.sh" ]; then
    echo -e "${CYAN}Copying init.sh from parent directory...${NC}"
    cp ../init.sh .
fi

# Copy init_wrapper.sh if it doesn't exist
if [ ! -f "init_wrapper.sh" ]; then
    echo -e "${RED}Error: init_wrapper.sh not found.${NC}"
    echo -e "The wrapper script is missing. Please make sure it's in the project directory."
    exit 1
fi

# Build the project
echo -e "${CYAN}Building MacDevKit CLI...${NC}"
cargo build --release
if [ $? -ne 0 ]; then
    echo -e "${RED}Error: Build failed.${NC}"
    exit 1
fi

# Create install directory
INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"

# Copy the binary
echo -e "${CYAN}Installing MacDevKit CLI to $INSTALL_DIR...${NC}"
cp target/release/macdevkit-cli "$INSTALL_DIR/macdevkit"
chmod +x "$INSTALL_DIR/macdevkit"

# Check if install directory is in PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo -e "${CYAN}Adding $INSTALL_DIR to PATH...${NC}"
    
    # Determine shell profile file
    SHELL_PROFILE=""
    if [ -f "$HOME/.zshrc" ]; then
        SHELL_PROFILE="$HOME/.zshrc"
    elif [ -f "$HOME/.bash_profile" ]; then
        SHELL_PROFILE="$HOME/.bash_profile"
    elif [ -f "$HOME/.bashrc" ]; then
        SHELL_PROFILE="$HOME/.bashrc"
    fi
    
    if [ -n "$SHELL_PROFILE" ]; then
        echo "export PATH=\"\$PATH:$INSTALL_DIR\"" >> "$SHELL_PROFILE"
        echo -e "${GREEN}Added $INSTALL_DIR to PATH in $SHELL_PROFILE${NC}"
        echo -e "${CYAN}Please restart your terminal or run 'source $SHELL_PROFILE' to update your PATH.${NC}"
    else
        echo -e "${RED}Could not find shell profile file. Please add $INSTALL_DIR to your PATH manually.${NC}"
    fi
fi

echo -e "${GREEN}MacDevKit CLI installed successfully!${NC}"
echo -e "You can now run it with: ${CYAN}macdevkit${NC}" 