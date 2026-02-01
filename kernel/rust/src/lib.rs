#![no_std]
#![feature(abi_x86_interrupt)]
#![feature(allocator_api)]
#![feature(alloc_error_handler)]

extern crate alloc;

use core::panic::PanicInfo;

pub mod drivers;
pub mod fs;
pub mod gui;
pub mod exec;
pub mod memory;
pub mod syscall;
pub mod kmain;
pub mod compat;
pub mod gpu;
pub mod wayland;
pub mod console;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[alloc_error_handler]
fn alloc_error(_layout: core::alloc::Layout) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn kmain() {
    kmain::kernel_main();
}
