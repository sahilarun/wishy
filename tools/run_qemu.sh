#!/bin/bash

qemu-system-i386 \
    -drive format=raw,file=build/wishy.img \
    -m 256M \
    -serial stdio \
    -vga std \
    -display gtk
