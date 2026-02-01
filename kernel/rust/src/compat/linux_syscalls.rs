use crate::compat::{process, ipc, signal, threading, vfs_bridge};
use crate::syscall;
use alloc::vec::Vec;

pub fn linux_syscall_handler(num: u32, args: &[usize; 6]) -> isize {
    match num {
        0 => sys_read(args[0] as i32, args[1] as *mut u8, args[2]),
        1 => sys_write(args[0] as i32, args[1] as *const u8, args[2]),
        2 => sys_open(args[0] as *const u8, args[1] as i32, args[2] as u32),
        3 => sys_close(args[0] as i32),
        4 => sys_stat(args[0] as *const u8, args[1] as *mut u8),
        5 => sys_fstat(args[0] as i32, args[1] as *mut u8),
        9 => sys_mmap(args[0], args[1], args[2] as i32, args[3] as i32, args[4] as i32, args[5]),
        11 => sys_munmap(args[0], args[1]),
        13 => sys_rt_sigaction(args[0] as i32, args[1], args[2]),
        14 => sys_rt_sigprocmask(args[0] as i32, args[1], args[2]),
        24 => sys_sched_yield(),
        39 => sys_getpid(),
        41 => sys_socket(args[0] as i32, args[1] as i32, args[2] as i32),
        42 => sys_connect(args[0] as i32, args[1], args[2]),
        43 => sys_accept(args[0] as i32, args[1], args[2]),
        44 => sys_sendto(args[0] as i32, args[1], args[2], args[3] as i32, args[4], args[5]),
        45 => sys_recvfrom(args[0] as i32, args[1], args[2], args[3] as i32, args[4], args[5]),
        49 => sys_bind(args[0] as i32, args[1], args[2]),
        50 => sys_listen(args[0] as i32, args[1] as i32),
        56 => sys_clone(args[0] as u32, args[1], args[2], args[3], args[4]),
        57 => sys_fork(),
        59 => sys_execve(args[0] as *const u8, args[1], args[2]),
        60 => sys_exit(args[0] as i32),
        61 => sys_wait4(args[0] as i32, args[1], args[2] as i32, args[3]),
        72 => sys_fcntl(args[0] as i32, args[1] as i32, args[2]),
        79 => sys_getcwd(args[0] as *mut u8, args[1]),
        80 => sys_chdir(args[0] as *const u8),
        102 => sys_getuid(),
        104 => sys_getgid(),
        110 => sys_getppid(),
        186 => sys_gettid(),
        202 => sys_futex(args[0], args[1] as i32, args[2] as i32, args[3], args[4], args[5] as i32),
        218 => sys_set_tid_address(args[0]),
        228 => sys_clock_gettime(args[0] as i32, args[1]),
        231 => sys_exit_group(args[0] as i32),
        257 => sys_openat(args[0] as i32, args[1] as *const u8, args[2] as i32, args[3] as u32),
        262 => sys_newfstatat(args[0] as i32, args[1] as *const u8, args[2], args[3] as i32),
        280 => sys_utimensat(args[0] as i32, args[1] as *const u8, args[2], args[3] as i32),
        281 => sys_epoll_pwait(args[0] as i32, args[1], args[2] as i32, args[3] as i32, args[4]),
        318 => sys_getrandom(args[0], args[1], args[2] as u32),
        _ => -38,
    }
}

fn sys_read(fd: i32, buf: *mut u8, count: usize) -> isize {
    vfs_bridge::compat_read(fd, buf, count)
}

fn sys_write(fd: i32, buf: *const u8, count: usize) -> isize {
    vfs_bridge::compat_write(fd, buf, count)
}

fn sys_open(path: *const u8, flags: i32, mode: u32) -> isize {
    vfs_bridge::compat_open(path, flags, mode)
}

fn sys_close(fd: i32) -> isize {
    vfs_bridge::compat_close(fd)
}

fn sys_stat(path: *const u8, statbuf: *mut u8) -> isize {
    vfs_bridge::compat_stat(path, statbuf)
}

fn sys_fstat(fd: i32, statbuf: *mut u8) -> isize {
    vfs_bridge::compat_fstat(fd, statbuf)
}

fn sys_mmap(addr: usize, length: usize, prot: i32, flags: i32, fd: i32, offset: usize) -> isize {
    match crate::memory::mmap::mmap(addr, length, prot, flags, fd, offset) {
        Ok(a) => a as isize,
        Err(_) => -12,
    }
}

fn sys_munmap(addr: usize, length: usize) -> isize {
    match crate::memory::mmap::munmap(addr, length) {
        Ok(_) => 0,
        Err(_) => -22,
    }
}

fn sys_rt_sigaction(signum: i32, act: usize, oldact: usize) -> isize {
    signal::set_signal_handler(signum, act, oldact)
}

