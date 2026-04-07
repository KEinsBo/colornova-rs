use openrgb2::Color as OpenRgbColor;

use std::collections::HashMap;
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, Copy, Clone)]
pub struct HsvColor {
    pub h: f32,
    pub s: f32,
    pub v: f32,
}

pub fn avg_color(buffer: &[u8], stride: usize, _height: usize) -> Color {
    let mut sum_r = 0u64;
    let mut sum_g = 0u64;
    let mut sum_b = 0u64;
    let mut pixel_count = 0u64;

    for row in buffer.chunks(stride) {
        for pixel in row.chunks(4) {
            sum_b += pixel[0] as u64;
            sum_g += pixel[1] as u64;
            sum_r += pixel[2] as u64;
            pixel_count += 1;
        }
    }

    Color {
        r: (sum_r / pixel_count) as u8,
        g: (sum_g / pixel_count) as u8,
        b: (sum_b / pixel_count) as u8,
    }
}

impl Color {
    pub fn to_hsv(self) -> HsvColor {
        let r = self.r as f32 / 255.0;
        let g = self.g as f32 / 255.0;
        let b = self.b as f32 / 255.0;

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let delta = max - min;

        let h = if delta == 0.0 {
            0.0
        } else if max == r {
            60.0 * ((g - b) / delta).rem_euclid(6.0)
        } else if max == g {
            60.0 * ((b - r) / delta + 2.0)
        } else {
            60.0 * ((r - g) / delta + 4.0)
        };

        let s = if max == 0.0 { 0.0 } else { delta / max };
        let v = max;

        println!("RGB -> HSV: r: {}, g: {}, b: {}", self.r, self.g, self.b);
        HsvColor { h, s, v }
    }
}

impl HsvColor {
    pub fn to_rgb(self) -> Color {
        let c = self.v * self.s;
        let x = c * (1.0 - ((self.h / 60.0) % 2.0 - 1.0).abs());
        let m = self.v - c;

        let (r, g, b) = if self.h >= 0.0 && self.h < 60.0 {
            (c, x, 0.0)
        } else if self.h >= 60.0 && self.h < 120.0 {
            (x, c, 0.0)
        } else if self.h >= 120.0 && self.h < 180.0 {
            (0.0, c, x)
        } else if self.h >= 180.0 && self.h < 240.0 {
            (0.0, x, c)
        } else if self.h >= 240.0 && self.h < 300.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };

        let r = ((r + m) * 255.0).round() as u8;
        let g = ((g + m) * 255.0).round() as u8;
        let b = ((b + m) * 255.0).round() as u8;

        println!("HSV -> RGB: h: {}, s: {}, v: {}", self.h, self.s, self.v);
        Color { r, g, b }
    }
}

pub fn avg_hsv_color(buffer: &[u8], stride: usize, _height: usize) -> Color {
    let mut sum_h = 0.0;
    let mut sum_s = 0.0;
    let mut sum_v = 0.0;
    let mut pixel_count = 0;

    for row in buffer.chunks(stride) {
        for pixel in row.chunks(4) {
            let hsv_c = Color {
                r: pixel[2],
                g: pixel[1],
                b: pixel[0],
            }
            .to_hsv();

            sum_h += hsv_c.h;
            sum_s += hsv_c.s;
            sum_v += hsv_c.v;
            pixel_count += 1;
        }
    }

    let avg_h = sum_h / pixel_count as f32;
    let avg_s = sum_s / pixel_count as f32;
    let avg_v = sum_v / pixel_count as f32;
    let avg_h = avg_h.rem_euclid(360.0);
    HsvColor {
        h: avg_h,
        s: avg_s,
        v: avg_v,
    }
    .to_rgb()
}

pub fn most_frequent_color(buffer: &[u8], stride: usize, _height: usize) -> Color {
    let mut color_counts: HashMap<Color, u64> = HashMap::new();

    for row in buffer.chunks(stride) {
        for pixel in row.chunks(4) {
            let color = Color {
                r: pixel[2],
                g: pixel[1],
                b: pixel[0],
            };

            *color_counts.entry(color).or_insert(0) += 1;
        }
    }

    color_counts
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(color, _)| color)
        .unwrap_or(Color { r: 0, g: 0, b: 0 })
}

pub fn smooth_color(prev: &OpenRgbColor, current: &OpenRgbColor, factor: f32) -> OpenRgbColor {
    fn gamma(v: u8) -> f32 {
        (v as f32 / 255.0).powf(2.2)
    }
    fn inv_gamma(v: f32) -> u8 {
        (v.powf(1.0 / 2.2) * 255.0).round() as u8
    }

    OpenRgbColor {
        r: inv_gamma(gamma(prev.r) * (1.0 - factor) + gamma(current.r) * factor),
        g: inv_gamma(gamma(prev.g) * (1.0 - factor) + gamma(current.g) * factor),
        b: inv_gamma(gamma(prev.b) * (1.0 - factor) + gamma(current.b) * factor),
    }
}
pub fn color_changed(a: &OpenRgbColor, b: &OpenRgbColor, threshold: u8) -> bool {
    let dr = (a.r as i16 - b.r as i16).abs();
    let dg = (a.g as i16 - b.g as i16).abs();
    let db = (a.b as i16 - b.b as i16).abs();

    dr > threshold as i16 || dg > threshold as i16 || db > threshold as i16
}
