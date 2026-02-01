# wishy OS - Quick Reference Guide

## Build Commands

```bash
# Basic OS
make all              # Build bootloader + kernel + user
make boot             # Build bootloader only
make kernel           # Build kernel only
make user             # Build userspace only
make image            # Create disk image
make clean            # Clean all build artifacts

# With Chromium support
make chromium         # Download and integrate Chromium
make run-chromium     # Run with GPU acceleration
```

## Run Commands

```bash
# Standard mode (256MB RAM, no GPU)
make run

# Debug mode (GDB server on port 1234)
make debug

# Chromium mode (512MB RAM, GPU acceleration)
make run-chromium

# Custom QEMU
qemu-system-i386 \
    -drive format=raw,file=build/wishy.img \
    -m 512M \
    -device virtio-vga-gl \
    -display gtk,gl=on \
    -enable-kvm
```

## Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Alt + Q` | Close focused window |
| `Alt + Enter` | Spawn new terminal |
| `Alt + Tab` | Cycle window focus |
| `Super + L` | Launch application grid |

## Syscall Numbers

### Process Management
- 39: getpid()
- 57: fork()
- 59: execve()
- 60: exit()
- 61: wait4()
- 110: getppid()
- 186: gettid()

### Memory
- 9: mmap()
- 11: munmap()

### File I/O
- 0: read()
- 1: write()
- 2: open()
- 3: close()
- 4: stat()
- 5: fstat()
- 257: openat()

### Networking
- 41: socket()
- 42: connect()
- 43: accept()
- 44: sendto()
- 45: recvfrom()
- 49: bind()
- 50: listen()

### Threading
- 56: clone()
- 202: futex()
- 218: set_tid_address()

### Signals
- 13: rt_sigaction()
- 14: rt_sigprocmask()

## Wayland Protocol

### Core Interfaces
- wl_display (1): Display connection
- wl_registry (2): Object registry
- wl_compositor (3): Surface creation
- wl_shm (4): Shared memory
- wl_surface (5): Window surface

### XDG Shell
- xdg_wm_base (12): Shell base
- xdg_surface (13): Surface role
- xdg_toplevel (14): Top-level window

### Requests
- wl_display.sync: Synchronize
- wl_display.get_registry: Get globals
- wl_registry.bind: Bind interface
- wl_compositor.create_surface: New surface
- wl_surface.attach: Attach buffer
- wl_surface.commit: Commit changes

## DRM ioctl Commands

```c
// Create buffer
struct drm_mode_create_dumb {
    uint32_t height, width, bpp;
    uint32_t handle, pitch;
    uint64_t size;
};
ioctl(fd, DRM_IOCTL_MODE_CREATE_DUMB, &args);

// Map buffer
struct drm_mode_map_dumb {
    uint32_t handle;
    uint64_t offset;
};
ioctl(fd, DRM_IOCTL_MODE_MAP_DUMB, &args);
void *ptr = mmap(0, size, PROT_READ|PROT_WRITE, MAP_SHARED, fd, args.offset);

// Page flip
ioctl(fd, DRM_IOCTL_MODE_PAGE_FLIP, &args);
```

## File Paths

### Kernel
- `/kernel/rust/src/compat/` - Linux compatibility
- `/kernel/rust/src/gpu/` - GPU drivers
- `/kernel/rust/src/wayland/` - Wayland server
- `/kernel/rust/src/gui/` - Compositor (unchanged)
- `/kernel/rust/src/fs/` - Filesystem (unchanged)

### Userspace
- `/userspace/chromium_launcher.rs` - Chromium startup
- `/rootfs/usr/bin/chromium` - Wrapper script
- `/rootfs/usr/lib/libchromium_shim.so` - libc shim

### Runtime
- `/tmp/wayland-0` - Wayland socket
- `/dev/dri/card0` - DRM device (virtual)
- `/root/` - Home directory

## Environment Variables

```bash
# Wayland
export WAYLAND_DISPLAY=wayland-0
export XDG_RUNTIME_DIR=/tmp

# Chromium
export LD_PRELOAD=/usr/lib/libchromium_shim.so

# Debugging
export WAYLAND_DEBUG=1     # Wayland protocol messages
export RUST_BACKTRACE=1    # Rust backtraces
```

## Chromium Flags

