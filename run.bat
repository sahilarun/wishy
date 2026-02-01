@echo off
title wishy OS
cls
echo ========================================
echo   wishy OS - Running in QEMU
echo ========================================
echo.
wsl bash -c "cd /mnt/c/Users/sahil/Downloads/wishy-dev && qemu-system-i386 -drive file=build/wishy.img,format=raw,if=ide,index=0 -m 256M"
