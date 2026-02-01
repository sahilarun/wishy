use spin::Mutex;

const PS2_DATA: u16 = 0x60;
const PS2_STATUS: u16 = 0x64;

pub struct InputState {
    pub mouse_x: i32,
    pub mouse_y: i32,
    pub mouse_buttons: u8,
    pub key_state: [bool; 256],
    pub shift_pressed: bool,
    pub alt_pressed: bool,
    pub ctrl_pressed: bool,
}

static INPUT: Mutex<InputState> = Mutex::new(InputState {
    mouse_x: 0,
    mouse_y: 0,
    mouse_buttons: 0,
    key_state: [false; 256],
    shift_pressed: false,
    alt_pressed: false,
    ctrl_pressed: false,
});

pub fn init() {
    init_keyboard();
    init_mouse();
}

fn init_keyboard() {
    outb(PS2_STATUS, 0xAE);
}

fn init_mouse() {
    wait_write();
    outb(PS2_STATUS, 0xA8);
    
    wait_write();
    outb(PS2_STATUS, 0x20);
    wait_read();
    let status = inb(PS2_DATA) | 2;
    
    wait_write();
    outb(PS2_STATUS, 0x60);
    wait_write();
    outb(PS2_DATA, status);
    
    write_mouse(0xF6);
    write_mouse(0xF4);
}

pub fn poll() {
    while (inb(PS2_STATUS) & 1) != 0 {
        let data = inb(PS2_DATA);
        
        if (inb(PS2_STATUS) & 0x20) != 0 {
            handle_mouse_data(data);
        } else {
            handle_keyboard_data(data);
        }
    }
}

fn handle_keyboard_data(scancode: u8) {
    let mut input = INPUT.lock();
    
    let released = (scancode & 0x80) != 0;
    let key = scancode & 0x7F;
    
    input.key_state[key as usize] = !released;
    
    match key {
        0x2A | 0x36 => input.shift_pressed = !released,
        0x38 => input.alt_pressed = !released,
        0x1D => input.ctrl_pressed = !released,
        _ => {}
    }
}

static mut MOUSE_CYCLE: u8 = 0;
static mut MOUSE_BYTES: [u8; 3] = [0; 3];

fn handle_mouse_data(data: u8) {
    unsafe {
        MOUSE_BYTES[MOUSE_CYCLE as usize] = data;
        MOUSE_CYCLE += 1;
        
        if MOUSE_CYCLE == 3 {
            MOUSE_CYCLE = 0;
            
            let mut input = INPUT.lock();
            input.mouse_buttons = MOUSE_BYTES[0] & 0x07;
            
            let mut dx = MOUSE_BYTES[1] as i32;
            let mut dy = MOUSE_BYTES[2] as i32;
            
            if (MOUSE_BYTES[0] & 0x10) != 0 {
                dx -= 256;
            }
            if (MOUSE_BYTES[0] & 0x20) != 0 {
                dy -= 256;
            }
            
            input.mouse_x = (input.mouse_x + dx).max(0).min(1023);
            input.mouse_y = (input.mouse_y - dy).max(0).min(767);
        }
    }
}

pub fn get_mouse_pos() -> (i32, i32) {
    let input = INPUT.lock();
    (input.mouse_x, input.mouse_y)
}

pub fn get_mouse_buttons() -> u8 {
    INPUT.lock().mouse_buttons
}

pub fn is_key_pressed(scancode: u8) -> bool {
    INPUT.lock().key_state[scancode as usize]
}

pub fn is_alt_pressed() -> bool {
    INPUT.lock().alt_pressed
}

pub fn is_shift_pressed() -> bool {
    INPUT.lock().shift_pressed
}

fn wait_read() {
    for _ in 0..1000 {
        if (inb(PS2_STATUS) & 1) != 0 {
            return;
        }
    }
}

fn wait_write() {
    for _ in 0..1000 {
        if (inb(PS2_STATUS) & 2) == 0 {
            return;
        }
    }
}

fn write_mouse(cmd: u8) {
    wait_write();
    outb(PS2_STATUS, 0xD4);
    wait_write();
    outb(PS2_DATA, cmd);
}

fn inb(port: u16) -> u8 {
    unsafe {
        let result: u8;
        core::arch::asm!("in al, dx", out("al") result, in("dx") port);
        result
    }
}

fn outb(port: u16, value: u8) {
    unsafe {
        core::arch::asm!("out dx, al", in("dx") port, in("al") value);
    }
}
