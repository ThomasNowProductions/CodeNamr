#!/bin/bash

# Exit immediately if a command exits with a non-zero status.
set -e

echo "Starting CodeNamr CLI uninstallation..."

# Check if the binary exists
if [ ! -f "/usr/local/bin/codenamr" ]; then
    echo "CodeNamr CLI is not installed at /usr/local/bin/codenamr."
    exit 1
fi

# Remove the binary
echo "Removing codenamr from /usr/local/bin..."
sudo rm /usr/local/bin/codenamr

echo "CodeNamr CLI uninstalled successfully!"