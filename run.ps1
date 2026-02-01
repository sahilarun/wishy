#!/usr/bin/env pwsh
# wishy OS Launcher

Write-Host "Starting wishy OS..." -ForegroundColor Green
Write-Host "Access the OS at: http://localhost:5900" -ForegroundColor Yellow
Write-Host "Or use a VNC client to connect to: localhost:5900" -ForegroundColor Yellow
Write-Host ""
Write-Host "Press Ctrl+C to stop" -ForegroundColor Cyan
Write-Host ""

wsl bash -c "cd /mnt/c/Users/sahil/Downloads/wishy-dev && qemu-system-i386 -drive format=raw,file=build/wishy.img -m 256M -vnc :0"