fn sys_rt_sigprocmask(how: i32, set: usize, oldset: usize) -> isize {
    signal::modify_signal_mask(how, set, oldset)
}

fn sys_sched_yield() -> isize {
    threading::yield_cpu();
    0
}

fn sys_getpid() -> isize {
    process::get_current_pid() as isize
}

fn sys_socket(domain: i32, sock_type: i32, protocol: i32) -> isize {
    ipc::create_socket(domain, sock_type, protocol)
}

fn sys_connect(sockfd: i32, addr: usize, addrlen: usize) -> isize {
    ipc::connect_socket(sockfd, addr, addrlen)
}

fn sys_accept(sockfd: i32, addr: usize, addrlen: usize) -> isize {
    ipc::accept_socket(sockfd, addr, addrlen)
}

fn sys_sendto(sockfd: i32, buf: usize, len: usize, flags: i32, dest_addr: usize, addrlen: usize) -> isize {
    ipc::send_to(sockfd, buf, len, flags, dest_addr, addrlen)
}

fn sys_recvfrom(sockfd: i32, buf: usize, len: usize, flags: i32, src_addr: usize, addrlen: usize) -> isize {
    ipc::recv_from(sockfd, buf, len, flags, src_addr, addrlen)
}

fn sys_bind(sockfd: i32, addr: usize, addrlen: usize) -> isize {
    ipc::bind_socket(sockfd, addr, addrlen)
}

fn sys_listen(sockfd: i32, backlog: i32) -> isize {
    ipc::listen_socket(sockfd, backlog)
}

fn sys_clone(flags: u32, stack: usize, ptid: usize, ctid: usize, tls: usize) -> isize {
    process::clone_process(flags, stack, ptid, ctid, tls)
}

fn sys_fork() -> isize {
    process::fork_process()
}

fn sys_execve(filename: *const u8, argv: usize, envp: usize) -> isize {
    process::execute_binary(filename, argv, envp)
}

fn sys_exit(code: i32) -> isize {
    process::exit_process(code);
    0
}

fn sys_wait4(pid: i32, status: usize, options: i32, rusage: usize) -> isize {
    process::wait_for_child(pid, status, options, rusage)
}

fn sys_fcntl(fd: i32, cmd: i32, arg: usize) -> isize {
    vfs_bridge::compat_fcntl(fd, cmd, arg)
}

fn sys_getcwd(buf: *mut u8, size: usize) -> isize {
    process::get_current_dir(buf, size)
}

fn sys_chdir(path: *const u8) -> isize {
    process::change_dir(path)
}

fn sys_getuid() -> isize {
    process::get_current_uid() as isize
}

fn sys_getgid() -> isize {
    process::get_current_gid() as isize
}

fn sys_getppid() -> isize {
    process::get_parent_pid() as isize
}

fn sys_gettid() -> isize {
    threading::get_thread_id() as isize
}

fn sys_futex(uaddr: usize, op: i32, val: i32, timeout: usize, uaddr2: usize, val3: i32) -> isize {
    threading::futex_wait_wake(uaddr, op, val, timeout, uaddr2, val3)
}

fn sys_set_tid_address(tidptr: usize) -> isize {
    threading::set_tid_address(tidptr)
}

fn sys_clock_gettime(clockid: i32, tp: usize) -> isize {
    let ticks = get_system_ticks();
    unsafe {
        let ptr = tp as *mut u64;
        *ptr = ticks / 1000;
        *ptr.add(1) = (ticks % 1000) * 1000000;
    }
    0
}

fn sys_exit_group(status: i32) -> isize {
    process::exit_group(status);
    0
}

fn sys_openat(dirfd: i32, path: *const u8, flags: i32, mode: u32) -> isize {
    vfs_bridge::compat_openat(dirfd, path, flags, mode)
}

fn sys_newfstatat(dirfd: i32, path: *const u8, statbuf: usize, flags: i32) -> isize {
    vfs_bridge::compat_fstatat(dirfd, path, statbuf, flags)
}

fn sys_utimensat(dirfd: i32, path: *const u8, times: usize, flags: i32) -> isize {
    0
}

fn sys_epoll_pwait(epfd: i32, events: usize, maxevents: i32, timeout: i32, sigmask: usize) -> isize {
    ipc::epoll_wait(epfd, events, maxevents, timeout)
}

fn sys_getrandom(buf: usize, buflen: usize, flags: u32) -> isize {
    unsafe {
        let ptr = buf as *mut u8;
        for i in 0..buflen {
            *ptr.add(i) = (get_system_ticks() & 0xFF) as u8;
        }
    }
    buflen as isize
}

fn get_system_ticks() -> u64 {
    static mut TICKS: u64 = 0;
    unsafe {
        TICKS += 1;
        TICKS
    }
  }
