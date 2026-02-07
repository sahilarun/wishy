#![no_std]
#![no_main]

mod utils;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Provide memset for compiler-generated code
// This C-compatible function is called by LLVM-generated code for memory initialization.
// Safety: The caller must ensure `s` is valid for writes of at least `n` bytes.
#[no_mangle]
pub unsafe extern "C" fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    core::ptr::write_bytes(s, c as u8, n);
    s
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let fd = utils::sys_open(b"/etc/motd\0", 0);
    if fd >= 0 {
        let mut buffer = [0u8; 1024];
        let n = utils::sys_read(fd, &mut buffer);
        if n > 0 {
        }
        utils::sys_close(fd);
    }
    
    let data = b"Hello from userspace!\n";
    let fd = utils::sys_open(b"/tmp/test.txt\0", 1 | 64);
    if fd >= 0 {
        utils::sys_write(fd, data);
        utils::sys_close(fd);
    }
    
    let addr = utils::sys_mmap(0, 4096, 3, 2);
    if addr != 0 {
        unsafe {
            let ptr = addr as *mut u8;
            *ptr = 42;
        }
    }
    
    utils::sys_exit(0);
}
