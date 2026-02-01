use crate::drivers::fb::Framebuffer;
use crate::gui::surface::Surface;
use crate::gui::theme::THEME;
use crate::gui::icons;

const TITLEBAR_HEIGHT: usize = 28;

pub fn render_window(fb: &mut Framebuffer, window: &Surface, alpha: u8) {
    let border_color = if window.focused {
        THEME.active_border
    } else {
        THEME.inactive_border
    };
    
    draw_shadow(fb, window);
    draw_border_rounded(fb, window, border_color);
    draw_titlebar(fb, window);
    draw_window_content(fb, window, alpha);
}

fn draw_shadow(fb: &mut Framebuffer, window: &Surface) {
    let offset = 4;
    let shadow_color = blend_color(THEME.shadow_color, THEME.shadow_intensity);
    
    for dy in 0..window.height + TITLEBAR_HEIGHT + offset {
        for dx in 0..window.width + offset {
            let x = (window.x + dx as i32 + offset as i32) as usize;
            let y = (window.y + dy as i32 - TITLEBAR_HEIGHT as i32) as usize;
            
            if x < fb.width() && y < fb.height() {
                fb.put_pixel(x, y, shadow_color);
            }
        }
    }
}

fn draw_border_rounded(fb: &mut Framebuffer, window: &Surface, color: u32) {
    let r = THEME.corner_radius;
    let w = THEME.border_width;
    
    for t in 0..w {
        for x in r..(window.width - r) {
            let px = (window.x + x as i32) as usize;
            let py_top = (window.y - TITLEBAR_HEIGHT as i32 + t as i32) as usize;
            let py_bot = (window.y + window.height as i32 - 1 - t as i32) as usize;
            
            if px < fb.width() {
                if py_top < fb.height() {
                    fb.put_pixel(px, py_top, color);
                }
                if py_bot < fb.height() {
                    fb.put_pixel(px, py_bot, color);
                }
            }
        }
        
        for y in r..(window.height + TITLEBAR_HEIGHT - r) {
            let py = (window.y - TITLEBAR_HEIGHT as i32 + y as i32) as usize;
            let px_left = (window.x + t as i32) as usize;
            let px_right = (window.x + window.width as i32 - 1 - t as i32) as usize;
            
            if py < fb.height() {
                if px_left < fb.width() {
                    fb.put_pixel(px_left, py, color);
                }
                if px_right < fb.width() {
                    fb.put_pixel(px_right, py, color);
                }
            }
        }
    }
}

fn draw_titlebar(fb: &mut Framebuffer, window: &Surface) {
    let y_start = window.y - TITLEBAR_HEIGHT as i32;
    
    for dy in 0..TITLEBAR_HEIGHT {
        for dx in 0..window.width {
            let x = (window.x + dx as i32) as usize;
            let y = (y_start + dy as i32) as usize;
            
            if x < fb.width() && y < fb.height() {
                fb.put_pixel(x, y, THEME.titlebar_bg);
            }
        }
    }
    
    draw_text(fb, window.x + 8, y_start + 8, &window.title[..window.title_len], THEME.titlebar_text);
    
    let close_x = window.x + window.width as i32 - 24;
    let close_y = y_start + 6;
    icons::draw_close_button(fb, close_x as usize, close_y as usize);
}

fn draw_window_content(fb: &mut Framebuffer, window: &Surface, alpha: u8) {
    for dy in 0..window.height {
        for dx in 0..window.width {
            let px = (window.x + dx as i32) as usize;
            let py = (window.y + dy as i32) as usize;
            
            if px < fb.width() && py < fb.height() {
                let color = window.pixels[dy * window.width + dx];
                let final_color = if alpha < 255 {
                    blend_alpha(color, alpha)
                } else {
                    color
                };
                fb.put_pixel(px, py, final_color);
            }
        }
    }
}

fn draw_text(fb: &mut Framebuffer, x: i32, y: i32, text: &[u8], color: u32) {
    for (i, &ch) in text.iter().enumerate() {
        let px = x + (i * 8) as i32;
        if px >= 0 && px < fb.width() as i32 {
            draw_char(fb, px as usize, y as usize, ch, color);
        }
    }
}

fn draw_char(fb: &mut Framebuffer, x: usize, y: usize, ch: u8, color: u32) {
    let glyph = get_glyph(ch);
    
    for dy in 0..8 {
        for dx in 0..8 {
            if (glyph[dy] & (1 << (7 - dx))) != 0 {
                if x + dx < fb.width() && y + dy < fb.height() {
                    fb.put_pixel(x + dx, y + dy, color);
                }
            }
        }
    }
}

fn get_glyph(ch: u8) -> [u8; 8] {
    match ch {
        b'A'..=b'Z' => [0x18, 0x24, 0x42, 0x7E, 0x42, 0x42, 0x42, 0x00],
        b'a'..=b'z' => [0x00, 0x00, 0x3C, 0x02, 0x3E, 0x42, 0x3E, 0x00],
        b'0'..=b'9' => [0x3C, 0x42, 0x46, 0x4A, 0x52, 0x62, 0x3C, 0x00],
        _ => [0x00; 8],
    }
}

fn blend_color(color: u32, intensity: u8) -> u32 {
    let r = ((color >> 16) & 0xFF) * intensity as u32 / 255;
    let g = ((color >> 8) & 0xFF) * intensity as u32 / 255;
    let b = (color & 0xFF) * intensity as u32 / 255;
    (r << 16) | (g << 8) | b
}

fn blend_alpha(color: u32, alpha: u8) -> u32 {
    let r = ((color >> 16) & 0xFF) * alpha as u32 / 255;
    let g = ((color >> 8) & 0xFF) * alpha as u32 / 255;
    let b = (color & 0xFF) * alpha as u32 / 255;
    (r << 16) | (g << 8) | b
}

pub fn draw_panel(fb: &mut Framebuffer, width: usize, height: usize) {
    for y in 0..height {
        for x in 0..width {
            fb.put_pixel(x, y, THEME.panel_bg);
        }
    }
    
    draw_text(fb, 8, 8, b"wishy v0.1", THEME.panel_text);
    
    let time_str = b"12:34";
    draw_text(fb, (width - 50) as i32, 8, time_str, THEME.panel_text);
    
    icons::draw_launcher_icon(fb, width - 80, 5);
                          }
