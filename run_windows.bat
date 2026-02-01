@echo off
title wishy OS (Native Windows QEMU)
cls
echo ========================================
echo   wishy OS - Windows Native QEMU
echo ========================================
echo.

REM Try to find QEMU in common locations
set QEMU_EXE=
if exist "C:\Program Files\qemu\qemu-system-i386w.exe" set QEMU_EXE=C:\Program Files\qemu\qemu-system-i386w.exe
if exist "C:\qemu\qemu-system-i386w.exe" set QEMU_EXE=C:\qemu\qemu-system-i386w.exe
if exist "%LOCALAPPDATA%\Programs\QEMU\qemu-system-i386w.exe" set QEMU_EXE=%LOCALAPPDATA%\Programs\QEMU\qemu-system-i386w.exe

if not defined QEMU_EXE (
    echo ERROR: Could not find QEMU installation
    echo Please install QEMU for Windows or use WSL version
    pause
    exit /b 1
)

echo Using: %QEMU_EXE%
echo.
"%QEMU_EXE%" -drive file=build\wishy.img,format=raw,index=0,media=disk -m 256M
