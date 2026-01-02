#!/bin/bash

# Exit immediately if a command exits with a non-zero status.
set -e

echo "Starting Codenamr CLI update..."

# Pull the latest changes from the git repository
echo "Pulling latest changes from git..."
cd .. && git pull && cd cli

# Run the installation script
echo "Running the installation script..."
./install.sh

echo "Codenamr CLI updated successfully!"
