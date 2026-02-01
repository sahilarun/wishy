use spin::Mutex;

const VGA_BUFFER: usize = 0xB8000;
const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;

static CONSOLE: Mutex<ConsoleWriter> = Mutex::new(ConsoleWriter {
    col: 0,
    row: 0,
    color: 0x0F,
});

struct ConsoleWriter {
    col: usize,
    row: usize,
    color: u8,
}

impl ConsoleWriter {
    fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.newline(),
            byte => {
                if self.col >= VGA_WIDTH {
                    self.newline();
                }
                
                let offset = self.row * VGA_WIDTH + self.col;
                unsafe {
                    let vga = VGA_BUFFER as *mut u16;
                    *vga.add(offset) = ((self.color as u16) << 8) | byte as u16;
                }
                self.col += 1;
            }
        }
    }
    
    fn write_str(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
    }
    
    fn newline(&mut self) {
        self.col = 0;
        self.row += 1;
        
        if self.row >= VGA_HEIGHT {
            self.scroll();
            self.row = VGA_HEIGHT - 1;
        }
    }
    
    fn scroll(&mut self) {
        unsafe {
            let vga = VGA_BUFFER as *mut u16;
            for row in 1..VGA_HEIGHT {
                for col in 0..VGA_WIDTH {
                    let src = vga.add(row * VGA_WIDTH + col);
                    let dst = vga.add((row - 1) * VGA_WIDTH + col);
                    *dst = *src;
                }
            }
            
            for col in 0..VGA_WIDTH {
                let offset = (VGA_HEIGHT - 1) * VGA_WIDTH + col;
                *vga.add(offset) = ((self.color as u16) << 8) | b' ' as u16;
            }
        }
    }
    
    fn clear(&mut self) {
        unsafe {
            let vga = VGA_BUFFER as *mut u16;
            for i in 0..(VGA_WIDTH * VGA_HEIGHT) {
                *vga.add(i) = ((self.color as u16) << 8) | b' ' as u16;
            }
        }
        self.col = 0;
        self.row = 0;
    }
    
    fn set_color(&mut self, color: u8) {
        self.color = color;
    }
}

pub struct Console;

impl Console {
    pub fn init() {
        let mut console = CONSOLE.lock();
        console.clear();
    }
    
    pub fn clear() {
        CONSOLE.lock().clear();
    }
    
    pub fn print(s: &str) {
        CONSOLE.lock().write_str(s);
    }
    
    pub fn println(s: &str) {
        let mut console = CONSOLE.lock();
        console.write_str(s);
        console.newline();
    }
    
    pub fn set_color(fg: u8, bg: u8) {
        CONSOLE.lock().set_color((bg << 4) | fg);
    }
}
