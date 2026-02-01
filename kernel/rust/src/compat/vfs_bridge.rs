use crate::fs::mount;

pub fn compat_read(fd: i32, buf: *mut u8, count: usize) -> isize {
    crate::syscall::syscall_handler(0, fd as u32, buf as u32, count as u32, 0) as isize
}

pub fn compat_write(fd: i32, buf: *const u8, count: usize) -> isize {
    crate::syscall::syscall_handler(1, fd as u32, buf as u32, count as u32, 0) as isize
}

pub fn compat_open(path: *const u8, flags: i32, mode: u32) -> isize {
    let mut path_buf = [0u8; 256];
    let mut len = 0;
    
    unsafe {
        while len < 255 && *path.add(len) != 0 {
            path_buf[len] = *path.add(len);
            len += 1;
        }
    }
    
    crate::syscall::syscall_handler(2, path_buf.as_ptr() as u32, flags as u32, 0, 0) as isize
}

pub fn compat_close(fd: i32) -> isize {
    crate::syscall::syscall_handler(3, fd as u32, 0, 0, 0) as isize
}

pub fn compat_stat(path: *const u8, statbuf: *mut u8) -> isize {
    let mut path_buf = [0u8; 256];
    let mut len = 0;
    
    unsafe {
        while len < 255 && *path.add(len) != 0 {
            path_buf[len] = *path.add(len);
            len += 1;
        }
    }
    
    let path_str = core::str::from_utf8(&path_buf[..len]).unwrap_or("");
    
    match mount::stat(path_str) {
        Ok(stat) => {
            unsafe {
                let buf = statbuf as *mut u64;
                *buf = 0;
                *buf.add(1) = 0;
                *buf.add(2) = stat.mode as u64;
                *buf.add(3) = 1;
                *buf.add(4) = stat.uid as u64;
                *buf.add(5) = stat.gid as u64;
                *buf.add(6) = 0;
                *buf.add(7) = stat.size;
                *buf.add(8) = 512;
                *buf.add(9) = (stat.size + 511) / 512;
            }
            0
        }
        Err(_) => -2,
    }
}

pub fn compat_fstat(fd: i32, statbuf: *mut u8) -> isize {
    unsafe {
        let buf = statbuf as *mut u64;
        for i in 0..16 {
            *buf.add(i) = 0;
        }
        *buf.add(2) = 0o100644;
        *buf.add(7) = 4096;
    }
    0
}

pub fn compat_fcntl(fd: i32, cmd: i32, arg: usize) -> isize {
    match cmd {
        0 => 0,
        1 => 0,
        2 => 1,
        3 => 0,
        4 => 0,
        _ => 0,
    }
}

pub fn compat_openat(dirfd: i32, path: *const u8, flags: i32, mode: u32) -> isize {
    compat_open(path, flags, mode)
}

pub fn compat_fstatat(dirfd: i32, path: *const u8, statbuf: usize, flags: i32) -> isize {
    compat_stat(path, statbuf as *mut u8)
              }
