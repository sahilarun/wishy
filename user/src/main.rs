#![no_std]
#![no_main]
#![feature(lang_items)]

mod utils;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

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
