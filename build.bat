@echo off
echo Building wishy OS...
wsl bash -c "cd /mnt/c/Users/sahil/Downloads/wishy-dev && nasm -f bin boot/stage1.asm -o build/stage1.bin && nasm -f bin boot/stage2.asm -o build/stage2.bin && cat build/stage1.bin build/stage2.bin > build/boot.bin && cd kernel/rust && ~/.cargo/bin/cargo +nightly build --release -Zbuild-std=core,alloc --target ./i686-unknown-none.json && cd ../.. && nasm -f elf32 kernel/entry.asm -o build/entry.o && ld -m elf_i386 -T kernel/linker.ld -o build/kernel.elf build/entry.o kernel/rust/target/i686-unknown-none/release/libwishy_kernel.a && objcopy -O binary build/kernel.elf build/kernel.bin && dd if=/dev/zero of=build/wishy.img bs=1M count=64 2>&1 | tail -1 && dd if=build/boot.bin of=build/wishy.img conv=notrunc 2>&1 | tail -1 && dd if=build/kernel.bin of=build/wishy.img seek=18 conv=notrunc 2>&1 | tail -1 && echo 'Build successful!' && ls -lh build/wishy.img build/kernel.bin"
echo.
echo Build complete! Run with: .\run.bat
pause
