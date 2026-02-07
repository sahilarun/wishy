.PHONY: all clean boot kernel user image run debug chromium

ASM = nasm
LD = ld
RUSTC = cargo
QEMU = qemu-system-i386

BUILD_DIR = build
BOOT_DIR = boot
KERNEL_DIR = kernel
USER_DIR = user
TOOLS_DIR = tools
IMAGES_DIR = images
USERSPACE_DIR = userspace
ROOTFS_DIR = rootfs

BOOT_BIN = $(BUILD_DIR)/boot.bin
KERNEL_BIN = $(BUILD_DIR)/kernel.bin
USER_BIN = $(BUILD_DIR)/user.bin
DISK_IMG = $(BUILD_DIR)/wishy.img

all: $(DISK_IMG)

$(BUILD_DIR):
	mkdir -p $(BUILD_DIR)

boot: $(BOOT_BIN)

$(BOOT_BIN): $(BUILD_DIR)
	$(ASM) -f bin $(BOOT_DIR)/stage1.asm -o $(BUILD_DIR)/stage1.bin
	$(ASM) -f bin $(BOOT_DIR)/stage2.asm -o $(BUILD_DIR)/stage2.bin
	cat $(BUILD_DIR)/stage1.bin $(BUILD_DIR)/stage2.bin > $(BOOT_BIN)

kernel: $(KERNEL_BIN)

$(KERNEL_BIN): $(BUILD_DIR)
	$(ASM) -f elf32 $(KERNEL_DIR)/entry.asm -o $(BUILD_DIR)/entry.o
	$(ASM) -f elf32 $(KERNEL_DIR)/interrupts.asm -o $(BUILD_DIR)/interrupts.o
	cd $(KERNEL_DIR)/rust && $(RUSTC) build --release --target i686-unknown-none.json -Z build-std=core,alloc -Z json-target-spec
	$(LD) -m elf_i386 -T $(KERNEL_DIR)/linker.ld -o $(BUILD_DIR)/kernel.elf \
		$(BUILD_DIR)/entry.o \
		$(BUILD_DIR)/interrupts.o \
		$(KERNEL_DIR)/rust/target/i686-unknown-none/release/libwishy_kernel.a
	objcopy -O binary $(BUILD_DIR)/kernel.elf $(KERNEL_BIN)

user: $(USER_BIN)

$(USER_BIN): $(BUILD_DIR)
	cd $(USER_DIR) && $(RUSTC) build --release --target i686-unknown-linux-musl
	cp $(USER_DIR)/target/i686-unknown-linux-musl/release/wishy_user $(USER_BIN)

chromium: $(BUILD_DIR)
	bash $(TOOLS_DIR)/build_chromium_support.sh

image: $(DISK_IMG)

$(DISK_IMG): boot kernel user
	dd if=/dev/zero of=$(DISK_IMG) bs=1M count=6432
	dd if=$(BOOT_BIN) of=$(DISK_IMG) conv=notrunc
	dd if=$(KERNEL_BIN) of=$(DISK_IMG) bs=512 seek=18 conv=notrunc
	bash $(TOOLS_DIR)/mkext2.sh $(DISK_IMG) $(USER_BIN) $(IMAGES_DIR)/initrd.img
	dd if=$(KERNEL_BIN) of=$(DISK_IMG) bs=512 seek=17 conv=notrunc
run: $(DISK_IMG)
	bash $(TOOLS_DIR)/run_qemu.sh

run-chromium: $(DISK_IMG)
	$(QEMU) \
		-drive format=raw,file=$(DISK_IMG) \
		-m 512M \
		-device virtio-vga-gl \
		-display gtk,gl=on \
		-device virtio-keyboard \
		-device virtio-mouse \
		-enable-kvm

debug: $(DISK_IMG)
	bash $(TOOLS_DIR)/debug.sh

clean:
	rm -rf $(BUILD_DIR)
	cd $(KERNEL_DIR)/rust && cargo clean
	cd $(USER_DIR) && cargo clean
