use alloc::collections::BTreeMap;
use spin::Mutex;

static SIGNAL_HANDLERS: Mutex<BTreeMap<u32, SignalHandler>> = Mutex::new(BTreeMap::new());
static SIGNAL_MASKS: Mutex<BTreeMap<u32, u64>> = Mutex::new(BTreeMap::new());

struct SignalHandler {
    handler: usize,
    flags: i32,
    mask: u64,
}

pub fn set_signal_handler(signum: i32, act: usize, oldact: usize) -> isize {
    let pid = crate::compat::process::get_current_pid();
    let mut handlers = SIGNAL_HANDLERS.lock();
    
    if oldact != 0 {
        if let Some(old) = handlers.get(&((pid << 8) | signum as u32)) {
            unsafe {
                let ptr = oldact as *mut usize;
                *ptr = old.handler;
                *ptr.add(1) = old.flags as usize;
                *ptr.add(2) = old.mask as usize;
            }
        }
    }
    
    if act != 0 {
        unsafe {
            let ptr = act as *const usize;
            let handler = *ptr;
            let flags = *ptr.add(1) as i32;
            let mask = *ptr.add(2) as u64;
            
            handlers.insert(
                (pid << 8) | signum as u32,
                SignalHandler { handler, flags, mask }
            );
        }
    }
    
    0
}

pub fn modify_signal_mask(how: i32, set: usize, oldset: usize) -> isize {
    let pid = crate::compat::process::get_current_pid();
    let mut masks = SIGNAL_MASKS.lock();
    
    let current_mask = masks.get(&pid).copied().unwrap_or(0);
    
    if oldset != 0 {
        unsafe {
            *(oldset as *mut u64) = current_mask;
        }
    }
    
    if set != 0 {
        let new_mask = unsafe { *(set as *const u64) };
        
        let updated_mask = match how {
            0 => current_mask & !new_mask,
            1 => current_mask | new_mask,
            2 => new_mask,
            _ => current_mask,
        };
        
        masks.insert(pid, updated_mask);
    }
    
    0
}

pub fn send_signal(pid: u32, signum: i32) -> isize {
    let handlers = SIGNAL_HANDLERS.lock();
    
    if let Some(handler) = handlers.get(&((pid << 8) | signum as u32)) {
        if handler.handler != 0 && handler.handler != 1 {
            unsafe {
                let handler_fn: extern "C" fn(i32) = core::mem::transmute(handler.handler);
                handler_fn(signum);
            }
        }
    }
    
    0
      }
