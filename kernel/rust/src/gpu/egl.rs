use alloc::vec::Vec;
use spin::Mutex;

static EGL_STATE: Mutex<EglState> = Mutex::new(EglState::new());

struct EglState {
    displays: Vec<EglDisplay>,
    contexts: Vec<EglContext>,
    surfaces: Vec<EglSurface>,
}

impl EglState {
    const fn new() -> Self {
        Self {
            displays: Vec::new(),
            contexts: Vec::new(),
            surfaces: Vec::new(),
        }
    }
}

struct EglDisplay {
    id: usize,
    native_display: usize,
}

struct EglContext {
    id: usize,
    display_id: usize,
    config: usize,
}

struct EglSurface {
    id: usize,
    display_id: usize,
    width: u32,
    height: u32,
    buffer: usize,
}

pub fn get_display(native_display: usize) -> usize {
    let mut state = EGL_STATE.lock();
    let id = state.displays.len() + 1;
    
    state.displays.push(EglDisplay {
        id,
        native_display,
    });
    
    id
}

pub fn initialize(display: usize, major: *mut i32, minor: *mut i32) -> bool {
    unsafe {
        if !major.is_null() {
            *major = 1;
        }
        if !minor.is_null() {
            *minor = 5;
        }
    }
    true
}

pub fn choose_config(display: usize, attribs: *const i32, configs: *mut usize, config_size: i32, num_config: *mut i32) -> bool {
    unsafe {
        if !configs.is_null() && config_size > 0 {
            *configs = 1;
        }
        if !num_config.is_null() {
            *num_config = 1;
        }
    }
    true
}

pub fn create_context(display: usize, config: usize, share_context: usize, attribs: *const i32) -> usize {
    let mut state = EGL_STATE.lock();
    let id = state.contexts.len() + 1;
    
    state.contexts.push(EglContext {
        id,
        display_id: display,
        config,
    });
    
    id
}

pub fn create_window_surface(display: usize, config: usize, native_window: usize, attribs: *const i32) -> usize {
    let mut state = EGL_STATE.lock();
    let id = state.surfaces.len() + 1;
    
    state.surfaces.push(EglSurface {
        id,
        display_id: display,
        width: 1024,
        height: 768,
        buffer: native_window,
    });
    
    id
}

pub fn make_current(display: usize, draw: usize, read: usize, context: usize) -> bool {
    true
}

pub fn swap_buffers(display: usize, surface: usize) -> bool {
    let state = EGL_STATE.lock();
    
    if let Some(surf) = state.surfaces.iter().find(|s| s.id == surface) {
        if surf.buffer != 0 {
            let fb_lock = crate::drivers::fb::get();
            let mut fb_opt = fb_lock.lock();
            if let Some(fb) = fb_opt.as_mut() {
                fb.swap();
            }
        }
    }
    
    true
}

pub fn get_proc_address(procname: *const u8) -> usize {
    0
}
