#!/bin/bash

set -euo pipefail
trap 'rm -rf "$TEMP_DIR"' EXIT

echo "Starting Codenamr CLI update..."

command -v sudo >/dev/null || { echo "sudo is required for update"; exit 1; }

if [ ! -f "/usr/local/bin/codenamr" ]; then
    echo "Codenamr CLI is not installed. Please run install.sh first."
    exit 1
fi

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

echo "Installing updated codenamr to /usr/local/bin..."
sudo mv target/release/codenamr-cli /usr/local/bin/codenamr

echo "Codenamr CLI updated successfully!"