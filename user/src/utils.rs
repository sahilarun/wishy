pub fn sys_read(fd: i32, buf: &mut [u8]) -> isize {
    let ret: isize;
    unsafe {
        core::arch::asm!(
            "int 0x80",
            in("eax") 0,
            in("ebx") fd,
            in("ecx") buf.as_mut_ptr(),
            in("edx") buf.len(),
            lateout("eax") ret
        );
    }
    ret
}

pub fn sys_write(fd: i32, buf: &[u8]) -> isize {
    let ret: isize;
    unsafe {
        core::arch::asm!(
            "int 0x80",
            in("eax") 1,
            in("ebx") fd,
            in("ecx") buf.as_ptr(),
            in("edx") buf.len(),
            lateout("eax") ret
        );
    }
    ret
}

pub fn sys_open(path: &[u8], flags: i32) -> i32 {
    let ret: i32;
    unsafe {
        core::arch::asm!(
            "int 0x80",
            in("eax") 2,
            in("ebx") path.as_ptr(),
            in("ecx") flags,
            lateout("eax") ret
        );
    }
    ret
}

pub fn sys_close(fd: i32) -> i32 {
    let ret: i32;
    unsafe {
        core::arch::asm!(
            "int 0x80",
            in("eax") 3,
            in("ebx") fd,
            lateout("eax") ret
        );
    }
    ret
}

pub fn sys_mmap(addr: usize, length: usize, prot: i32, flags: i32) -> usize {
    let ret: usize;
    unsafe {
        core::arch::asm!(
            "int 0x80",
            in("eax") 9,
            in("ebx") addr,
            in("ecx") length,
            in("edx") prot,
            in("edi") flags,
            lateout("eax") ret
        );
    }
    ret
}

pub fn sys_exit(code: i32) -> ! {
    unsafe {
        core::arch::asm!(
            "int 0x80",
            in("eax") 60,
            in("ebx") code,
            options(noreturn)
        );
    }
}
