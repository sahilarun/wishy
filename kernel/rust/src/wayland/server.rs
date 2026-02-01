use alloc::vec;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use spin::Mutex;
use crate::wayland::protocol::*;
use crate::wayland::compositor_bridge;

static WAYLAND_SERVER: Mutex<WaylandServer> = Mutex::new(WaylandServer::new());

pub struct WaylandServer {
    clients: Vec<Client>,
    objects: BTreeMap<u32, WaylandObject>,
    next_object_id: u32,
    socket_path: [u8; 128],
}

impl WaylandServer {
    const fn new() -> Self {
        Self {
            clients: Vec::new(),
            objects: BTreeMap::new(),
            next_object_id: 2,
            socket_path: [0; 128],
        }
    }
}

struct Client {
    fd: i32,
    objects: Vec<u32>,
}

enum WaylandObject {
    Display,
    Registry,
    Compositor,
    Shm,
    Surface { width: u32, height: u32, buffer_id: u32 },
    ShmPool { fd: i32, size: usize, offset: usize },
    Buffer { width: u32, height: u32, stride: u32, format: u32, data: usize },
    Seat,
    Pointer,
    Keyboard,
    XdgWmBase,
    XdgSurface { surface_id: u32 },
    XdgToplevel { surface_id: u32 },
}

pub fn init() {
    let socket_path = b"/tmp/wayland-0";
    let mut server = WAYLAND_SERVER.lock();
    server.socket_path[..socket_path.len()].copy_from_slice(socket_path);
    
    server.objects.insert(1, WaylandObject::Display);
}

pub fn accept_client(fd: i32) {
    let mut server = WAYLAND_SERVER.lock();
    server.clients.push(Client {
        fd,
        objects: vec![1],
    });
}

pub fn handle_message(fd: i32, data: &[u8]) -> Option<Vec<u8>> {
    let (msg, payload) = parse_message(data)?;
    
    let mut server = WAYLAND_SERVER.lock();
    
    match server.objects.get(&msg.object_id)? {
        WaylandObject::Display => handle_display(&mut server, fd, msg.opcode, payload),
        WaylandObject::Registry => handle_registry(&mut server, fd, msg.opcode, payload),
        WaylandObject::Compositor => handle_compositor(&mut server, fd, msg.opcode, payload),
        WaylandObject::Shm => handle_shm(&mut server, fd, msg.opcode, payload),
        WaylandObject::Surface { .. } => handle_surface(&mut server, fd, msg.object_id, msg.opcode, payload),
        WaylandObject::ShmPool { .. } => handle_shm_pool(&mut server, fd, msg.object_id, msg.opcode, payload),
        WaylandObject::XdgWmBase => handle_xdg_wm_base(&mut server, fd, msg.opcode, payload),
        WaylandObject::XdgSurface { .. } => handle_xdg_surface(&mut server, fd, msg.object_id, msg.opcode, payload),
        WaylandObject::XdgToplevel { .. } => handle_xdg_toplevel(&mut server, fd, msg.object_id, msg.opcode, payload),
        _ => None,
    }
}

fn handle_display(server: &mut WaylandServer, fd: i32, opcode: u16, payload: &[u8]) -> Option<Vec<u8>> {
    match opcode {
        0 => {
            let registry_id = server.next_object_id;
            server.next_object_id += 1;
            server.objects.insert(registry_id, WaylandObject::Registry);
            
            if let Some(client) = server.clients.iter_mut().find(|c| c.fd == fd) {
                client.objects.push(registry_id);
            }
            
            None
        }
        1 => None,
        _ => None,
    }
}

fn handle_registry(server: &mut WaylandServer, fd: i32, opcode: u16, payload: &[u8]) -> Option<Vec<u8>> {
    match opcode {
        0 => {
            if payload.len() < 12 {
                return None;
            }
            
            let name = u32::from_ne_bytes([payload[0], payload[1], payload[2], payload[3]]);
            let new_id = u32::from_ne_bytes([payload[8], payload[9], payload[10], payload[11]]);
            
            let (interface, _) = decode_string(&payload[4..])?;
            
            let object = match interface {
                "wl_compositor" => WaylandObject::Compositor,
                "wl_shm" => WaylandObject::Shm,
                "wl_seat" => WaylandObject::Seat,
                "xdg_wm_base" => WaylandObject::XdgWmBase,
                _ => return None,
            };
            
            server.objects.insert(new_id, object);
            None
        }
        _ => None,
    }
}

