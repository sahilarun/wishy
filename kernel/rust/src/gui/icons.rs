use crate::drivers::fb::Framebuffer;

pub fn draw_close_button(fb: &mut Framebuffer, x: usize, y: usize) {
    let size = 16;
    let color = 0xFF5555;
    
    for i in 0..size {
        fb.put_pixel(x + i, y + i, color);
        fb.put_pixel(x + size - i - 1, y + i, color);
    }
}

pub fn draw_launcher_icon(fb: &mut Framebuffer, x: usize, y: usize) {
    let size = 20;
    let color = 0x00D9FF;
    
    for dy in 0..size {
        for dx in 0..size {
            if (dx < 2 || dx >= size - 2 || dy < 2 || dy >= size - 2) {
                fb.put_pixel(x + dx, y + dy, color);
            }
        }
    }
    
    for i in 0..3 {
        for j in 0..3 {
            let px = x + 5 + i * 5;
            let py = y + 5 + j * 5;
            for dy in 0..3 {
                for dx in 0..3 {
                    fb.put_pixel(px + dx, py + dy, color);
                }
            }
        }
    }
}

pub fn draw_terminal_icon(fb: &mut Framebuffer, x: usize, y: usize) {
    let color = 0x00FF00;
    let size = 24;
    
    for dy in 0..size {
        for dx in 0..size {
            if dx == 0 || dx == size - 1 || dy == 0 || dy == size - 1 {
                fb.put_pixel(x + dx, y + dy, color);
            }
        }
    }
    
    for i in 0..8 {
        fb.put_pixel(x + 4 + i, y + 8, color);
        fb.put_pixel(x + 4 + i / 2, y + 8 + i, color);
    }
    
    for i in 0..6 {
        fb.put_pixel(x + 4 + i, y + 16, color);
    }
}

pub fn draw_file_icon(fb: &mut Framebuffer, x: usize, y: usize) {
    let color = 0xFFAA00;
    let size = 24;
    
    for dy in 2..size {
        fb.put_pixel(x + 4, y + dy, color);
        fb.put_pixel(x + size - 4, y + dy, color);
    }
    
    for dx in 4..size - 4 {
        fb.put_pixel(x + dx, y + 2, color);
        fb.put_pixel(x + dx, y + size - 2, color);
    }
    
    for dx in 4..size - 8 {
        fb.put_pixel(x + dx, y + 0, color);
    }
    
    for dy in 0..4 {
        fb.put_pixel(x + size - 8, y + dy, color);
    }
}

pub fn draw_cursor(fb: &mut Framebuffer, x: usize, y: usize) {
    let color = 0xFFFFFF;
    let size = 16;
    
    for i in 0..size {
        for j in 0..=(i / 2) {
            if x + j < fb.width() && y + i < fb.height() {
                fb.put_pixel(x + j, y + i, color);
            }
        }
    }
}

pub struct Icon {
    pub width: usize,
    pub height: usize,
    pub pixels: &'static [u32],
}

pub fn load_icon(name: &str) -> Option<Icon> {
    match name {
        "close" => Some(create_close_icon()),
        "terminal" => Some(create_terminal_icon()),
        "file" => Some(create_file_icon()),
        _ => None,
    }
}

fn create_close_icon() -> Icon {
    static PIXELS: [u32; 256] = [0xFFFF5555; 256];
    Icon {
        width: 16,
        height: 16,
        pixels: &PIXELS,
    }
}

fn create_terminal_icon() -> Icon {
    static PIXELS: [u32; 576] = [0xFF00FF00; 576];
    Icon {
        width: 24,
        height: 24,
        pixels: &PIXELS,
    }
}

fn create_file_icon() -> Icon {
    static PIXELS: [u32; 576] = [0xFFFFAA00; 576];
    Icon {
        width: 24,
        height: 24,
        pixels: &PIXELS,
    }
  }
