#!/bin/bash

# Exit immediately if a command exits with a non-zero status.
set -e

echo "Starting CodeNamr CLI installation..."

# Check for Rust installation
if ! command -v cargo &> /dev/null
then
    echo "Rust (cargo) is not installed. Please install Rust by running:"
    echo "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    echo "Then restart your terminal or source your shell's rc file before trying again."
    exit 1
fi

echo "Rust is installed. Proceeding with build."

# Create a temporary directory for cloning
TEMP_DIR=$(mktemp -d)
cd "$TEMP_DIR"

# Clone the repository
echo "Cloning CodeNamr repository..."
git clone https://github.com/ThomasNowProductions/CodeNamr.git

# Navigate to the CLI directory
cd CodeNamr/cli

# Build the CLI in release mode
echo "Building CodeNamr CLI in release mode..."
cargo build --release

# Move the executable to /usr/local/bin
echo "Installing codenamr to /usr/local/bin..."
sudo mv target/release/codenamr-cli /usr/local/bin/codenamr

# Clean up temporary directory
cd /
rm -rf "$TEMP_DIR"

echo "CodeNamr CLI installed successfully! You can now run 'codenamr' from anywhere."
echo "Try: codenamr --help"