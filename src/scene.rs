//! Static Phosphor desktop composition based on the selected visual identity.

use crate::{Color, RenderTarget, text::Typeface};

/// Paint the desktop, file manager, assistant panel, open launcher, and bottom bar.
///
/// This is a visual mockup. Window interaction and the Redox display/input
/// backend have not been implemented yet.
pub fn render_desktop<T: RenderTarget>(surface: &mut T, typeface: &Typeface) {
    let width = surface.width() as f32;
    let height = surface.height() as f32;
    if width < 1.0 || height < 1.0 {
        return;
    }

    let x = |value: f32| (value * width) as i32;
    let y = |value: f32| (value * height) as i32;
    let w = |value: f32| (value * width).max(1.0) as u32;
    let h = |value: f32| (value * height).max(1.0) as u32;
    let round = |surface: &mut dyn RenderTarget,
                 bx: f32,
                 by: f32,
                 bw: f32,
                 bh: f32,
                 radius: f32,
                 color: Color| {
        surface.fill_round_rect(x(bx), y(by), w(bw), h(bh), h(radius).min(w(radius)), color);
    };
    let rect =
        |surface: &mut dyn RenderTarget, bx: f32, by: f32, bw: f32, bh: f32, color: Color| {
            surface.fill_rect(x(bx), y(by), w(bw), h(bh), color);
        };
    let ellipse =
        |surface: &mut dyn RenderTarget, bx: f32, by: f32, bw: f32, bh: f32, color: Color| {
            surface.fill_ellipse(x(bx), y(by), w(bw), h(bh), color);
        };
    let label =
        |surface: &mut dyn RenderTarget, bx: f32, by: f32, size: f32, value: &str, color: Color| {
            typeface.draw(surface, x(bx), y(by), height * size, value, color);
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
        label(
            surface,
            0.019,
            yy + 0.066,
            0.017,
            if index == 0 {
                "Mes fichiers"
            } else {
                "Corbeille"
            },
            Color::rgb(255, 255, 255),
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
        0.069,
        0.333,
        0.037,
        0.009,
        Color::rgb(255, 255, 253),
    );
    for bx in [0.665, 0.693, 0.72] {
        round(surface, bx, 0.079, 0.008, 0.004, 0.002, ink);
    }
    label(
        surface,
        0.322,
        0.071,
        0.017,
        "Accueil  ›  Documents  ›  Projets",
        muted,
    );

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
        label(
            surface,
            0.284,
            yy - 0.006,
            0.018,
            [
                "Accueil",
                "Documents",
                "Téléchargements",
                "Images",
                "Musique",
                "Bureau",
            ][index],
            if index == 1 { ink } else { muted },
        );
    }

    // File-list headings and rows; the second row is selected.
    label(surface, 0.397, 0.133, 0.015, "Nom", muted);
    label(surface, 0.56, 0.133, 0.015, "Modifié", muted);
    label(surface, 0.663, 0.133, 0.015, "Taille", muted);
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
        label(
            surface,
            0.43,
            yy - 0.004,
            0.019,
            [
                "Maquette-v3",
                "Notes-de-cours",
                "Budget-2026",
                "Présentation",
                "Photos-équipe",
                "Rapport-final",
            ][index],
            ink,
        );
        label(
            surface,
            0.56,
            yy - 0.004,
            0.016,
            [
                "Aujourd'hui, 14:12",
                "Aujourd'hui, 13:40",
                "Aujourd'hui, 11:05",
                "Hier, 18:22",
                "Hier, 09:15",
                "18 nov., 16:48",
            ][index],
            muted,
        );
        label(
            surface,
            0.663,
            yy - 0.004,
            0.016,
            [
                "12 éléments",
                "86 Ko",
                "214 Ko",
                "3,4 Mo",
                "28 Mo",
                "1,1 Mo",
            ][index],
            muted,
        );
    }

    // Assistant panel, visually linked to the selected document.
    round(
        surface,
        0.755,
        0.094,
        0.232,
        0.48,
        0.012,
        Color::rgba(0, 0, 0, 46),
    );
    round(surface, 0.752, 0.09, 0.232, 0.48, 0.012, window);
    round(surface, 0.752, 0.09, 0.232, 0.053, 0.012, panel_header);
    rect(surface, 0.752, 0.133, 0.232, 0.01, panel_header);
    ellipse(surface, 0.77, 0.111, 0.009, 0.016, teal);
    label(surface, 0.787, 0.098, 0.021, "Assistant", ink);
    ellipse(surface, 0.968, 0.111, 0.009, 0.016, ink);
    round(surface, 0.766, 0.164, 0.176, 0.032, 0.016, icon_bg);
    label(
        surface,
        0.78,
        0.169,
        0.015,
        "Traitement local, rien n'est envoyé",
        teal,
    );
    round(surface, 0.796, 0.208, 0.17, 0.047, 0.012, teal);
    label(
        surface,
        0.805,
        0.215,
        0.017,
        "Résume le fichier Notes-de-cours.",
        Color::rgb(255, 255, 255),
    );
    round(
        surface,
        0.766,
        0.266,
        0.2,
        0.092,
        0.012,
        Color::rgb(240, 238, 233),
    );
    label(
        surface,
        0.78,
        0.278,
        0.018,
        "Trois chapitres, deux exercices à rendre",
        ink,
    );
    label(
        surface,
        0.78,
        0.304,
        0.018,
        "jeudi et un point à revoir sur les index.",
        ink,
    );
    label(
        surface,
        0.78,
        0.33,
        0.018,
        "Je peux préparer une fiche de révision.",
        ink,
    );
    for (bx, bw) in [(0.766, 0.088), (0.861, 0.088)] {
        round(
            surface,
            bx,
            0.372,
            bw,
            0.04,
            0.02,
            Color::rgb(255, 255, 253),
        );
        label(
            surface,
            bx + 0.012,
            0.379,
            0.016,
            if bx < 0.8 {
                "Ouvrir le fichier"
            } else {
                "Créer la fiche"
            },
            ink,
        );
    }
    rect(
        surface,
        0.752,
        0.49,
        0.232,
        0.0015,
        Color::rgb(226, 225, 220),
    );
    round(
        surface,
        0.765,
        0.505,
        0.207,
        0.048,
        0.009,
        Color::rgb(255, 255, 253),
    );
    label(
        surface,
        0.776,
        0.513,
        0.016,
        "Demander à l'assistant",
        muted,
    );
    round(surface, 0.947, 0.512, 0.018, 0.034, 0.006, teal);

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
    label(
        surface,
        0.06,
        0.359,
        0.021,
        "Rechercher apps, fichiers, paramètres, actions",
        muted,
    );
    label(surface, 0.025, 0.414, 0.021, "Épinglés", ink);
    label(surface, 0.297, 0.414, 0.018, "Toutes les apps", teal);

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
        draw_app_icon(
            surface,
            x(bx),
            y(by),
            w(0.032),
            h(0.051),
            index,
            app_colors[index],
        );
        label(
            surface,
            bx - 0.005,
            by + 0.056,
            0.016,
            [
                "Fichiers",
                "Web",
                "Courrier",
                "Agenda",
                "Terminal",
                "Texte",
                "Tableur",
                "Diapos",
                "Photos",
                "Musique",
                "Boutique",
                "Paramètres",
            ][index],
            ink,
        );
    }

    label(surface, 0.025, 0.691, 0.021, "Récents", ink);
    for index in 0..4 {
        let bx = 0.034 + (index % 2) as f32 * 0.178;
        let by = 0.748 + (index / 2) as f32 * 0.064;
        round(surface, bx, by, 0.025, 0.042, 0.007, icon_bg);
        round(surface, bx + 0.007, by + 0.012, 0.011, 0.018, 0.002, teal);
        label(
            surface,
            bx + 0.034,
            by + 0.003,
            0.017,
            [
                "Notes-de-cours",
                "Budget-2026",
                "Présentation-soutenance",
                "Photos-équipe",
            ][index],
            ink,
        );
        label(
            surface,
            bx + 0.034,
            by + 0.023,
            0.014,
            ["Il y a 12 min", "Il y a 1 h", "Hier", "Hier"][index],
            muted,
        );
    }
    rect(
        surface,
        0.025,
        0.86,
        0.354,
        0.0015,
        Color::rgb(226, 225, 220),
    );
    ellipse(surface, 0.029, 0.873, 0.023, 0.034, teal);
    label(surface, 0.061, 0.876, 0.02, "Mon compte", ink);
    ellipse(surface, 0.353, 0.875, 0.018, 0.027, ink);

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
    label(
        surface,
        0.078,
        0.951,
        0.019,
        "Rechercher",
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
    label(
        surface,
        0.942,
        0.946,
        0.019,
        "15:30",
        Color::rgb(255, 255, 255),
    );
    label(
        surface,
        0.939,
        0.97,
        0.015,
        "21 nov. 2026",
        Color::rgb(255, 255, 255),
    );
}

