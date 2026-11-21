//! Phosphor's small software-rendering core.
//!
//! This crate deliberately starts with a backend-independent pixel surface.
//! A Redox display backend can present this surface once its driver interface
//! is wired in, without coupling the drawing code to a window manager.

#![no_std]

extern crate alloc;

use alloc::{vec, vec::Vec};

pub mod scene;
pub mod text;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl Color {
    pub const fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha: 255,
        }
    }

    pub const fn rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }
}

/// An owned 32-bit RGBA pixel surface used by Phosphor's software renderer.
#[derive(Clone, Debug)]
pub struct Surface {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl Surface {
    pub fn new(width: usize, height: usize, clear: Color) -> Option<Self> {
        let length = width.checked_mul(height)?;
        Some(Self {
            width,
            height,
            pixels: vec![clear; length],
        })
    }

    pub const fn width(&self) -> usize {
        self.width
    }
    pub const fn height(&self) -> usize {
        self.height
    }
    pub fn pixels(&self) -> &[Color] {
        &self.pixels
    }

    /// Blend one pixel into the surface. Pixels outside the surface are ignored.
    pub fn blend_pixel(&mut self, x: i32, y: i32, color: Color) {
        if x < 0 || y < 0 || x as usize >= self.width || y as usize >= self.height {
            return;
        }
        let index = y as usize * self.width + x as usize;
        self.pixels[index] = blend(self.pixels[index], color);
    }

    /// Draw an integer line with a small square brush.
    pub fn draw_line(&mut self, start: (i32, i32), end: (i32, i32), thickness: u32, color: Color) {
        let (mut x0, mut y0) = start;
        let (x1, y1) = end;
        let dx = (x1 - x0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let dy = -(y1 - y0).abs();
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut error = dx + dy;
        let radius = (thickness.max(1) as i32 - 1) / 2;

        loop {
            for oy in -radius..=radius {
                for ox in -radius..=radius {
                    self.blend_pixel(x0 + ox, y0 + oy, color);
                }
            }
            if x0 == x1 && y0 == y1 {
                break;
            }
            let twice_error = 2 * error;
            if twice_error >= dy {
                error += dy;
                x0 += sx;
            }
            if twice_error <= dx {
                error += dx;
                y0 += sy;
            }
        }
    }

    pub fn clear(&mut self, color: Color) {
        self.pixels.fill(color);
    }

    /// Paint a vertical color ramp across the complete surface.
    pub fn vertical_gradient(&mut self, top: Color, bottom: Color) {
        if self.height == 0 {
            return;
        }
        for y in 0..self.height {
            let t = if self.height == 1 {
                0.0
            } else {
                y as f32 / (self.height - 1) as f32
            };
            let color = Color::rgb(
                lerp(top.red, bottom.red, t),
                lerp(top.green, bottom.green, t),
                lerp(top.blue, bottom.blue, t),
            );
            let start = y * self.width;
            self.pixels[start..start + self.width].fill(color);
        }
    }

    /// Draw a clipped, alpha-blended rectangle.
    pub fn fill_rect(&mut self, x: i32, y: i32, width: u32, height: u32, color: Color) {
        let Some(right) = x.checked_add(width.min(i32::MAX as u32) as i32) else {
            return;
        };
        let Some(bottom) = y.checked_add(height.min(i32::MAX as u32) as i32) else {
            return;
        };
        let left = x.max(0) as usize;
        let top = y.max(0) as usize;
        let right = (right.max(0) as usize).min(self.width);
        let bottom = (bottom.max(0) as usize).min(self.height);
        if left >= right || top >= bottom {
            return;
        }

        for row in top..bottom {
            let offset = row * self.width;
            for column in left..right {
                let index = offset + column;
                self.pixels[index] = blend(self.pixels[index], color);
            }
        }
    }

    /// Draw a rounded rectangle using a simple per-pixel distance check.
    pub fn fill_round_rect(
        &mut self,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        radius: u32,
        color: Color,
    ) {
        let width = width.min(i32::MAX as u32) as i32;
        let height = height.min(i32::MAX as u32) as i32;
        if width <= 0 || height <= 0 {
            return;
        }
        let radius = (radius.min((width.min(height) / 2) as u32)) as i32;
        if radius == 0 {
            self.fill_rect(x, y, width as u32, height as u32, color);
            return;
        }
        let Some(right) = x.checked_add(width) else {
            return;
        };
        let Some(bottom) = y.checked_add(height) else {
            return;
        };
        let left_clip = x.max(0);
        let top_clip = y.max(0);
        let right_clip = right.min(self.width.min(i32::MAX as usize) as i32);
        let bottom_clip = bottom.min(self.height.min(i32::MAX as usize) as i32);
        let radius_sq = (radius * radius) as i64;

        for py in top_clip..bottom_clip {
            for px in left_clip..right_clip {
                let cx = if px < x + radius {
                    x + radius
                } else if px >= right - radius {
                    right - radius - 1
                } else {
                    px
                };
                let cy = if py < y + radius {
                    y + radius
                } else if py >= bottom - radius {
                    bottom - radius - 1
                } else {
                    py
                };
                let dx = (px - cx) as i64;
                let dy = (py - cy) as i64;
                if dx * dx + dy * dy <= radius_sq {
                    let index = py as usize * self.width + px as usize;
                    self.pixels[index] = blend(self.pixels[index], color);
                }
            }
        }
    }

    /// Draw a filled ellipse, clipped to the surface bounds.
    pub fn fill_ellipse(&mut self, x: i32, y: i32, width: u32, height: u32, color: Color) {
        if width == 0 || height == 0 {
            return;
        }
        let Some(right) = x.checked_add(width.min(i32::MAX as u32) as i32) else {
            return;
        };
        let Some(bottom) = y.checked_add(height.min(i32::MAX as u32) as i32) else {
            return;
        };
        let left = x.max(0);
        let top = y.max(0);
        let right = right.min(self.width.min(i32::MAX as usize) as i32);
        let bottom = bottom.min(self.height.min(i32::MAX as usize) as i32);
        let rx = width as f64 / 2.0;
        let ry = height as f64 / 2.0;
        let cx = x as f64 + rx;
        let cy = y as f64 + ry;

        for py in top..bottom {
            for px in left..right {
                let dx = (px as f64 + 0.5 - cx) / rx;
                let dy = (py as f64 + 0.5 - cy) / ry;
                if dx * dx + dy * dy <= 1.0 {
                    let index = py as usize * self.width + px as usize;
                    self.pixels[index] = blend(self.pixels[index], color);
                }
            }
        }
    }
}

fn lerp(a: u8, b: u8, t: f32) -> u8 {
    let value = a as f32 + (b as f32 - a as f32) * t;
    (value + 0.5).clamp(0.0, 255.0) as u8
}

fn blend(dst: Color, src: Color) -> Color {
    let alpha = src.alpha as u32;
    let inverse = 255 - alpha;
    let channel = |d: u8, s: u8| ((d as u32 * inverse + s as u32 * alpha + 127) / 255) as u8;
    Color::rgb(
        channel(dst.red, src.red),
        channel(dst.green, src.green),
        channel(dst.blue, src.blue),
    )
}