```bash
chromium \
  --no-sandbox                          # Disable sandbox (required)
  --disable-gpu-sandbox                 # Disable GPU sandbox
  --disable-setuid-sandbox              # Disable setuid sandbox
  --disable-dev-shm-usage               # Don't use /dev/shm
  --enable-features=UseOzonePlatform    # Enable Ozone
  --ozone-platform=wayland              # Use Wayland backend
  --enable-logging=stderr               # Debug logging
  --v=1                                 # Verbosity level
```

## Debugging

### GDB Debugging
```bash
# Terminal 1
make debug

# Terminal 2
gdb build/kernel.bin
(gdb) target remote localhost:1234
(gdb) break kmain
(gdb) continue
```

### Wayland Protocol Debugging
```bash
export WAYLAND_DEBUG=1
chromium 2>&1 | grep "wl_"
```

### Syscall Tracing
Add to `linux_syscalls.rs`:
```rust
pub fn linux_syscall_handler(num: u32, args: &[usize; 6]) -> isize {
    debug_print!("syscall {}: {:?}", num, args);
    // ... existing code
}
```

## Memory Layout

```
0x00000000 - 0x00100000   Kernel code/data
0x00100000 - 0x00200000   Page tables
0x00200000 - 0x00600000   Kernel heap
0x01000000 - 0x10000000   Process memory
0x40000000 - 0x50000000   mmap region
0x60000000 - 0x70000000   DRM buffers
0x70000000 - 0x80000000   Shared memory
0xE0000000 - 0xE1000000   Framebuffer
0xFEBD0000                Virtio-GPU MMIO
```

## Performance Tips

1. **Enable KVM:**
   ```bash
   qemu-system-i386 -enable-kvm ...
   ```

2. **Increase memory:**
   ```bash
   -m 512M  # or 1024M
   ```

3. **Use virtio devices:**
   ```bash
   -device virtio-vga-gl
   -device virtio-keyboard
   -device virtio-mouse
   ```

4. **Parallel builds:**
   ```bash
   make -j$(nproc)
   ```

## Common Issues

### Chromium won't start
- Check memory: needs 512MB minimum
- Verify Wayland socket: `ls /tmp/wayland-0`
- Check environment: `echo $WAYLAND_DISPLAY`

### Black screen
- Increase QEMU memory
- Check framebuffer initialization
- Try software rendering: `--disable-gpu`

### No input
- Verify input devices in QEMU
- Check compositor input handling
- Test with keyboard shortcuts first

### Slow performance
- Enable KVM acceleration
- Use virtio-gpu
- Increase CPU cores: `-smp 2`

## File Sizes

- Kernel: ~2MB
- Chromium: ~200MB
- Total OS image: ~300MB
- Chromium launcher: ~100KB
- libchromium_shim.so: ~50KB

## Build Dependencies

```bash
# Ubuntu/Debian
sudo apt install nasm gcc build-essential \
    qemu-system-x86 wget unzip gcc-multilib

# Arch Linux
sudo pacman -S nasm gcc qemu wget unzip

# Fedora
sudo dnf install nasm gcc qemu-system-x86 \
    wget unzip gcc.i686

# Rust
curl --proto '=https' --tlsv1.2 -sSf \
    https://sh.rustup.rs | sh
rustup toolchain install nightly
rustup component add rust-src
```

## Testing Checklist

- [ ] OS boots to compositor
- [ ] Keyboard input works
- [ ] Mouse input works
- [ ] Windows tile correctly
- [ ] Chromium launcher executes
- [ ] Chromium window appears
- [ ] Chromium accepts input
- [ ] Chromium renders UI
- [ ] Window close button works
- [ ] Alt+Q closes window
- [ ] GPU acceleration (if enabled)

## Useful Links

- [Wayland Protocol](https://wayland.freedesktop.org/docs/html/)
- [DRM Documentation](https://dri.freedesktop.org/wiki/DRM/)
- [Linux Syscalls](https://man7.org/linux/man-pages/man2/syscalls.2.html)
- [Chromium Ozone](https://chromium.googlesource.com/chromium/src/+/master/docs/ozone_overview.md)
- [EGL Specification](https://www.khronos.org/registry/EGL/specs/eglspec.1.5.pdf)

## Version Info

- wishy OS: 0.1.0
- Linux Compat: 60+ syscalls
- Wayland: Core + XDG Shell
- GPU: DRM + Virtio + EGL
- Chromium: Latest stable
