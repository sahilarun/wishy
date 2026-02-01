#!/bin/bash
# Setup script for wishy OS build environment

echo "=== Setting up wishy OS build environment ==="

# Update package lists
echo "Updating package lists..."
sudo apt update

# Install build tools
echo "Installing build tools..."
sudo apt install -y nasm gcc qemu-system-x86 build-essential gcc-multilib

# Install Rust
echo "Installing Rust..."
if ! command -v rustc &> /dev/null; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

# Install nightly Rust toolchain
echo "Installing Rust nightly toolchain..."
rustup toolchain install nightly

echo "=== Setup complete! ==="
echo ""
echo "You can now build wishy OS with:"
echo "  make all"
echo "  make run"
