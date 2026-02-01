# wishy OS - Installation Guide

Complete step-by-step installation and usage instructions for the wishy operating system.

---

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [System Requirements](#system-requirements)
3. [Installation Steps](#installation-steps)
4. [Building the OS](#building-the-os)
5. [Running in QEMU](#running-in-qemu)
6. [Using the OS](#using-the-os)
7. [Advanced Usage](#advanced-usage)
8. [Troubleshooting](#troubleshooting)

---

## Prerequisites

### Required Software

Install the following tools on your development machine (Ubuntu/Debian example):

```bash
# Update package list
sudo apt update

# Install essential build tools
sudo apt install -y build-essential nasm

# Install QEMU for emulation
sudo apt install -y qemu-system-x86

# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install Rust nightly (required for kernel development)
rustup toolchain install nightly
rustup default nightly

# Add Rust source for custom target
rustup component add rust-src

# Install additional tools
sudo apt install -y git losetup e2fsprogs
```

### For Other Distributions

**Arch Linux:**
```bash
sudo pacman -S base-devel nasm qemu rust git
```

**Fedora:**
```bash
sudo dnf install gcc nasm qemu-system-x86 rust cargo git
```

**macOS:**
```bash
brew install nasm qemu rust
```

---

## System Requirements

### Host System (for building)
- **OS:** Linux, macOS, or WSL2 on Windows
- **RAM:** Minimum 2GB (4GB recommended)
- **Disk:** 500MB free space
- **CPU:** x86_64 processor

### Target System (for running wishy)
- **Architecture:** x86 (i686 or later)
- **RAM:** 32MB minimum, 256MB recommended
- **Display:** VGA-compatible framebuffer
- **Storage:** ATA-compatible disk controller
- **Input:** PS/2 keyboard and mouse

---

## Installation Steps

### Step 1: Clone the Repository

```bash
# Clone from GitHub
git clone https://github.com/notwaris/wishy.git
cd wishy
```

### Step 2: Verify Dependencies

```bash
# Check NASM version (should be 2.14+)
nasm -v

# Check Rust version (should be 1.70+)
rustc --version

# Check QEMU version
qemu-system-i386 --version

# Verify losetup is available
losetup --version
```

### Step 3: Set Up Rust Target

The kernel requires a custom target specification:

```bash
# The target file is already included in kernel/rust/i686-unknown-none.json
# Verify it exists
ls kernel/rust/i686-unknown-none.json
```

### Step 4: Make Scripts Executable

```bash
chmod +x tools/*.sh
```

---

## Building the OS

### Full Build

Build all components (bootloader, kernel, userspace, and disk image):

```bash
make all
```

This will:
1. Assemble the two-stage bootloader
2. Compile the Rust kernel
3. Build userspace programs
4. Create a bootable ext2 disk image

### Build Output

After successful build, you'll find:
```
build/
├── boot.bin          # Combined bootloader (512 bytes stage1 + 8KB stage2)
├── kernel.bin        # Kernel binary
├── user.bin          # Userspace init binary
└── wishy.img         # Bootable disk image (64MB)
```

### Incremental Builds

Build individual components:

```bash
make boot      # Build bootloader only
make kernel    # Build kernel only
make user      # Build userspace only
make image     # Create disk image only
```

### Clean Build

Remove all build artifacts:

```bash
make clean
```

---

## Running in QEMU

### Standard Boot

Launch wishy in QEMU with graphical output:

```bash
make run
```

Or manually:
```bash
qemu-system-i386 \
    -drive format=raw,file=build/wishy.img \
    -m 256M \
    -serial stdio \
    -vga std \
    -display gtk
```

### Debug Mode

Run with GDB server for debugging:

```bash
make debug
```

In another terminal:
```bash
gdb build/kernel.bin
(gdb) target remote localhost:1234
(gdb) continue
```

### QEMU Options Explained

- `-drive format=raw,file=build/wishy.img` - Boot from disk image
- `-m 256M` - Allocate 256MB RAM
- `-serial stdio` - Redirect serial output to terminal
- `-vga std` - Standard VGA graphics adapter
- `-display gtk` - Use GTK display backend
- `-s -S` - Start GDB server on port 1234 and pause on start

---

## Using the OS

### First Boot

When wishy boots, you'll see:

1. **Stage 1 bootloader message**: "Loading stage2..."
2. **Stage 2 bootloader message**: "Stage2 loaded"
3. **Kernel initialization**: Drivers load, filesystem mounts
4. **Compositor starts**: Graphical tiled window manager appears

### Desktop Environment

The compositor features a Hyprland-inspired tiling interface:

#### Panel (Top Bar)
- **Left side**: "wishy v0.1" branding
- **Right side**: 
  - System time (12:34)
  - Launcher icon (grid icon)

#### Window Management

Two demo windows spawn automatically:
- **Terminal** - Main window (left side in master layout)
- **File Manager** - Secondary window (right side)

### Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Alt + Q` | Close focused window |
| `Alt + Enter` | Spawn new terminal window |
| `Alt + Tab` | Cycle through windows (focus next) |
| `Super + L` | Open application launcher grid |

### Mouse Controls

- **Left Click**: Focus window
- **Click + Drag**: Move window
- **Drag from corner**: Resize window (planned)
- **Click close button**: Close window (red X in titlebar)

### Window Features

Each window has:
- **Rounded corners** (8px radius)
- **Border** - Blue for focused, gray for unfocused
- **Drop shadow** - Soft shadow underneath
- **Titlebar** - Dark background with window title
- **Close button** - Top-right corner

### Tiling Behavior

- **1 window**: Fullscreen with gaps
- **2+ windows**: Master-stack layout
  - First window occupies left half
  - Others stack vertically on right half
- **Automatic retiling**: Windows reposition when added/removed

---

## Advanced Usage

### File System Operations

The OS boots with an ext2 filesystem containing:

```
/
├── sbin/
│   └── init          # Userspace init binary
├── etc/
│   └── motd          # Message of the day
├── tmp/              # Temporary files
└── usr/
    └── bin/          # User binaries
```

#### Reading Files (from kernel)

```rust
use wishy_kernel::fs::mount;

let data = mount::read_file("/etc/motd")?;
```

#### Writing Files

```rust
let content = b"Hello, wishy!";
mount::write_file("/tmp/test.txt", content)?;
```

#### Creating Files with Permissions

```rust
mount::create_file("/tmp/newfile", 0o644, 1000, 1000)?;
// mode: rw-r--r--, uid: 1000, gid: 1000
```

### Syscalls from Userspace

The userspace init program (`user/src/main.rs`) demonstrates syscall usage:

```rust
// Open file
let fd = sys_open(b"/etc/motd\0", O_RDONLY);

// Read data
let mut buffer = [0u8; 1024];
let bytes_read = sys_read(fd, &mut buffer);

// Write file
let fd = sys_open(b"/tmp/output.txt\0", O_WRONLY | O_CREAT);
sys_write(fd, b"test data");

// Memory mapping
let addr = sys_mmap(0, 4096, PROT_READ | PROT_WRITE, MAP_PRIVATE);

// Close file
sys_close(fd);
```

### Memory Mapping

Map files into process address space:

```rust
// Map 4KB page
let addr = sys_mmap(
    0,              // Let kernel choose address
    4096,           // Size in bytes
    PROT_READ | PROT_WRITE,  // Permissions
    MAP_PRIVATE     // Private mapping
);

// Access mapped memory
unsafe {
    let ptr = addr as *mut u8;
    *ptr = 42;
}
```

### Customizing the Theme

Edit `kernel/rust/src/gui/theme.rs`:

```rust
pub const fn default() -> Self {
    Self {
        bg_color: 0x1a1a2e,           // Background
        active_border: 0x00d9ff,       // Focused window border (cyan)
        inactive_border: 0x2d3142,     // Unfocused border (gray)
        titlebar_bg: 0x16213e,         // Titlebar background
        titlebar_text: 0xeaeaea,       // Titlebar text
        panel_bg: 0x0f3460,            // Top panel background
        panel_text: 0xe94560,          // Panel text (pink)
        shadow_color: 0x000000,        // Shadow color
        shadow_intensity: 40,          // Shadow opacity (0-255)
        border_width: 2,               // Border thickness
        corner_radius: 8,              // Rounded corner radius
    }
}
```

Rebuild after changes:
```bash
make kernel && make image && make run
```

### Adding Custom Icons

Icons are defined in `kernel/rust/src/gui/icons.rs`. To add a new icon:

```rust
pub fn draw_custom_icon(fb: &mut Framebuffer, x: usize, y: usize) {
    let color = 0xFF00FF;  // Magenta
    let size = 24;
    
    // Draw your icon pixel by pixel
    for dy in 0..size {
        for dx in 0..size {
            if /* your pixel logic */ {
                fb.put_pixel(x + dx, y + dy, color);
            }
        }
    }
}
```

### Creating Custom Userspace Programs

1. Create new binary in `user/src/`:

```rust
// user/src/myapp.rs
#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! { loop {} }

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Your application code
    syscall_exit(0);
}
```

2. Add to `user/Cargo.toml`:

```toml
[[bin]]
name = "myapp"
path = "src/myapp.rs"
```

3. Build and install:

```bash
cd user && cargo build --release
sudo cp target/i686-unknown-linux-musl/release/myapp /path/to/mount/usr/bin/
```

---

## Troubleshooting

### Build Errors

#### "nasm: command not found"
```bash
sudo apt install nasm
```

#### "error: failed to run custom build command for `wishy_kernel`"
Ensure Rust nightly is installed:
```bash
rustup toolchain install nightly
rustup default nightly
rustup component add rust-src
```

#### "ld: cannot find entry symbol _start"
Verify linker script exists:
```bash
ls kernel/linker.ld
```

### Runtime Issues

#### QEMU fails to start
Check QEMU installation:
```bash
qemu-system-i386 --version
```

#### Black screen on boot
- Increase QEMU memory: `-m 512M`
- Check VGA mode: `-vga std`
- Try different display: `-display sdl`

#### No mouse cursor
Mouse support requires PS/2 mouse device:
```bash
qemu-system-i386 -drive file=build/wishy.img -usb -device usb-mouse
```

#### Keyboard not responding
Ensure PS/2 keyboard is enabled (default in QEMU).

#### Disk I/O errors
Recreate the disk image:
```bash
make clean
make all
```

### Debugging Kernel Issues

Enable serial output debugging:

1. Add debug prints in kernel code:
```rust
// In kernel code (not available by default, implement serial driver)
serial_println!("Debug message");
```

2. View output:
```bash
make run
# Serial output appears in terminal
```

### GDB Debugging

```bash
# Terminal 1: Start QEMU in debug mode
make debug

# Terminal 2: Connect GDB
gdb build/kernel.bin
(gdb) target remote localhost:1234
(gdb) break kmain
(gdb) continue
```

### Memory Issues

If experiencing allocation errors:
- Increase heap size in `kernel/rust/src/memory/alloc.rs`
- Increase QEMU RAM: `-m 512M`

### Filesystem Corruption

Recreate ext2 filesystem:
```bash
sudo bash tools/mkext2.sh build/wishy.img build/user.bin images/initrd.img
```

---

## Boot Process Explained

1. **BIOS/UEFI** loads Stage 1 bootloader (sector 0)
2. **Stage 1** loads Stage 2 from sectors 1-16
3. **Stage 2**:
   - Enables A20 line
   - Loads kernel from sector 17+
   - Sets up GDT
   - Switches to protected mode
   - Jumps to kernel entry
4. **Kernel**:
   - Initializes IDT
   - Sets up paging
   - Initializes heap allocator
   - Loads ATA driver
   - Mounts ext2 filesystem
   - Initializes framebuffer
   - Starts compositor
   - Loads userspace init
5. **Compositor** enters main event loop

---

## Performance Tips

### QEMU Acceleration

Enable KVM for near-native performance (Linux hosts):

```bash
qemu-system-i386 \
    -enable-kvm \
    -cpu host \
    -drive format=raw,file=build/wishy.img \
    -m 512M
```

### Build Optimization

Parallel build:
```bash
make -j$(nproc) all
```

### Reduce Build Time

Use incremental builds - only rebuild changed components:
```bash
make kernel  # Only rebuild kernel if modified
```

---

## Creating Bootable USB

To run on real hardware:

```bash
# WARNING: This will destroy all data on the USB drive
# Identify your USB device (e.g., /dev/sdb)
lsblk

# Write image to USB (replace /dev/sdX with your device)
sudo dd if=build/wishy.img of=/dev/sdX bs=4M status=progress
sudo sync
```

Boot from USB:
1. Insert USB drive
2. Restart computer
3. Enter BIOS/UEFI boot menu (usually F12, F2, or Del)
4. Select USB device
5. wishy should boot

---

## Additional Resources

### Project Structure
```
wishy/
├── boot/          - Assembly bootloader
├── kernel/        - Kernel source
│   ├── rust/      - Rust kernel code
│   └── include/   - C headers
├── user/          - Userspace programs
├── tools/         - Build and test scripts
├── tests/         - Unit tests
└── images/        - Disk images
```

### Useful Commands

```bash
# Quick rebuild and run
make clean && make all && make run

# Check disk image contents
sudo mount -o loop,offset=1048576 build/wishy.img /mnt
ls -la /mnt
sudo umount /mnt

# View bootloader hex dump
hexdump -C build/stage1.bin | head

# Check kernel symbols
nm build/kernel.bin | grep kmain
```

### Contributing

For development:
1. Fork the repository
2. Create feature branch
3. Make changes
4. Test thoroughly
5. Submit pull request

---

## Support

For issues or questions:
- GitHub Issues: https://github.com/notwaris/wishy/issues
- Check existing issues before creating new ones
- Provide full build output for build errors
- Include QEMU command and output for runtime issues

---

## Version Information

- **wishy OS Version:** 0.1.0
- **Target Architecture:** i686
- **Minimum QEMU:** 4.0
- **Rust Version:** 1.70+ nightly
- **NASM Version:** 2.14+

---

**Last Updated:** December 2025

**Happy Hacking!** 🚀
