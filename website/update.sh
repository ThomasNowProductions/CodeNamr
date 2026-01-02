#!/bin/bash

# Exit immediately if a command exits with a non-zero status.
set -e

echo "Starting Codenamr CLI update..."

# Create a temporary directory for cloning
TEMP_DIR=$(mktemp -d)
cd "$TEMP_DIR"

# Clone the repository
echo "Cloning Codenamr repository..."
git clone https://github.com/ThomasNowProductions/Codenamr.git

# Navigate to the CLI directory
cd Codenamr/cli

# Pull the latest changes from the git repository
echo "Pulling latest changes from git..."
git pull

# Build the CLI in release mode
echo "Building Codenamr CLI in release mode..."
cargo build --release

# Move the executable to /usr/local/bin
echo "Installing updated codenamr to /usr/local/bin..."
sudo mv target/release/codenamr-cli /usr/local/bin/codenamr

# Clean up temporary directory
cd /
rm -rf "$TEMP_DIR"

echo "Codenamr CLI updated successfully!"