pub const WL_DISPLAY_INTERFACE: u32 = 1;
pub const WL_REGISTRY_INTERFACE: u32 = 2;
pub const WL_COMPOSITOR_INTERFACE: u32 = 3;
pub const WL_SHM_INTERFACE: u32 = 4;
pub const WL_SURFACE_INTERFACE: u32 = 5;
pub const WL_REGION_INTERFACE: u32 = 6;
pub const WL_SHM_POOL_INTERFACE: u32 = 7;
pub const WL_BUFFER_INTERFACE: u32 = 8;
pub const WL_SEAT_INTERFACE: u32 = 9;
pub const WL_POINTER_INTERFACE: u32 = 10;
pub const WL_KEYBOARD_INTERFACE: u32 = 11;
pub const XDG_WM_BASE_INTERFACE: u32 = 12;
pub const XDG_SURFACE_INTERFACE: u32 = 13;
pub const XDG_TOPLEVEL_INTERFACE: u32 = 14;

#[repr(C)]
pub struct WaylandMessage {
    pub object_id: u32,
    pub opcode: u16,
    pub size: u16,
}

pub fn parse_message(data: &[u8]) -> Option<(WaylandMessage, &[u8])> {
    if data.len() < 8 {
        return None;
    }
    
    let object_id = u32::from_ne_bytes([data[0], data[1], data[2], data[3]]);
    let opcode = u16::from_ne_bytes([data[4], data[5]]);
    let size = u16::from_ne_bytes([data[6], data[7]]);
    
    if data.len() < size as usize {
        return None;
    }
    
    let msg = WaylandMessage {
        object_id,
        opcode,
        size,
    };
    
    let payload = &data[8..size as usize];
    Some((msg, payload))
}

pub fn create_message(object_id: u32, opcode: u16, payload: &[u8]) -> alloc::vec::Vec<u8> {
    let size = 8 + payload.len();
    let mut msg = alloc::vec::Vec::with_capacity(size);
    
    msg.extend_from_slice(&object_id.to_ne_bytes());
    msg.extend_from_slice(&opcode.to_ne_bytes());
    msg.extend_from_slice(&(size as u16).to_ne_bytes());
    msg.extend_from_slice(payload);
    
    msg
}

pub fn encode_string(s: &str) -> alloc::vec::Vec<u8> {
    let len = s.len() as u32;
    let mut encoded = alloc::vec::Vec::new();
    
    encoded.extend_from_slice(&len.to_ne_bytes());
    encoded.extend_from_slice(s.as_bytes());
    
    let padding = (4 - (len % 4)) % 4;
    for _ in 0..padding {
        encoded.push(0);
    }
    
    encoded
}

pub fn decode_string(data: &[u8]) -> Option<(&str, usize)> {
    if data.len() < 4 {
        return None;
    }
    
    let len = u32::from_ne_bytes([data[0], data[1], data[2], data[3]]) as usize;
    let total_size = 4 + len + ((4 - (len % 4)) % 4);
    
    if data.len() < total_size {
        return None;
    }
    
    let s = core::str::from_utf8(&data[4..4+len]).ok()?;
    Some((s, total_size))
      }
