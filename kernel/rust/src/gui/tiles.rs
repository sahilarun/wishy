use crate::gui::surface::Surface;
use alloc::vec::Vec;

pub struct TilingManager {
    pub windows: Vec<Surface>,
    pub focused_index: usize,
    pub screen_width: usize,
    pub screen_height: usize,
    pub panel_height: usize,
}

impl TilingManager {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            windows: Vec::new(),
            focused_index: 0,
            screen_width: width,
            screen_height: height,
            panel_height: 30,
        }
    }
    
    pub fn add_window(&mut self, mut surface: Surface) {
        self.retile();
        self.windows.push(surface);
        self.retile();
        self.focused_index = self.windows.len() - 1;
    }
    
    pub fn remove_window(&mut self, index: usize) {
        if index < self.windows.len() {
            self.windows.remove(index);
            if self.focused_index >= self.windows.len() && !self.windows.is_empty() {
                self.focused_index = self.windows.len() - 1;
            }
            self.retile();
        }
    }
    
    pub fn focus_window(&mut self, index: usize) {
        if index < self.windows.len() {
            if self.focused_index < self.windows.len() {
                self.windows[self.focused_index].focused = false;
            }
            self.focused_index = index;
            self.windows[index].focused = true;
        }
    }
    
    pub fn focus_next(&mut self) {
        if !self.windows.is_empty() {
            let next = (self.focused_index + 1) % self.windows.len();
            self.focus_window(next);
        }
    }
    
    pub fn retile(&mut self) {
        let count = self.windows.len();
        if count == 0 {
            return;
        }
        
        let usable_height = self.screen_height - self.panel_height;
        let gap = 8;
        
        if count == 1 {
            let w = &mut self.windows[0];
            w.x = gap as i32;
            w.y = (self.panel_height + gap) as i32;
            w.width = self.screen_width - gap * 2;
            w.height = usable_height - gap * 2;
        } else {
            let master_width = self.screen_width / 2 - gap * 2;
            let stack_width = self.screen_width / 2 - gap * 2;
            let stack_height = (usable_height - gap * (count)) / (count - 1);
            
            self.windows[0].x = gap as i32;
            self.windows[0].y = (self.panel_height + gap) as i32;
            self.windows[0].width = master_width;
            self.windows[0].height = usable_height - gap * 2;
            
            for i in 1..count {
                let w = &mut self.windows[i];
                w.x = (self.screen_width / 2 + gap) as i32;
                w.y = (self.panel_height + gap + (i - 1) * (stack_height + gap)) as i32;
                w.width = stack_width;
                w.height = stack_height;
            }
        }
    }
    
    pub fn find_window_at(&self, x: i32, y: i32) -> Option<usize> {
        for (i, window) in self.windows.iter().enumerate().rev() {
            if window.contains_point(x, y) {
                return Some(i);
            }
        }
        None
    }
}
