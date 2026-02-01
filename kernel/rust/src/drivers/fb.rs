use spin::Mutex;

const FB_ADDRESS: usize = 0xA0000;
const FB_WIDTH: usize = 1024;
const FB_HEIGHT: usize = 768;

pub struct Framebuffer {
    buffer: &'static mut [u32],
    width: usize,
    height: usize,
    back_buffer: [u32; FB_WIDTH * FB_HEIGHT],
}

static FRAMEBUFFER: Mutex<Option<Framebuffer>> = Mutex::new(None);

pub fn init() {
    set_vbe_mode(FB_WIDTH as u16, FB_HEIGHT as u16, 32);
    
    let buffer = unsafe {
        core::slice::from_raw_parts_mut(
            0xE0000000 as *mut u32,
            FB_WIDTH * FB_HEIGHT
        )
    };
    
    let fb = Framebuffer {
        buffer,
        width: FB_WIDTH,
        height: FB_HEIGHT,
        back_buffer: [0; FB_WIDTH * FB_HEIGHT],
    };
    
    *FRAMEBUFFER.lock() = Some(fb);
}

pub fn get() -> &'static Mutex<Option<Framebuffer>> {
    &FRAMEBUFFER
}

fn get_font_bitmap(ch: char) -> &'static [u8; 16] {
    // Basic 8x16 font for ASCII printable characters
    static FONT: [[u8; 16]; 96] = [[0; 16]; 96];
    let idx = (ch as usize).saturating_sub(32).min(95);
    &FONT[idx]
}

impl Framebuffer {
    pub fn width(&self) -> usize {
        self.width
    }
    
    pub fn height(&self) -> usize {
        self.height
    }
    
    pub fn put_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            self.back_buffer[y * self.width + x] = color;
        }
    }
    
    pub fn fill_rect(&mut self, x: usize, y: usize, w: usize, h: usize, color: u32) {
        for dy in 0..h {
            for dx in 0..w {
                self.put_pixel(x + dx, y + dy, color);
            }
        }
    }
    
    pub fn blit(&mut self, x: usize, y: usize, w: usize, h: usize, data: &[u32]) {
        for dy in 0..h {
            for dx in 0..w {
                if x + dx < self.width && y + dy < self.height {
                    let idx = dy * w + dx;
                    if idx < data.len() {
                        self.back_buffer[(y + dy) * self.width + (x + dx)] = data[idx];
                    }
                }
            }
        }
    }
    
    pub fn swap(&mut self) {
        self.buffer.copy_from_slice(&self.back_buffer);
    }
    
    pub fn put_char(&mut self, x: usize, y: usize, ch: char, fg: u32, bg: u32) {
        let font = get_font_bitmap(ch);
        for dy in 0..16 {
            for dx in 0..8 {
                let pixel = if font[dy] & (1 << (7 - dx)) != 0 { fg } else { bg };
                self.put_pixel(x + dx, y + dy, pixel);
            }
        }
    }
    
    pub fn print_string(&mut self, x: usize, y: usize, s: &str, fg: u32, bg: u32) {
        let mut cx = x;
        for ch in s.chars() {
            if ch == '\n' {
                break;
            }
            self.put_char(cx, y, ch, fg, bg);
            cx += 8;
            if cx >= self.width {
                break;
            }
        }
    }
    
    pub fn clear(&mut self, color: u32) {
        self.back_buffer.fill(color);
    }
}

fn set_vbe_mode(width: u16, height: u16, bpp: u16) {
    unsafe {
        let mode_info = 0x5000 as *mut u16;
        core::ptr::write_volatile(mode_info, 0x4118);
        core::ptr::write_volatile(mode_info.offset(1), width);
        core::ptr::write_volatile(mode_info.offset(2), height);
        core::ptr::write_volatile(mode_info.offset(3), bpp);
    }
                }
