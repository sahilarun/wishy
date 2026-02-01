use alloc::vec;
use alloc::vec::Vec;

pub struct Surface {
    pub x: i32,
    pub y: i32,
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u32>,
    pub focused: bool,
    pub title: [u8; 64],
    pub title_len: usize,
}

impl Surface {
    pub fn new(x: i32, y: i32, width: usize, height: usize) -> Self {
        Self {
            x,
            y,
            width,
            height,
            pixels: vec![0xFF202020; width * height],
            focused: false,
            title: [0; 64],
            title_len: 0,
        }
    }
    
    pub fn set_title(&mut self, title: &str) {
        let bytes = title.as_bytes();
        let len = bytes.len().min(63);
        self.title[..len].copy_from_slice(&bytes[..len]);
        self.title_len = len;
    }
    
    pub fn clear(&mut self, color: u32) {
        self.pixels.fill(color);
    }
    
    pub fn put_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            self.pixels[y * self.width + x] = color;
        }
    }
    
    pub fn fill_rect(&mut self, x: usize, y: usize, w: usize, h: usize, color: u32) {
        for dy in 0..h {
            for dx in 0..w {
                self.put_pixel(x + dx, y + dy, color);
            }
        }
    }
    
    pub fn contains_point(&self, px: i32, py: i32) -> bool {
        px >= self.x && px < self.x + self.width as i32 &&
        py >= self.y && py < self.y + self.height as i32
    }
}
