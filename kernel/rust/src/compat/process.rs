use alloc::vec::Vec;
use alloc::string::String;
use spin::Mutex;
use crate::exec::loader;
use crate::memory::paging;

static PROCESS_TABLE: Mutex<ProcessTable> = Mutex::new(ProcessTable::new());

struct ProcessTable {
    processes: Vec<Process>,
    next_pid: u32,
}

impl ProcessTable {
    const fn new() -> Self {
        Self {
            processes: Vec::new(),
            next_pid: 1,
        }
    }
}

struct Process {
    pid: u32,
    ppid: u32,
    uid: u32,
    gid: u32,
    state: ProcessState,
    exit_code: i32,
    cwd: [u8; 256],
    cwd_len: usize,
}

enum ProcessState {
    Running,
    Sleeping,
    Zombie,
}

pub fn clone_process(flags: u32, stack: usize, ptid: usize, ctid: usize, tls: usize) -> isize {
    let mut table = PROCESS_TABLE.lock();
    let current_pid = get_current_pid_internal(&table);
    
    let new_pid = table.next_pid;
    table.next_pid += 1;
    
    let current = table.processes.iter().find(|p| p.pid == current_pid);
    let (ppid, uid, gid, cwd, cwd_len) = if let Some(p) = current {
        (p.pid, p.uid, p.gid, p.cwd, p.cwd_len)
    } else {
        (1, 0, 0, [0; 256], 1)
    };
    
    table.processes.push(Process {
        pid: new_pid,
        ppid,
        uid,
        gid,
        state: ProcessState::Running,
        exit_code: 0,
        cwd,
        cwd_len,
    });
    
    if (flags & 0x00010000) != 0 {
        unsafe {
            *(ptid as *mut u32) = new_pid;
        }
    }
    
    if (flags & 0x00200000) != 0 {
        unsafe {
            *(ctid as *mut u32) = new_pid;
        }
    }
    
    new_pid as isize
}

pub fn fork_process() -> isize {
    clone_process(0x01200011, 0, 0, 0, 0)
}

pub fn execute_binary(filename: *const u8, argv: usize, envp: usize) -> isize {
    let mut path = [0u8; 256];
    let mut len = 0;
    
    unsafe {
        while len < 255 && *filename.add(len) != 0 {
            path[len] = *filename.add(len);
            len += 1;
        }
    }
    
    let path_str = core::str::from_utf8(&path[..len]).unwrap_or("");
    
    if let Ok(data) = crate::fs::mount::read_file(path_str) {
        if let Ok(entry) = loader::load_elf(&data) {
            unsafe {
                let entry_fn: extern "C" fn() = core::mem::transmute(entry as usize);
                entry_fn();
            }
        }
    }
    
    -2
}

pub fn exit_process(code: i32) {
    let mut table = PROCESS_TABLE.lock();
    let pid = get_current_pid_internal(&table);
    
    if let Some(proc) = table.processes.iter_mut().find(|p| p.pid == pid) {
        proc.state = ProcessState::Zombie;
        proc.exit_code = code;
    }
}

pub fn exit_group(status: i32) {
    exit_process(status);
}

pub fn wait_for_child(pid: i32, status: usize, options: i32, rusage: usize) -> isize {
    let mut table = PROCESS_TABLE.lock();
    
    let target_pid = if pid == -1 {
        table.processes.iter()
            .find(|p| matches!(p.state, ProcessState::Zombie))
            .map(|p| p.pid)
    } else {
        Some(pid as u32)
    };
    
    if let Some(child_pid) = target_pid {
        if let Some(idx) = table.processes.iter().position(|p| p.pid == child_pid) {
            let proc = &table.processes[idx];
            let exit_code = proc.exit_code;
            
            if status != 0 {
                unsafe {
                    *(status as *mut i32) = (exit_code & 0xFF) << 8;
                }
            }
            
            table.processes.remove(idx);
            return child_pid as isize;
        }
    }
    
    if (options & 1) != 0 {
        return 0;
    }
    
    -10
}

pub fn get_current_pid() -> u32 {
    let table = PROCESS_TABLE.lock();
    get_current_pid_internal(&table)
}

fn get_current_pid_internal(table: &ProcessTable) -> u32 {
    table.processes.last().map(|p| p.pid).unwrap_or(1)
}

pub fn get_parent_pid() -> u32 {
    let table = PROCESS_TABLE.lock();
    let pid = get_current_pid_internal(&table);
    table.processes.iter()
        .find(|p| p.pid == pid)
        .map(|p| p.ppid)
        .unwrap_or(0)
}

pub fn get_current_uid() -> u32 {
    let table = PROCESS_TABLE.lock();
    let pid = get_current_pid_internal(&table);
    table.processes.iter()
        .find(|p| p.pid == pid)
        .map(|p| p.uid)
        .unwrap_or(0)
}

pub fn get_current_gid() -> u32 {
    let table = PROCESS_TABLE.lock();
    let pid = get_current_pid_internal(&table);
    table.processes.iter()
        .find(|p| p.pid == pid)
        .map(|p| p.gid)
        .unwrap_or(0)
}

pub fn get_current_dir(buf: *mut u8, size: usize) -> isize {
    let table = PROCESS_TABLE.lock();
    let pid = get_current_pid_internal(&table);
    
    if let Some(proc) = table.processes.iter().find(|p| p.pid == pid) {
        let copy_len = proc.cwd_len.min(size);
        unsafe {
            core::ptr::copy_nonoverlapping(proc.cwd.as_ptr(), buf, copy_len);
        }
        return buf as isize;
    }
    
    -14
}

pub fn change_dir(path: *const u8) -> isize {
    let mut table = PROCESS_TABLE.lock();
    let pid = get_current_pid_internal(&table);
    
    if let Some(proc) = table.processes.iter_mut().find(|p| p.pid == pid) {
        let mut len = 0;
        unsafe {
            while len < 255 && *path.add(len) != 0 {
                proc.cwd[len] = *path.add(len);
                len += 1;
            }
        }
        proc.cwd_len = len;
        return 0;
    }
    
    -14
}
