# wishy OS – Architecture Documentation

This document describes the actual, current architecture of the wishy operating
system prototype. Only components that exist in code or have been demonstrably
booted are described as implemented. Planned subsystems are explicitly marked
as WIP (work in progress).

wishy is an experimental x86 OS kernel project developed and tested under QEMU.

## High-Level System Architecture

The current execution flow of wishy is:
```
BIOS / QEMU
  |
  v
Stage 1 Bootloader (Real Mode)
  |
  v
Stage 2 Loader (Protected Mode Transition)
  |
  v
Kernel (Protected Mode)
  |
  v
Kernel Subsystems (Partial)
  |
  v
Userspace Infrastructure (WIP)
```

## Boot Architecture

### Stage 1 Bootloader
- Written in x86 assembly
- Loaded by BIOS from the first disk sector (MBR)
- Responsibilities:
  - Minimal hardware setup
  - Load Stage 2 from disk into memory
  - Jump to Stage 2 entry point

### Stage 2 Loader
- Runs initially in real mode
- Responsibilities:
  - Enable A20 line
  - Set up Global Descriptor Table (GDT)
  - Switch CPU to protected mode
  - Load kernel binary from disk
  - Transfer control to kernel entry point

Stage 2 does not implement filesystem parsing; it reads fixed disk sectors.

## Kernel Architecture

### Execution Mode
- Protected mode (x86)
- Single address space
- Ring 0 only (no user/kernel separation yet)

### Kernel Entry
- Kernel entry point is invoked by Stage 2
- Early initialization includes:
  - Basic CPU setup
  - Early memory structures
  - VGA / serial output initialization


## Kernel Subsystems (Current State)

### Memory Management (Partial)
- Paging structures are initialized
- Static memory regions are defined
- Heap allocator exists in early form (WIP)
- No full virtual memory isolation between processes yet

### Output
- VGA text or framebuffer output supported
- Used primarily for boot and debug messages

### Interrupts
- IDT setup exists in early form
- Hardware interrupt handling is incomplete (WIP)


## Filesystem and Storage (WIP)

- ext2 filesystem code exists in the repository
- VFS abstraction exists as scaffolding
- Disk access is present at low level
- Filesystem mounting and persistence are not yet demonstrated end-to-end

Filesystem-related code should be considered **experimental**.

## Userspace Infrastructure (WIP)

- ELF loader code exists
- Userspace execution pipeline is under development
- No interactive shell or stable init process is exposed yet
- No privilege separation (Ring 3) implemented

Userspace binaries are not yet demonstrated running independently.

## Graphics and GUI (WIP)

- Framebuffer-based drawing code exists
- GUI / compositor directories exist as experimental work
- No complete window manager or graphical session is demonstrated
- No Wayland server is currently running in a verifiable manner

All GUI-related code should be treated as **non-functional prototypes**.


## Linux Compatibility Layer (WIP)

- Compatibility layer code exists in the tree
- No verified syscall coverage
- No proof of Linux binaries executing
- No ABI guarantees

This subsystem is **planned**, not implemented.


## Hardware Support (Current)

- QEMU-emulated x86 CPU
- ATA-compatible disk (sector reads)
- VGA-compatible display
- Keyboard and mouse drivers are incomplete (WIP)
- No GPU acceleration

Real hardware support is untested.

## File Structure Overview
```
wishy/
|
|-- boot/        - Assembly bootloader (Stage 1 and Stage 2)
|
|-- kernel/      - Kernel source code (Rust)
|   |-- drivers/ - Hardware drivers (WIP)
|   |-- fs/      - Filesystem code (WIP)
|   |-- gui/     - Graphics / compositor experiments (WIP)
|   |-- compat/  - Linux compatibility experiments (WIP)
|   |-- wayland/ - Wayland protocol experiments (WIP)
|
|-- userspace/   - Userspace programs (WIP)
|
|-- rootfs/      - Root filesystem layout (experimental)
```

## Memory Layout (Conceptual)

This layout represents intended organization, not a finalized ABI.
```
0x00000000  - Null / reserved
0x00010000  - Kernel code and data
0x00100000  - Paging structures
0x00200000  - Kernel heap (WIP)
0x01000000  - Future userspace memory
0xE0000000  - Framebuffer region (QEMU)
```

Addresses may change as memory management matures.


## Design Principles

- Incremental development
- Verifiable behavior over claims
- QEMU-first testing
- No undocumented features
- Clear separation between implemented and planned subsystems


## Known Limitations

- No stable userspace
- No multitasking scheduler
- No syscall ABI
- No networking stack
- No GUI session
- No browser support
- No GPU acceleration

These limitations are expected at this stage.

## Roadmap (High-Level)

- Demonstrate ext2 read/write persistence
- Execute a real userspace ELF binary
- Introduce basic process scheduling
- Establish kernel/userspace boundary
- Incrementally expand I/O and drivers

---

Note:
This architecture document will evolve strictly alongside demonstrable
progress. Subsystems will be marked implemented only after proof of execution.
