use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use spin::Mutex;

static THREAD_TABLE: Mutex<ThreadTable> = Mutex::new(ThreadTable::new());
static FUTEX_TABLE: Mutex<BTreeMap<usize, Vec<u32>>> = Mutex::new(BTreeMap::new());

struct ThreadTable {
    threads: Vec<Thread>,
    next_tid: u32,
}

impl ThreadTable {
    const fn new() -> Self {
        Self {
            threads: Vec::new(),
            next_tid: 1,
        }
    }
}

struct Thread {
    tid: u32,
    pid: u32,
    state: ThreadState,
    tid_address: usize,
}

enum ThreadState {
    Running,
    Blocked,
    Dead,
}

pub fn get_thread_id() -> u32 {
    let table = THREAD_TABLE.lock();
    table.threads.last().map(|t| t.tid).unwrap_or(1)
}

pub fn yield_cpu() {
    for _ in 0..1000 {
        core::hint::spin_loop();
    }
}

pub fn futex_wait_wake(uaddr: usize, op: i32, val: i32, timeout: usize, uaddr2: usize, val3: i32) -> isize {
    let futex_wait = 0;
    let futex_wake = 1;
    let futex_private = 128;
    
    let op_masked = op & !futex_private;
    
    match op_masked {
        op if op == futex_wait => {
            let current = unsafe { *(uaddr as *const i32) };
            if current != val {
                return -11;
            }
            
            let tid = get_thread_id();
            let mut table = FUTEX_TABLE.lock();
            table.entry(uaddr).or_insert_with(Vec::new).push(tid);
            
            for _ in 0..10000 {
                core::hint::spin_loop();
            }
            
            0
        }
        op if op == futex_wake => {
            let mut table = FUTEX_TABLE.lock();
            if let Some(waiters) = table.get_mut(&uaddr) {
                let woken = waiters.len().min(val as usize);
                waiters.drain(..woken);
                return woken as isize;
            }
            0
        }
        _ => -38,
    }
}

pub fn set_tid_address(tidptr: usize) -> isize {
    let mut table = THREAD_TABLE.lock();
    let tid = get_thread_id();
    
    if let Some(thread) = table.threads.iter_mut().find(|t| t.tid == tid) {
        thread.tid_address = tidptr;
    } else {
        table.threads.push(Thread {
            tid,
            pid: crate::compat::process::get_current_pid(),
            state: ThreadState::Running,
            tid_address: tidptr,
        });
    }
    
    tid as isize
}
