use alloc::vec::Vec;
use spin::Mutex;

static DRM_STATE: Mutex<DrmState> = Mutex::new(DrmState::new());

struct DrmState {
    devices: Vec<DrmDevice>,
    framebuffers: Vec<DrmFramebuffer>,
}

impl DrmState {
    const fn new() -> Self {
        Self {
            devices: Vec::new(),
            framebuffers: Vec::new(),
        }
    }
}

struct DrmDevice {
    fd: i32,
    card_id: u32,
}

struct DrmFramebuffer {
    id: u32,
    width: u32,
    height: u32,
    pitch: u32,
    bpp: u32,
    handle: u32,
}

pub fn open_device() -> i32 {
    let mut state = DRM_STATE.lock();
    let fd = 100 + state.devices.len() as i32;
    
    state.devices.push(DrmDevice {
        fd,
        card_id: 0,
    });
    
    fd
}

pub fn create_dumb_buffer(fd: i32, width: u32, height: u32, bpp: u32) -> (u32, usize) {
    let pitch = width * (bpp / 8);
    let size = pitch * height;
    
    let handle = allocate_buffer_handle();
    (handle, size as usize)
}

pub fn map_dumb_buffer(fd: i32, handle: u32) -> usize {
    let addr = 0x60000000 + (handle as usize * 0x100000);
    addr
}

pub fn create_framebuffer(fd: i32, width: u32, height: u32, pitch: u32, bpp: u32, handle: u32) -> u32 {
    let mut state = DRM_STATE.lock();
    let fb_id = state.framebuffers.len() as u32 + 1;
    
    state.framebuffers.push(DrmFramebuffer {
        id: fb_id,
        width,
        height,
        pitch,
        bpp,
        handle,
    });
    
    fb_id
}

pub fn set_crtc(fd: i32, crtc_id: u32, fb_id: u32, x: u32, y: u32) -> i32 {
    0
}

pub fn page_flip(fd: i32, crtc_id: u32, fb_id: u32) -> i32 {
    let state = DRM_STATE.lock();
    
    if let Some(fb) = state.framebuffers.iter().find(|f| f.id == fb_id) {
        let addr = map_dumb_buffer(fd, fb.handle);
        crate::gpu::virtio_gpu::transfer_to_host(fb.handle, 0, 0, fb.width, fb.height);
    }
    
    0
}

fn allocate_buffer_handle() -> u32 {
    static mut NEXT_HANDLE: u32 = 1;
    unsafe {
        let handle = NEXT_HANDLE;
        NEXT_HANDLE += 1;
        handle
    }
}

pub fn ioctl_handler(fd: i32, request: u32, arg: usize) -> isize {
    const DRM_IOCTL_MODE_CREATE_DUMB: u32 = 0xC02064B2;
    const DRM_IOCTL_MODE_MAP_DUMB: u32 = 0xC01064B3;
    const DRM_IOCTL_MODE_ADDFB: u32 = 0xC06864AE;
    const DRM_IOCTL_MODE_SETCRTC: u32 = 0xC06864A2;
    const DRM_IOCTL_MODE_PAGE_FLIP: u32 = 0xC01064B0;
    
    match request {
        DRM_IOCTL_MODE_CREATE_DUMB => {
            unsafe {
                let args = arg as *mut DrmModeCreateDumb;
                (*args).handle = allocate_buffer_handle();
                (*args).pitch = (*args).width * ((*args).bpp / 8);
                (*args).size = ((*args).pitch * (*args).height) as u64;
            }
            0
        }
        DRM_IOCTL_MODE_MAP_DUMB => {
            unsafe {
                let args = arg as *mut DrmModeMapDumb;
                (*args).offset = 0x60000000 + ((*args).handle as u64 * 0x100000);
            }
            0
        }
        DRM_IOCTL_MODE_ADDFB => 0,
        DRM_IOCTL_MODE_SETCRTC => 0,
        DRM_IOCTL_MODE_PAGE_FLIP => 0,
        _ => -22,
    }
}

#[repr(C)]
struct DrmModeCreateDumb {
    height: u32,
    width: u32,
    bpp: u32,
    flags: u32,
    handle: u32,
    pitch: u32,
    size: u64,
}

#[repr(C)]
struct DrmModeMapDumb {
    handle: u32,
    pad: u32,
    offset: u64,
}
