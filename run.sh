#!/bin/bash
cd /mnt/c/Users/sahil/Downloads/wishy-dev
echo "Starting wishy OS..."
qemu-system-i386 -drive format=raw,file=build/wishy.img -m 256M -vga std -serial mon:stdio
