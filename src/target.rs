//! Rendering contract shared by Phosphor and its platform adapters.

use crate::{Color, Surface};

/// A clipped pixel target that can receive Phosphor's desktop scene.
///
/// Platform code can implement this for its own framebuffer surface, keeping
/// Phosphor independent from Redox schemes and PrismOS framebuffer details.
/// Colors are straight RGBA; methods that paint shapes must honor alpha.
pub trait RenderTarget {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn blend_pixel(&mut self, x: i32, y: i32, color: Color);
    fn fill_rect(&mut self, x: i32, y: i32, width: u32, height: u32, color: Color);
    fn fill_round_rect(
        &mut self,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        radius: u32,
        color: Color,
    );

    fn clear(&mut self, color: Color) {
        self.fill_rect(
            0,
            0,
            self.width().min(u32::MAX as usize) as u32,
            self.height().min(u32::MAX as usize) as u32,
            color,
        );
    }

    fn vertical_gradient(&mut self, top: Color, bottom: Color) {
        let height = self.height();
        let width = self.width().min(u32::MAX as usize) as u32;
        if height == 0 || width == 0 {
            return;
        }
        for row in 0..height.min(u32::MAX as usize) {
            let t = if height == 1 {
                0.0
            } else {
                row as f32 / (height - 1) as f32
            };
            let color = Color::rgb(
                lerp(top.red, bottom.red, t),
                lerp(top.green, bottom.green, t),
                lerp(top.blue, bottom.blue, t),
            );
            self.fill_rect(0, row as i32, width, 1, color);
        }
    }

    fn fill_ellipse(&mut self, x: i32, y: i32, width: u32, height: u32, color: Color) {
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
        let right = right.min(self.width().min(i32::MAX as usize) as i32);
        let bottom = bottom.min(self.height().min(i32::MAX as usize) as i32);
        let rx = width as f64 / 2.0;
        let ry = height as f64 / 2.0;
        let cx = x as f64 + rx;
        let cy = y as f64 + ry;

        for py in top..bottom {
            for px in left..right {
                let dx = (px as f64 + 0.5 - cx) / rx;
                let dy = (py as f64 + 0.5 - cy) / ry;
                if dx * dx + dy * dy <= 1.0 {
                    self.blend_pixel(px, py, color);
                }
            }
        }
    }

    fn draw_line(&mut self, start: (i32, i32), end: (i32, i32), thickness: u32, color: Color) {
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
}

impl RenderTarget for Surface {
    fn width(&self) -> usize {
        Surface::width(self)
    }
    fn height(&self) -> usize {
        Surface::height(self)
    }
    fn blend_pixel(&mut self, x: i32, y: i32, color: Color) {
        Surface::blend_pixel(self, x, y, color);
    }
    fn fill_rect(&mut self, x: i32, y: i32, width: u32, height: u32, color: Color) {
        Surface::fill_rect(self, x, y, width, height, color);
    }
    fn fill_round_rect(
        &mut self,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        radius: u32,
        color: Color,
    ) {
        Surface::fill_round_rect(self, x, y, width, height, radius, color);
    }
    fn clear(&mut self, color: Color) {
        Surface::clear(self, color);
    }
    fn vertical_gradient(&mut self, top: Color, bottom: Color) {
        Surface::vertical_gradient(self, top, bottom);
    }
    fn fill_ellipse(&mut self, x: i32, y: i32, width: u32, height: u32, color: Color) {
        Surface::fill_ellipse(self, x, y, width, height, color);
    }
    fn draw_line(&mut self, start: (i32, i32), end: (i32, i32), thickness: u32, color: Color) {
        Surface::draw_line(self, start, end, thickness, color);
    }
}

fn lerp(from: u8, to: u8, t: f32) -> u8 {
    let value = from as f32 + (to as f32 - from as f32) * t;
    (value + 0.5).clamp(0.0, 255.0) as u8
}
