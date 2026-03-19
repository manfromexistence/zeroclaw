//! Visual effects for the onboard CLI

use std::time::Instant;

#[derive(Debug, Clone, Copy)]
pub struct RgbColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, Clone)]
pub struct RainbowEffect {
    start_time: Instant,
    speed: f32,
}

impl RainbowEffect {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            speed: 0.5, // 0.5 cycles per second (slower)
        }
    }

    pub fn elapsed(&self) -> f32 {
        self.start_time.elapsed().as_secs_f32()
    }

    pub fn current_color(&self) -> RgbColor {
        let elapsed = self.start_time.elapsed().as_secs_f32();
        let hue = (elapsed * self.speed * 360.0) % 360.0;
        Self::hsl_to_rgb(hue, 0.8, 0.6)
    }

    pub fn color_at(&self, index: usize) -> RgbColor {
        let elapsed = self.start_time.elapsed().as_secs_f32();
        let hue = ((elapsed * self.speed * 360.0) + (index as f32 * 10.0)) % 360.0;
        Self::hsl_to_rgb(hue, 0.8, 0.6)
    }

    fn hsl_to_rgb(h: f32, s: f32, l: f32) -> RgbColor {
        let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = l - c / 2.0;

        let (r, g, b) = if h < 60.0 {
            (c, x, 0.0)
        } else if h < 120.0 {
            (x, c, 0.0)
        } else if h < 180.0 {
            (0.0, c, x)
        } else if h < 240.0 {
            (0.0, x, c)
        } else if h < 300.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };

        RgbColor {
            r: ((r + m) * 255.0) as u8,
            g: ((g + m) * 255.0) as u8,
            b: ((b + m) * 255.0) as u8,
        }
    }
}

impl Default for RainbowEffect {
    fn default() -> Self {
        Self::new()
    }
}