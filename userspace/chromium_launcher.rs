#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    setup_environment();
    launch_chromium();
    
    loop {}
}

fn setup_environment() {
    set_env(b"WAYLAND_DISPLAY", b"wayland-0");
    set_env(b"XDG_RUNTIME_DIR", b"/tmp");
    set_env(b"HOME", b"/root");
    set_env(b"DISPLAY", b":0");
}

fn launch_chromium() {
    let chromium_path = b"/usr/bin/chromium\0";
    let args = [
        chromium_path.as_ptr(),
        b"--no-sandbox\0".as_ptr(),
        b"--disable-gpu-sandbox\0".as_ptr(),
        b"--disable-setuid-sandbox\0".as_ptr(),
        b"--disable-dev-shm-usage\0".as_ptr(),
        b"--enable-features=UseOzonePlatform\0".as_ptr(),
        b"--ozone-platform=wayland\0".as_ptr(),
        core::ptr::null(),
    ];
    
    let envp = [
        b"WAYLAND_DISPLAY=wayland-0\0".as_ptr(),
        b"XDG_RUNTIME_DIR=/tmp\0".as_ptr(),
        b"HOME=/root\0".as_ptr(),
        core::ptr::null(),
    ];
    
    syscall_execve(chromium_path.as_ptr(), args.as_ptr() as usize, envp.as_ptr() as usize);
}

fn set_env(key: &[u8], value: &[u8]) {
}

fn syscall_execve(filename: *const u8, argv: usize, envp: usize) -> isize {
    let ret: isize;
    unsafe {
        core::arch::asm!(
            "int 0x80",
            in("eax") 59,
            in("ebx") filename,
            in("ecx") argv,
            in("edx") envp,
            lateout("eax") ret
        );
    }
    ret
}
