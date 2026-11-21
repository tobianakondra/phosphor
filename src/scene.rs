//! Static Phosphor desktop composition based on the selected visual identity.

use crate::{Color, Surface};

/// Paint the desktop, file manager, assistant panel, open launcher, and bottom bar.
///
/// This is a visual mockup. Text, real icons, window interaction, and the Redox
/// display/input backend have not been implemented yet.
pub fn render_desktop(surface: &mut Surface) {
    let width = surface.width() as f32;
    let height = surface.height() as f32;
    if width < 1.0 || height < 1.0 {
        return;
    }

    let x = |value: f32| (value * width) as i32;
    let y = |value: f32| (value * height) as i32;
    let w = |value: f32| (value * width).max(1.0) as u32;
    let h = |value: f32| (value * height).max(1.0) as u32;
    let round =
        |surface: &mut Surface, bx: f32, by: f32, bw: f32, bh: f32, radius: f32, color: Color| {
            surface.fill_round_rect(x(bx), y(by), w(bw), h(bh), h(radius).min(w(radius)), color);
        };
    let rect = |surface: &mut Surface, bx: f32, by: f32, bw: f32, bh: f32, color: Color| {
        surface.fill_rect(x(bx), y(by), w(bw), h(bh), color);
    };
    let ellipse = |surface: &mut Surface, bx: f32, by: f32, bw: f32, bh: f32, color: Color| {
        surface.fill_ellipse(x(bx), y(by), w(bw), h(bh), color);
    };

    // Deep teal wallpaper with soft green light in the lower-right corner.
    surface.vertical_gradient(Color::rgb(8, 46, 49), Color::rgb(17, 91, 77));
    ellipse(
        surface,
        0.39,
        0.08,
        0.57,
        0.79,
        Color::rgba(72, 132, 98, 24),
    );
    ellipse(
        surface,
        0.55,
        0.42,
        0.48,
        0.53,
        Color::rgba(21, 134, 103, 22),
    );

    let ink = Color::rgb(30, 37, 35);
    let muted = Color::rgb(99, 108, 104);
    let teal = Color::rgb(0, 126, 103);
    let icon_bg = Color::rgb(225, 239, 234);
    let window = Color::rgb(250, 249, 246);
    let panel_header = Color::rgb(242, 240, 235);

    // Desktop shortcuts at the upper-left.
    for (index, yy) in [0.03, 0.135].into_iter().enumerate() {
        round(
            surface,
            0.027,
            yy,
            0.034,
            0.055,
            0.012,
            Color::rgba(226, 244, 238, 52),
        );
        round(
            surface,
            0.037,
            yy + 0.017,
            0.015,
            0.019,
            0.003,
            if index == 0 {
                Color::rgb(235, 250, 246)
            } else {
                Color::rgb(245, 248, 241)
            },
        );
        rect(
            surface,
            0.035,
            yy + 0.024,
            0.019,
            0.003,
            Color::rgb(235, 250, 246),
        );
        round(
            surface,
            0.019,
            yy + 0.066,
            0.05,
            0.006,
            0.003,
            Color::rgba(255, 255, 255, 220),
        );
    }

    // File manager window and its title/navigation bar.
    round(
        surface,
        0.252,
        0.054,
        0.492,
        0.731,
        0.012,
        Color::rgba(0, 0, 0, 48),
    );
    round(surface, 0.25, 0.05, 0.492, 0.731, 0.012, window);
    round(surface, 0.25, 0.05, 0.492, 0.075, 0.012, panel_header);
    rect(surface, 0.25, 0.113, 0.492, 0.012, panel_header);
    ellipse(surface, 0.267, 0.079, 0.006, 0.016, ink);
    ellipse(surface, 0.297, 0.079, 0.006, 0.016, muted);
    round(
        surface,
        0.315,
        0.061,
        0.333,
        0.058,
        0.009,
        Color::rgb(255, 255, 253),
    );
    round(surface, 0.328, 0.089, 0.07, 0.004, 0.002, muted);
    round(surface, 0.403, 0.089, 0.055, 0.004, 0.002, muted);
    round(surface, 0.463, 0.089, 0.041, 0.004, 0.002, ink);
    for bx in [0.665, 0.693, 0.72] {
        round(surface, bx, 0.079, 0.008, 0.004, 0.002, ink);
    }

    // Sidebar and location highlights.
    rect(surface, 0.25, 0.125, 0.13, 0.656, Color::rgb(247, 246, 242));
    rect(
        surface,
        0.38,
        0.125,
        0.001,
        0.656,
        Color::rgb(226, 225, 219),
    );
    for (index, yy) in [0.151, 0.192, 0.233, 0.274, 0.315, 0.356]
        .into_iter()
        .enumerate()
    {
        if index == 1 {
            round(surface, 0.257, yy - 0.009, 0.116, 0.037, 0.008, icon_bg);
        }
        ellipse(
            surface,
            0.267,
            yy,
            0.009,
            0.016,
            if index == 1 { teal } else { ink },
        );
        round(
            surface,
            0.284,
            yy + 0.006,
            0.061,
            0.005,
            0.002,
            if index == 1 { ink } else { muted },
        );
    }

    // File-list headings and rows; the second row is selected.
    for (bx, bw) in [(0.397, 0.044), (0.56, 0.051), (0.663, 0.036)] {
        round(surface, bx, 0.147, bw, 0.006, 0.002, muted);
    }
    rect(
        surface,
        0.392,
        0.169,
        0.337,
        0.0015,
        Color::rgb(227, 226, 221),
    );
    for (index, yy) in [0.184, 0.239, 0.294, 0.349, 0.404, 0.459]
        .into_iter()
        .enumerate()
    {
        if index == 1 {
            round(
                surface,
                0.392,
                yy - 0.009,
                0.333,
                0.05,
                0.008,
                Color::rgb(230, 241, 236),
            );
        }
        round(surface, 0.399, yy, 0.021, 0.031, 0.006, icon_bg);
        round(surface, 0.406, yy + 0.008, 0.008, 0.015, 0.002, teal);
        round(surface, 0.43, yy + 0.009, 0.087, 0.006, 0.002, ink);
        round(surface, 0.56, yy + 0.009, 0.06, 0.005, 0.002, muted);
        round(surface, 0.663, yy + 0.009, 0.033, 0.005, 0.002, muted);
    }

    // Assistant panel, visually linked to the selected document.
    round(
        surface,
        0.755,
        0.094,
        0.232,
        0.65,
        0.012,
        Color::rgba(0, 0, 0, 46),
    );
    round(surface, 0.752, 0.09, 0.232, 0.65, 0.012, window);
    round(surface, 0.752, 0.09, 0.232, 0.061, 0.012, panel_header);
    rect(surface, 0.752, 0.139, 0.232, 0.012, panel_header);
    ellipse(surface, 0.77, 0.111, 0.009, 0.016, teal);
    round(surface, 0.787, 0.117, 0.058, 0.006, 0.002, ink);
    ellipse(surface, 0.968, 0.111, 0.009, 0.016, ink);
    round(surface, 0.766, 0.17, 0.176, 0.036, 0.018, icon_bg);
    round(surface, 0.783, 0.185, 0.137, 0.006, 0.003, teal);
    round(surface, 0.796, 0.226, 0.17, 0.073, 0.012, teal);
    round(
        surface,
        0.81,
        0.25,
        0.126,
        0.007,
        0.003,
        Color::rgb(255, 255, 255),
    );
    round(
        surface,
        0.766,
        0.319,
        0.2,
        0.14,
        0.012,
        Color::rgb(240, 238, 233),
    );
    for (yy, ww) in [(0.343, 0.16), (0.369, 0.17), (0.395, 0.145), (0.421, 0.11)] {
        round(surface, 0.78, yy, ww, 0.006, 0.002, ink);
    }
    for (bx, bw) in [(0.766, 0.088), (0.861, 0.088)] {
        round(
            surface,
            bx,
            0.474,
            bw,
            0.04,
            0.02,
            Color::rgb(255, 255, 253),
        );
        round(surface, bx + 0.017, 0.491, bw - 0.034, 0.006, 0.003, ink);
    }
    rect(
        surface,
        0.752,
        0.694,
        0.232,
        0.0015,
        Color::rgb(226, 225, 220),
    );
    round(
        surface,
        0.765,
        0.707,
        0.207,
        0.048,
        0.009,
        Color::rgb(255, 255, 253),
    );
    round(surface, 0.779, 0.728, 0.13, 0.006, 0.002, muted);
    round(surface, 0.947, 0.715, 0.018, 0.034, 0.006, teal);

    // Large launcher overlay with search, pinned apps, recents, account and power.
    round(
        surface,
        0.012,
        0.33,
        0.385,
        0.597,
        0.014,
        Color::rgba(0, 0, 0, 52),
    );
    round(surface, 0.01, 0.325, 0.385, 0.597, 0.014, window);
    round(
        surface,
        0.025,
        0.347,
        0.354,
        0.054,
        0.012,
        Color::rgb(240, 238, 233),
    );
    ellipse(surface, 0.039, 0.366, 0.01, 0.018, muted);
    round(
        surface,
        0.06,
        0.374,
        0.24,
        0.006,
        0.003,
        Color::rgb(132, 137, 133),
    );
    round(surface, 0.025, 0.422, 0.061, 0.009, 0.003, ink);
    round(surface, 0.297, 0.423, 0.08, 0.007, 0.003, teal);

    let app_colors = [
        Color::rgb(0, 126, 103),
        Color::rgb(45, 111, 219),
        Color::rgb(192, 61, 13),
        Color::rgb(119, 54, 224),
        Color::rgb(33, 43, 57),
        Color::rgb(0, 111, 165),
        Color::rgb(23, 132, 65),
        Color::rgb(184, 81, 10),
        Color::rgb(192, 28, 89),
        Color::rgb(102, 49, 206),
        Color::rgb(0, 120, 111),
        Color::rgb(70, 83, 104),
    ];
    for index in 0..12 {
        let col = index % 6;
        let row = index / 6;
        let bx = 0.037 + col as f32 * 0.056;
        let by = 0.478 + row as f32 * 0.104;
        round(surface, bx, by, 0.032, 0.051, 0.008, app_colors[index]);
        // Simple white glyph marks stand in for the final icon set.
        round(
            surface,
            bx + 0.009,
            by + 0.017,
            0.014,
            0.018,
            0.003,
            Color::rgb(255, 255, 255),
        );
        round(
            surface,
            bx + 0.012,
            by + 0.021,
            0.008,
            0.01,
            0.002,
            app_colors[index],
        );
        round(surface, bx - 0.003, by + 0.058, 0.039, 0.005, 0.002, ink);
    }

    round(surface, 0.025, 0.704, 0.052, 0.009, 0.003, ink);
    for index in 0..4 {
        let bx = 0.034 + (index % 2) as f32 * 0.178;
        let by = 0.748 + (index / 2) as f32 * 0.064;
        round(surface, bx, by, 0.025, 0.042, 0.007, icon_bg);
        round(surface, bx + 0.007, by + 0.012, 0.011, 0.018, 0.002, teal);
        round(surface, bx + 0.034, by + 0.009, 0.105, 0.006, 0.002, ink);
        round(surface, bx + 0.034, by + 0.022, 0.075, 0.005, 0.002, muted);
    }
    rect(
        surface,
        0.025,
        0.873,
        0.354,
        0.0015,
        Color::rgb(226, 225, 220),
    );
    ellipse(surface, 0.029, 0.887, 0.023, 0.034, teal);
    round(surface, 0.061, 0.901, 0.075, 0.007, 0.003, ink);
    ellipse(surface, 0.353, 0.89, 0.018, 0.027, ink);

    // Persistent bottom panel: launcher, search, pinned apps, system tray and clock.
    rect(surface, 0.0, 0.935, 1.0, 0.065, Color::rgb(9, 36, 34));
    round(surface, 0.01, 0.944, 0.031, 0.047, 0.009, teal);
    for (row, col) in [
        (0.018, 0.017),
        (0.018, 0.028),
        (0.037, 0.017),
        (0.037, 0.028),
    ] {
        round(
            surface,
            col,
            row,
            0.006,
            0.009,
            0.0015,
            Color::rgb(255, 255, 255),
        );
    }
    round(
        surface,
        0.047,
        0.944,
        0.167,
        0.047,
        0.023,
        Color::rgb(47, 70, 67),
    );
    ellipse(
        surface,
        0.061,
        0.958,
        0.009,
        0.015,
        Color::rgb(240, 249, 245),
    );
    round(
        surface,
        0.078,
        0.964,
        0.051,
        0.006,
        0.003,
        Color::rgb(255, 255, 255),
    );

    let dock_colors = [
        teal,
        Color::rgb(40, 105, 207),
        Color::rgb(184, 61, 13),
        Color::rgb(0, 126, 103),
        Color::rgb(112, 52, 214),
        Color::rgb(68, 81, 101),
    ];
    for (index, color) in dock_colors.into_iter().enumerate() {
        let bx = 0.4 + index as f32 * 0.047;
        if index == 0 || index == 4 {
            round(
                surface,
                bx,
                0.944,
                0.032,
                0.047,
                0.009,
                Color::rgb(39, 62, 59),
            );
            round(
                surface,
                bx + 0.009,
                0.985,
                0.014,
                0.003,
                0.001,
                Color::rgb(76, 216, 177),
            );
        }
        round(surface, bx + 0.009, 0.958, 0.014, 0.019, 0.003, color);
    }
    for index in 0..3 {
        ellipse(
            surface,
            0.87 + index as f32 * 0.018,
            0.959,
            0.009,
            0.012,
            Color::rgb(239, 247, 244),
        );
    }
    round(
        surface,
        0.917,
        0.962,
        0.019,
        0.011,
        0.003,
        Color::rgb(239, 247, 244),
    );
    round(
        surface,
        0.939,
        0.951,
        0.037,
        0.006,
        0.003,
        Color::rgb(255, 255, 255),
    );
    round(
        surface,
        0.939,
        0.971,
        0.037,
        0.006,
        0.003,
        Color::rgb(255, 255, 255),
    );
}
