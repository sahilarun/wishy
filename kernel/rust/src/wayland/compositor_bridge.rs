use alloc::collections::BTreeMap;
use spin::Mutex;
use crate::gui::surface::Surface;
use crate::gui::compositor;

static WAYLAND_SURFACES: Mutex<BTreeMap<u32, WaylandSurfaceState>> = Mutex::new(BTreeMap::new());

struct WaylandSurfaceState {
    surface_id: u32,
    compositor_surface: Option<usize>,
    buffer_id: u32,
    width: u32,
    height: u32,
    pending_commit: bool,
}

pub fn create_surface(surface_id: u32) {
    let mut surfaces = WAYLAND_SURFACES.lock();
    surfaces.insert(surface_id, WaylandSurfaceState {
        surface_id,
        compositor_surface: None,
        buffer_id: 0,
        width: 800,
        height: 600,
        pending_commit: false,
    });
}

pub fn attach_buffer(surface_id: u32, buffer_id: u32, width: u32, height: u32) {
    let mut surfaces = WAYLAND_SURFACES.lock();
    if let Some(state) = surfaces.get_mut(&surface_id) {
        state.buffer_id = buffer_id;
        state.width = width;
        state.height = height;
        state.pending_commit = true;
    }
}

pub fn commit_surface(surface_id: u32) {
    let mut surfaces = WAYLAND_SURFACES.lock();
    
    if let Some(state) = surfaces.get_mut(&surface_id) {
        if state.pending_commit {
            if state.compositor_surface.is_none() {
                let mut surface = Surface::new(100, 100, state.width as usize, state.height as usize);
                surface.set_title("Chromium");
                
                state.compositor_surface = Some(0);
            }
            
            state.pending_commit = false;
        }
    }
}

pub fn destroy_surface(surface_id: u32) {
    let mut surfaces = WAYLAND_SURFACES.lock();
    surfaces.remove(&surface_id);
}

pub fn get_buffer_data(buffer_id: u32) -> Option<*const u32> {
    None
}