fn handle_compositor(server: &mut WaylandServer, fd: i32, opcode: u16, payload: &[u8]) -> Option<Vec<u8>> {
    match opcode {
        0 => {
            if payload.len() < 4 {
                return None;
            }
            
            let surface_id = u32::from_ne_bytes([payload[0], payload[1], payload[2], payload[3]]);
            server.objects.insert(surface_id, WaylandObject::Surface {
                width: 0,
                height: 0,
                buffer_id: 0,
            });
            
            compositor_bridge::create_surface(surface_id);
            None
        }
        _ => None,
    }
}

fn handle_shm(server: &mut WaylandServer, fd: i32, opcode: u16, payload: &[u8]) -> Option<Vec<u8>> {
    match opcode {
        0 => {
            if payload.len() < 12 {
                return None;
            }
            
            let pool_id = u32::from_ne_bytes([payload[0], payload[1], payload[2], payload[3]]);
            let shm_fd = i32::from_ne_bytes([payload[4], payload[5], payload[6], payload[7]]);
            let size = i32::from_ne_bytes([payload[8], payload[9], payload[10], payload[11]]) as usize;
            
            server.objects.insert(pool_id, WaylandObject::ShmPool {
                fd: shm_fd,
                size,
                offset: 0,
            });
            
            None
        }
        _ => None,
    }
}

fn handle_surface(server: &mut WaylandServer, fd: i32, surface_id: u32, opcode: u16, payload: &[u8]) -> Option<Vec<u8>> {
    match opcode {
        1 => {
            if payload.len() < 4 {
                return None;
            }
            
            let buffer_id = u32::from_ne_bytes([payload[0], payload[1], payload[2], payload[3]]);
            
            if let Some(WaylandObject::Surface { buffer_id: buf, .. }) = server.objects.get_mut(&surface_id) {
                *buf = buffer_id;
            }
            
            None
        }
        6 => {
            compositor_bridge::commit_surface(surface_id);
            None
        }
        _ => None,
    }
}

fn handle_shm_pool(server: &mut WaylandServer, fd: i32, pool_id: u32, opcode: u16, payload: &[u8]) -> Option<Vec<u8>> {
    match opcode {
        0 => {
            if payload.len() < 20 {
                return None;
            }
            
            let buffer_id = u32::from_ne_bytes([payload[0], payload[1], payload[2], payload[3]]);
            let offset = i32::from_ne_bytes([payload[4], payload[5], payload[6], payload[7]]) as usize;
            let width = i32::from_ne_bytes([payload[8], payload[9], payload[10], payload[11]]) as u32;
            let height = i32::from_ne_bytes([payload[12], payload[13], payload[14], payload[15]]) as u32;
            let stride = i32::from_ne_bytes([payload[16], payload[17], payload[18], payload[19]]) as u32;
            
            server.objects.insert(buffer_id, WaylandObject::Buffer {
                width,
                height,
                stride,
                format: 0,
                data: offset,
            });
            
            None
        }
        _ => None,
    }
}

fn handle_xdg_wm_base(server: &mut WaylandServer, fd: i32, opcode: u16, payload: &[u8]) -> Option<Vec<u8>> {
    match opcode {
        2 => {
            if payload.len() < 8 {
                return None;
            }
            
            let xdg_surface_id = u32::from_ne_bytes([payload[0], payload[1], payload[2], payload[3]]);
            let surface_id = u32::from_ne_bytes([payload[4], payload[5], payload[6], payload[7]]);
            
            server.objects.insert(xdg_surface_id, WaylandObject::XdgSurface { surface_id });
            None
        }
        _ => None,
    }
}

fn handle_xdg_surface(server: &mut WaylandServer, fd: i32, xdg_surface_id: u32, opcode: u16, payload: &[u8]) -> Option<Vec<u8>> {
    match opcode {
        1 => {
            if payload.len() < 4 {
                return None;
            }
            
            let toplevel_id = u32::from_ne_bytes([payload[0], payload[1], payload[2], payload[3]]);
            
            if let Some(WaylandObject::XdgSurface { surface_id }) = server.objects.get(&xdg_surface_id) {
                server.objects.insert(toplevel_id, WaylandObject::XdgToplevel { surface_id: *surface_id });
            }
            
            None
        }
        _ => None,
    }
}

fn handle_xdg_toplevel(server: &mut WaylandServer, fd: i32, toplevel_id: u32, opcode: u16, payload: &[u8]) -> Option<Vec<u8>> {
    None
  }
