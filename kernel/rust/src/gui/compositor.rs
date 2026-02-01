use crate::drivers::fb;
use crate::gui::{surface::Surface, tiles::TilingManager, input, renderer, animation, icons, theme::THEME};
use spin::Mutex;

static COMPOSITOR: Mutex<Option<Compositor>> = Mutex::new(None);

pub struct Compositor {
    tiling: TilingManager,
    animations: alloc::vec::Vec<(usize, animation::Animation)>,
    dragging: Option<(usize, i32, i32)>,
    last_mouse_buttons: u8,
}

pub fn init() {
    input::init();
    
    let compositor = Compositor {
        tiling: TilingManager::new(1024, 768),
        animations: alloc::vec::Vec::new(),
        dragging: None,
        last_mouse_buttons: 0,
    };
    
    *COMPOSITOR.lock() = Some(compositor);
    
    spawn_demo_windows();
}

pub fn run() {
    loop {
        input::poll();
        update();
        render();
        
        for _ in 0..100000 {
            core::hint::spin_loop();
        }
    }
}

fn update() {
    let mut comp = COMPOSITOR.lock();
    let comp = comp.as_mut().unwrap();
    
    let (mx, my) = input::get_mouse_pos();
    let buttons = input::get_mouse_buttons();
    
    if (buttons & 1) != 0 && (comp.last_mouse_buttons & 1) == 0 {
        if let Some(idx) = comp.tiling.find_window_at(mx, my) {
            comp.tiling.focus_window(idx);
            comp.dragging = Some((idx, mx - comp.tiling.windows[idx].x, my - comp.tiling.windows[idx].y));
        }
    }
    
    if (buttons & 1) == 0 {
        comp.dragging = None;
    }
    
    if let Some((idx, ox, oy)) = comp.dragging {
        if idx < comp.tiling.windows.len() {
            comp.tiling.windows[idx].x = mx - ox;
            comp.tiling.windows[idx].y = my - oy;
        }
    }
    
    comp.last_mouse_buttons = buttons;
    
    if input::is_alt_pressed() && input::is_key_pressed(0x10) {
        if !comp.tiling.windows.is_empty() {
            comp.tiling.remove_window(comp.tiling.focused_index);
        }
    }
    
    if input::is_alt_pressed() && input::is_key_pressed(0x0F) {
        comp.tiling.focus_next();
    }
    
    if input::is_alt_pressed() && input::is_key_pressed(0x1C) {
        spawn_window();
    }
    
    comp.animations.retain(|(_, anim)| !anim.is_complete());
}

fn render() {
    let fb_lock = fb::get();
    let mut fb_opt = fb_lock.lock();
    let fb = fb_opt.as_mut().unwrap();
    
    fb.clear(THEME.bg_color);
    
    renderer::draw_panel(fb, 1024, 30);
    
    let comp = COMPOSITOR.lock();
    let comp = comp.as_ref().unwrap();
    
    for (i, window) in comp.tiling.windows.iter().enumerate() {
        let alpha = comp.animations.iter()
            .find(|(idx, _)| *idx == i)
            .map(|(_, anim)| anim.get_alpha())
            .unwrap_or(255);
        
        renderer::render_window(fb, window, alpha);
    }
    
    let (mx, my) = input::get_mouse_pos();
    icons::draw_cursor(fb, mx as usize, my as usize);
    
    fb.swap();
}

fn spawn_demo_windows() {
    for i in 0..2 {
        let mut surface = Surface::new(100 + i * 50, 100 + i * 50, 400, 300);
        surface.set_title(match i {
            0 => "Terminal",
            1 => "File Manager",
            _ => "Window",
        });
        surface.clear(0xFF1a1a2e);
        
        let mut comp = COMPOSITOR.lock();
        let comp = comp.as_mut().unwrap();
        comp.tiling.add_window(surface);
        comp.animations.push((i as usize, animation::Animation::new(animation::AnimationType::FadeIn, 20)));
    }
}

fn spawn_window() {
    let mut surface = Surface::new(200, 150, 500, 400);
    surface.set_title("New Window");
    surface.clear(0xFF202030);
    
    let mut comp = COMPOSITOR.lock();
    let comp = comp.as_mut().unwrap();
    let idx = comp.tiling.windows.len();
    comp.tiling.add_window(surface);
    comp.animations.push((idx, animation::Animation::new(animation::AnimationType::FadeIn, 20)));
}
