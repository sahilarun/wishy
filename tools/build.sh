#!/bin/bash

set -e

echo "Building wishy OS..."

echo "Building bootloader..."
make boot

echo "Building kernel..."
make kernel

echo "Building userspace..."
make user

echo "Creating disk image..."
make image

echo "Build complete!"
