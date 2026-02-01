use spin::Mutex;

const VIRTIO_GPU_BASE: u32 = 0xFEBD0000;

static GPU_STATE: Mutex<GpuState> = Mutex::new(GpuState::new());

struct GpuState {
    resources: [Resource; 256],
    next_resource_id: u32,
}

impl GpuState {
    const fn new() -> Self {
        Self {
            resources: [Resource::empty(); 256],
            next_resource_id: 1,
        }
    }
}

#[derive(Clone, Copy)]
struct Resource {
    id: u32,
    width: u32,
    height: u32,
    format: u32,
    backing: usize,
}

impl Resource {
    const fn empty() -> Self {
        Self {
            id: 0,
            width: 0,
            height: 0,
            format: 0,
            backing: 0,
        }
    }
}

pub fn init() {
    let mut state = GPU_STATE.lock();
    
    mmio_write(VIRTIO_GPU_BASE, 0);
    mmio_write(VIRTIO_GPU_BASE + 4, 1);
    mmio_write(VIRTIO_GPU_BASE + 8, 3);
}

pub fn create_resource_2d(width: u32, height: u32, format: u32) -> u32 {
    let mut state = GPU_STATE.lock();
    let id = state.next_resource_id;
    state.next_resource_id += 1;
    
    if (id as usize) < state.resources.len() {
        state.resources[id as usize] = Resource {
            id,
            width,
            height,
            format,
            backing: 0,
        };
    }
    
    id
}

pub fn attach_backing(resource_id: u32, addr: usize, size: usize) {
    let mut state = GPU_STATE.lock();
    
    if let Some(res) = state.resources.iter_mut().find(|r| r.id == resource_id) {
        res.backing = addr;
    }
}

pub fn transfer_to_host(resource_id: u32, x: u32, y: u32, width: u32, height: u32) {
    let state = GPU_STATE.lock();
    
    if let Some(res) = state.resources.iter().find(|r| r.id == resource_id) {
        if res.backing != 0 {
            let fb_lock = crate::drivers::fb::get();
            let mut fb_opt = fb_lock.lock();
            if let Some(fb) = fb_opt.as_mut() {
                let src = unsafe {
                    core::slice::from_raw_parts(
                        res.backing as *const u32,
                        (width * height) as usize
                    )
                };
                fb.blit(x as usize, y as usize, width as usize, height as usize, src);
            }
        }
    }
}

pub fn set_scanout(scanout_id: u32, resource_id: u32, x: u32, y: u32, width: u32, height: u32) {
}

pub fn flush(resource_id: u32, x: u32, y: u32, width: u32, height: u32) {
    transfer_to_host(resource_id, x, y, width, height);
}

fn mmio_write(addr: u32, value: u32) {
    unsafe {
        core::ptr::write_volatile(addr as *mut u32, value);
    }
}

fn mmio_read(addr: u32) -> u32 {
    unsafe {
        core::ptr::read_volatile(addr as *const u32)
    }
                     }
