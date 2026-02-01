use crate::fs::mount;
use crate::memory::mmap;
use alloc::vec::Vec;
use spin::Mutex;

const MAX_FDS: usize = 256;

struct FileDescriptor {
    path: Vec<u8>,
    offset: usize,
    flags: i32,
}

static FD_TABLE: Mutex<Vec<Option<FileDescriptor>>> = Mutex::new(Vec::new());

pub fn init() {
    let mut table = FD_TABLE.lock();
    for _ in 0..MAX_FDS {
        table.push(None);
    }
}

#[no_mangle]
pub extern "C" fn syscall_handler(num: u32, arg1: u32, arg2: u32, arg3: u32, arg4: u32) -> i32 {
    let args = [arg1 as usize, arg2 as usize, arg3 as usize, arg4 as usize, 0, 0];
    
    if num >= 100 {
        crate::compat::linux_syscalls::linux_syscall_handler(num, &args) as i32
    } else {
        match num {
            0 => sys_read(arg1 as i32, arg2 as *mut u8, arg3 as usize),
            1 => sys_write(arg1 as i32, arg2 as *const u8, arg3 as usize),
            2 => sys_open(arg1 as *const u8, arg2 as i32),
            3 => sys_close(arg1 as i32),
            9 => sys_mmap(arg1 as usize, arg2 as usize, arg3 as i32, arg4 as i32, 0, 0),
            60 => sys_exit(arg1 as i32),
            _ => crate::compat::linux_syscalls::linux_syscall_handler(num, &args) as i32,
        }
    }
}

fn sys_read(fd: i32, buf: *mut u8, count: usize) -> i32 {
    if fd < 0 || fd >= MAX_FDS as i32 {
        return -1;
    }
    
    let table = FD_TABLE.lock();
    if let Some(Some(file)) = table.get(fd as usize) {
        let path = core::str::from_utf8(&file.path).unwrap_or("");
        if let Ok(data) = mount::read_file(path) {
            let to_read = count.min(data.len() - file.offset);
            unsafe {
                core::ptr::copy_nonoverlapping(
                    data[file.offset..].as_ptr(),
                    buf,
                    to_read
                );
            }
            return to_read as i32;
        }
    }
    
    -1
}

fn sys_write(fd: i32, buf: *const u8, count: usize) -> i32 {
    if fd < 0 || fd >= MAX_FDS as i32 {
        return -1;
    }
    
    let table = FD_TABLE.lock();
    if let Some(Some(file)) = table.get(fd as usize) {
        let path = core::str::from_utf8(&file.path).unwrap_or("");
        let data = unsafe { core::slice::from_raw_parts(buf, count) };
        if mount::write_file(path, data).is_ok() {
            return count as i32;
        }
    }
    
    -1
}

fn sys_open(path: *const u8, flags: i32) -> i32 {
    let mut len = 0;
    unsafe {
        while len < 4096 && *path.add(len) != 0 {
            len += 1;
        }
    }
    
    let path_slice = unsafe { core::slice::from_raw_parts(path, len) };
    let mut table = FD_TABLE.lock();
    
    for (i, slot) in table.iter_mut().enumerate() {
        if slot.is_none() {
            *slot = Some(FileDescriptor {
                path: path_slice.to_vec(),
                offset: 0,
                flags,
            });
            return i as i32;
        }
    }
    
    -1
}

fn sys_close(fd: i32) -> i32 {
    if fd < 0 || fd >= MAX_FDS as i32 {
        return -1;
    }
    
    let mut table = FD_TABLE.lock();
    if let Some(slot) = table.get_mut(fd as usize) {
        *slot = None;
        return 0;
    }
    
    -1
}

fn sys_mmap(addr: usize, length: usize, prot: i32, flags: i32, _fd: i32, _offset: usize) -> i32 {
    match mmap::mmap(addr, length, prot, flags, -1, 0) {
        Ok(addr) => addr as i32,
        Err(_) => -1,
    }
}

fn sys_exit(_code: i32) -> i32 {
    loop {}
                }