fn draw_app_icon(
    surface: &mut dyn RenderTarget,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    index: usize,
    tile: Color,
) {
    let w = width as i32;
    let h = height as i32;
    let left = x + w / 4;
    let right = x + w * 3 / 4;
    let top = y + h / 4;
    let bottom = y + h * 3 / 4;
    let center_x = x + w / 2;
    let center_y = y + h / 2;
    let white = Color::rgb(255, 255, 255);
    let stroke = (w / 13).max(1) as u32;
    let line = |surface: &mut dyn RenderTarget, a, b| surface.draw_line(a, b, stroke, white);

    match index {
        0 => {
            // Folder
            line(surface, (left, top + 2), (center_x - 1, top + 2));
            line(surface, (center_x - 1, top + 2), (center_x + 2, top + 5));
            line(surface, (center_x + 2, top + 5), (right, top + 5));
            line(surface, (right, top + 5), (right, bottom));
            line(surface, (right, bottom), (left, bottom));
            line(surface, (left, bottom), (left, top + 2));
        }
        1 => {
            // Globe
            let diameter = (w / 2).max(3) as u32;
            let gx = center_x - diameter as i32 / 2;
            let gy = center_y - diameter as i32 / 2;
            surface.fill_ellipse(gx, gy, diameter, diameter, white);
            let inset = (diameter / 5).max(1);
            surface.fill_ellipse(
                gx + inset as i32,
                gy + inset as i32,
                diameter - 2 * inset,
                diameter - 2 * inset,
                tile,
            );
            line(surface, (center_x, gy), (center_x, gy + diameter as i32));
            line(surface, (gx, center_y), (gx + diameter as i32, center_y));
        }
        2 => {
            // Envelope
            line(surface, (left, top + 2), (right, top + 2));
            line(surface, (right, top + 2), (right, bottom - 1));
            line(surface, (right, bottom - 1), (left, bottom - 1));
            line(surface, (left, bottom - 1), (left, top + 2));
            line(surface, (left, top + 3), (center_x, center_y + 1));
            line(surface, (center_x, center_y + 1), (right, top + 3));
        }
        3 => {
            // Calendar
            line(surface, (left, top + 3), (right, top + 3));
            line(surface, (right, top + 3), (right, bottom));
            line(surface, (right, bottom), (left, bottom));
            line(surface, (left, bottom), (left, top + 3));
            line(surface, (left, top + 1), (left, top + 5));
            line(surface, (right, top + 1), (right, top + 5));
            line(surface, (left, center_y), (right, center_y));
        }
        4 => {
            // Terminal
            line(surface, (left, top + 2), (right, top + 2));
            line(surface, (right, top + 2), (right, bottom));
            line(surface, (right, bottom), (left, bottom));
            line(surface, (left, bottom), (left, top + 2));
            line(surface, (left + 2, top + 5), (left + 5, top + 8));
            line(surface, (left + 5, top + 8), (left + 2, top + 11));
            line(surface, (left + 7, bottom - 3), (right - 2, bottom - 3));
        }
        5 => {
            // Document
            line(surface, (left + 2, top), (right - 3, top));
            line(surface, (right - 3, top), (right, top + 3));
            line(surface, (right, top + 3), (right, bottom));
            line(surface, (right, bottom), (left + 2, bottom));
            line(surface, (left + 2, bottom), (left + 2, top));
            line(surface, (left + 5, center_y), (right - 3, center_y));
            line(surface, (left + 5, center_y + 4), (right - 3, center_y + 4));
        }
        6 => {
            // Spreadsheet
            line(surface, (left, top), (right, top));
            line(surface, (right, top), (right, bottom));
            line(surface, (right, bottom), (left, bottom));
            line(surface, (left, bottom), (left, top));
            line(surface, (left, center_y), (right, center_y));
            line(surface, (center_x, top), (center_x, bottom));
        }
        7 => {
            // Presentation screen
            line(surface, (left, top), (right, top));
            line(surface, (right, top), (right, bottom - 3));
            line(surface, (right, bottom - 3), (left, bottom - 3));
            line(surface, (left, bottom - 3), (left, top));
            line(surface, (center_x, bottom - 3), (center_x, bottom + 2));
            line(
                surface,
                (center_x - 3, bottom + 2),
                (center_x + 3, bottom + 2),
            );
        }
        8 => {
            // Photo
            line(surface, (left, top), (right, top));
            line(surface, (right, top), (right, bottom));
            line(surface, (right, bottom), (left, bottom));
            line(surface, (left, bottom), (left, top));
            surface.fill_ellipse(left + 3, top + 3, stroke + 1, stroke + 1, white);
            line(surface, (left + 2, bottom - 3), (center_x, center_y));
            line(surface, (center_x, center_y), (right - 2, bottom - 3));
        }
        9 => {
            // Music note
            line(surface, (center_x + 3, top), (center_x + 3, bottom - 3));
            line(surface, (center_x + 3, top), (right, top + 2));
            line(surface, (right, top + 2), (right, bottom - 4));
            surface.fill_ellipse(
                left + 1,
                bottom - 5,
                w.max(3) as u32 / 5,
                h.max(3) as u32 / 6,
                white,
            );
            surface.fill_ellipse(
                right - 3,
                bottom - 6,
                w.max(3) as u32 / 5,
                h.max(3) as u32 / 6,
                white,
            );
        }
        10 => {
            // App store / shop
            line(surface, (left, top + 5), (right, top + 5));
            line(surface, (left + 2, top + 5), (left + 2, bottom));
            line(surface, (right - 2, top + 5), (right - 2, bottom));
            line(surface, (left + 2, bottom), (right - 2, bottom));
            line(surface, (center_x, center_y + 2), (center_x, bottom));
            line(surface, (left - 1, top + 4), (left + 2, top));
            line(surface, (left + 2, top), (right - 2, top));
            line(surface, (right - 2, top), (right + 1, top + 4));
        }
        _ => {
            // Settings
            let diameter = (w / 2).max(3) as u32;
            let gx = center_x - diameter as i32 / 2;
            let gy = center_y - diameter as i32 / 2;
            surface.fill_ellipse(gx, gy, diameter, diameter, white);
            let inset = (diameter / 3).max(1);
            surface.fill_ellipse(
                gx + inset as i32,
                gy + inset as i32,
                diameter - 2 * inset,
                diameter - 2 * inset,
                tile,
            );
        }
    }
}
