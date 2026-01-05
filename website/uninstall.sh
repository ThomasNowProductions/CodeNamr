#!/bin/bash

set -euo pipefail

echo "Starting Codenamr CLI uninstallation..."

command -v sudo >/dev/null || { echo "sudo is required for uninstallation"; exit 1; }

if [ ! -f "/usr/local/bin/codenamr" ]; then
    echo "Codenamr CLI is not installed at /usr/local/bin/codenamr."
    exit 1
fi

echo "Removing codenamr from /usr/local/bin..."
sudo rm /usr/local/bin/codenamr

if [ -f "/usr/local/bin/codenamr.backup" ]; then
    echo "Removing backup file..."
    sudo rm /usr/local/bin/codenamr.backup
fi

echo "Codenamr CLI uninstalled successfully!"