pub struct Animation {
    pub start_time: u64,
    pub duration: u64,
    pub animation_type: AnimationType,
}

#[derive(Clone, Copy)]
pub enum AnimationType {
    FadeIn,
    FadeOut,
    SlideIn,
    SlideOut,
}

impl Animation {
    pub fn new(anim_type: AnimationType, duration: u64) -> Self {
        Self {
            start_time: get_ticks(),
            duration,
            animation_type: anim_type,
        }
    }
    
    pub fn progress(&self) -> f32 {
        let elapsed = get_ticks() - self.start_time;
        if elapsed >= self.duration {
            return 1.0;
        }
        (elapsed as f32) / (self.duration as f32)
    }
    
    pub fn is_complete(&self) -> bool {
        self.progress() >= 1.0
    }
    
    pub fn get_alpha(&self) -> u8 {
        let p = self.progress();
        match self.animation_type {
            AnimationType::FadeIn => (p * 255.0) as u8,
            AnimationType::FadeOut => ((1.0 - p) * 255.0) as u8,
            _ => 255,
        }
    }
    
    pub fn get_offset(&self) -> i32 {
        let p = self.progress();
        match self.animation_type {
            AnimationType::SlideIn => ((1.0 - p) * 100.0) as i32,
            AnimationType::SlideOut => (p * 100.0) as i32,
            _ => 0,
        }
    }
}

fn get_ticks() -> u64 {
    static mut TICKS: u64 = 0;
    unsafe {
        TICKS += 1;
        TICKS
    }
}

pub fn ease_out_cubic(t: f32) -> f32 {
    let t = t - 1.0;
    t * t * t + 1.0
}

pub fn ease_in_out_cubic(t: f32) -> f32 {
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        let t = t - 1.0;
        1.0 + 4.0 * t * t * t
    }
}
