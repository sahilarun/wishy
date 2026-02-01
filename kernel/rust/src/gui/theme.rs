#[derive(Clone, Copy)]
pub struct Theme {
    pub bg_color: u32,
    pub active_border: u32,
    pub inactive_border: u32,
    pub titlebar_bg: u32,
    pub titlebar_text: u32,
    pub panel_bg: u32,
    pub panel_text: u32,
    pub shadow_color: u32,
    pub shadow_intensity: u8,
    pub border_width: usize,
    pub corner_radius: usize,
}

impl Theme {
    pub const fn default() -> Self {
        Self {
            bg_color: 0x1a1a2e,
            active_border: 0x00d9ff,
            inactive_border: 0x2d3142,
            titlebar_bg: 0x16213e,
            titlebar_text: 0xeaeaea,
            panel_bg: 0x0f3460,
            panel_text: 0xe94560,
            shadow_color: 0x000000,
            shadow_intensity: 40,
            border_width: 2,
            corner_radius: 8,
        }
    }
}

pub static THEME: Theme = Theme::default();
