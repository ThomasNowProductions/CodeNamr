#!/bin/bash

set -euo pipefail
trap 'rm -rf "$TEMP_DIR"' EXIT

echo "Starting Codenamr CLI installation..."

command -v sudo >/dev/null || { echo "sudo is required for installation"; exit 1; }

if ! command -v cargo &> /dev/null
then
    echo "Rust (cargo) is not installed. Please install Rust by running:"
    echo "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    echo "Then restart your terminal or source your shell's rc file before trying again."
    exit 1
fi

echo "Rust is installed. Proceeding with build."

TEMP_DIR=$(mktemp -d)
cd "$TEMP_DIR" || exit

echo "Cloning Codenamr repository..."
git clone https://github.com/ThomasNowProductions/Codenamr.git

cd Codenamr/cli || exit

echo "Building Codenamr CLI in release mode..."
cargo build --release

if [ ! -f "target/release/codenamr-cli" ]; then
    echo "Build failed: executable not found"
    exit 1
fi

if [ -f "/usr/local/bin/codenamr" ]; then
    echo "Backing up existing codenamr..."
    sudo cp /usr/local/bin/codenamr /usr/local/bin/codenamr.backup
fi

echo "Installing codenamr to /usr/local/bin..."
sudo mv target/release/codenamr-cli /usr/local/bin/codenamr

echo "Codenamr CLI installed successfully! You can now run 'codenamr' from anywhere."
echo "Try: codenamr --help"