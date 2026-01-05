#!/bin/bash

set -euo pipefail

ACTION="${1:-}"
TEMP_DIR=""

cleanup() {
    if [ -n "$TEMP_DIR" ] && [ -d "$TEMP_DIR" ]; then
        rm -rf "$TEMP_DIR"
    fi
}
trap cleanup EXIT

log_info() { echo "ℹ️  $*"; }
log_error() { echo "❌ Error: $*" >&2; }
log_success() { echo "✅ $*"; }

check_command() {
    local cmd="$1"
    if ! command -v "$cmd" &> /dev/null; then
        log_error "$cmd is required but not installed"
        exit 1
    fi
}

check_sudo() {
    check_command sudo
}
check_sudo() {
    check_command sudo
    if ! sudo -v; then
        log_error "sudo authentication failed"
        exit 1
    fi
}

check_cargo() {
    check_command cargo
}

check_git() {
    check_command git
}

clone_and_build() {
    TEMP_DIR=$(mktemp -d) || { log_error "Failed to create temp directory"; exit 1; }
    cd "$TEMP_DIR" || { log_error "Failed to change to temp directory"; exit 1; }
    log_info "Cloning Codenamr repository..."
    git clone https://github.com/ThomasNowProductions/Codenamr.git || { log_error "Failed to clone repository"; exit 1; }
    cd Codenamr/cli || { log_error "Failed to enter cli directory"; exit 1; }
    log_info "Building Codenamr CLI in release mode..."
    cargo build --release || { log_error "Failed to build project"; exit 1; }
    if [ ! -f "target/release/codenamr-cli" ]; then
        log_error "Build failed: executable not found"
        exit 1
    fi
}

backup_existing() {
    if [ -f "/usr/local/bin/codenamr" ]; then
        log_info "Backing up existing codenamr..."
        sudo cp "/usr/local/bin/codenamr" "/usr/local/bin/codenamr.backup" || { log_error "Failed to create backup"; exit 1; }
    fi
}

install_binary() {
    log_info "Installing codenamr to /usr/local/bin..."
    sudo install -m 755 "target/release/codenamr-cli" "/usr/local/bin/codenamr" || { log_error "Failed to install binary"; exit 1; }
}

install() {
    log_info "Starting Codenamr CLI installation..."
    check_sudo
    check_git
    check_cargo
    clone_and_build
    backup_existing
    install_binary
    log_success "Codenamr CLI installed successfully! You can now run 'codenamr' from anywhere."
    echo "Try: codenamr --help"
}

uninstall() {
    log_info "Starting Codenamr CLI uninstallation..."
    check_sudo
    if [ ! -f "/usr/local/bin/codenamr" ]; then
        log_error "Codenamr CLI is not installed at /usr/local/bin/codenamr."
        exit 1
    fi
    log_info "Removing codenamr from /usr/local/bin..."
    sudo rm "/usr/local/bin/codenamr" || { log_error "Failed to remove binary"; exit 1; }
    if [ -f "/usr/local/bin/codenamr.backup" ]; then
        log_info "Removing backup file..."
        sudo rm "/usr/local/bin/codenamr.backup" || { log_error "Failed to remove backup"; exit 1; }
    fi
    log_success "Codenamr CLI uninstalled successfully!"
}

update() {
    log_info "Starting Codenamr CLI update..."
    check_sudo
    if [ ! -f "/usr/local/bin/codenamr" ]; then
        log_error "Codenamr CLI is not installed. Please run install first."
        exit 1
    fi
    check_git
    check_cargo
    clone_and_build
    backup_existing
    if ! install_binary; then
        log_error "Update failed. Restoring previous version..."
        if [ -f "/usr/local/bin/codenamr.backup" ]; then
            sudo mv "/usr/local/bin/codenamr.backup" "/usr/local/bin/codenamr"
            log_info "Previous version restored"
        fi
        exit 1
    fi
    log_success "Codenamr CLI updated successfully!"
}

case "$ACTION" in
    install|uninstall|update)
        "$ACTION"
        ;;
    *)
        echo "Usage: $0 {install|uninstall|update}"
        exit 1
        ;;
esac
