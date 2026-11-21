//! TrueType text rasterization used by desktop labels and controls.

use crate::{Color, RenderTarget};

/// A loaded TrueType font face.
pub struct Typeface {
    font: fontdue::Font,
}

impl Typeface {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, &'static str> {
        let font = fontdue::Font::from_bytes(bytes, fontdue::FontSettings::default())?;
        Ok(Self { font })
    }

    /// Draw text with its top edge at `(x, y)`. The renderer clips at the surface.
    pub fn draw<T: RenderTarget + ?Sized>(
        &self,
        surface: &mut T,
        x: i32,
        y: i32,
        size: f32,
        text: &str,
        color: Color,
    ) {
        let size = size.max(1.0);
        let ascent = self
            .font
            .horizontal_line_metrics(size)
            .map(|metrics| ceil_to_i32(metrics.ascent))
            .unwrap_or_else(|| ceil_to_i32(size));
        let baseline = y.saturating_add(ascent);
        let mut pen_x = x;

        for character in text.chars() {
            let (metrics, bitmap) = self.font.rasterize(character, size);
            let glyph_x = pen_x.saturating_add(metrics.xmin);
            let glyph_y = baseline
                .saturating_sub(metrics.ymin)
                .saturating_sub(metrics.height as i32);

            for row in 0..metrics.height {
                for column in 0..metrics.width {
                    let coverage = bitmap[row * metrics.width + column];
                    if coverage == 0 {
                        continue;
                    }
                    let alpha = (u16::from(coverage) * u16::from(color.alpha) / 255) as u8;
                    surface.blend_pixel(
                        glyph_x.saturating_add(column as i32),
                        glyph_y.saturating_add(row as i32),
                        Color::rgba(color.red, color.green, color.blue, alpha),
                    );
                }
            }
            pen_x = pen_x.saturating_add(round_to_i32(metrics.advance_width));
        }
    }
}

fn ceil_to_i32(value: f32) -> i32 {
    let truncated = value as i32;
    if (truncated as f32) < value {
        truncated.saturating_add(1)
    } else {
        truncated
    }
}

fn round_to_i32(value: f32) -> i32 {
    if value >= 0.0 {
        (value + 0.5) as i32
    } else {
        (value - 0.5) as i32
    }
}
