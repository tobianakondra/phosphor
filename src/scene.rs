//! Static Phosphor desktop composition based on the selected visual identity.
//!
//! ROLE: Reproduce the `Bureau et lanceur@2x.png` mockup as faithfully as
//! possible using only the `RenderTarget` primitives (rect, round_rect,
//! ellipse, line, gradient, text). All coordinates are expressed as fractions
//! of the target width/height so the scene scales to any resolution.
//!
//! Layout reference (1366×868 logical, @2x source image):
//!   - Taskbar  : bottom 6.5 % of height
//!   - Desktop  : remaining area, teal gradient wallpaper
//!   - Shortcuts: top-left corner, two icon+label pairs
//!   - File mgr : centre, ~49 % wide × ~73 % tall, starts at y≈5 %
//!   - Assistant: right side, ~24 % wide × ~48 % tall, starts at y≈9 %
//!   - Launcher : bottom-left overlay, ~38 % wide × ~56 % tall

use crate::{Color, RenderTarget, text::Typeface};

/// Paint the full desktop scene into `surface`.
pub fn render_desktop<T: RenderTarget>(surface: &mut T, typeface: &Typeface) {
    let width = surface.width() as f32;
    let height = surface.height() as f32;
    if width < 1.0 || height < 1.0 {
        return;
    }

    // ── Coordinate helpers (all units are fractions of width / height) ──────
    let x = |v: f32| (v * width) as i32;
    let y = |v: f32| (v * height) as i32;
    let w = |v: f32| (v * width).max(1.0) as u32;
    let h = |v: f32| (v * height).max(1.0) as u32;

    // Rounded rect helper: radius is taken as a fraction of height so it
    // scales proportionally; clamped to half the smaller side.
    let round = |surface: &mut dyn RenderTarget,
                 bx: f32,
                 by: f32,
                 bw: f32,
                 bh: f32,
                 radius: f32,
                 color: Color| {
        let r = h(radius).min(w(radius));
        surface.fill_round_rect(x(bx), y(by), w(bw), h(bh), r, color);
    };

    let rect = |surface: &mut dyn RenderTarget,
                bx: f32,
                by: f32,
                bw: f32,
                bh: f32,
                color: Color| {
        surface.fill_rect(x(bx), y(by), w(bw), h(bh), color);
    };

    let ellipse = |surface: &mut dyn RenderTarget,
                   bx: f32,
                   by: f32,
                   bw: f32,
                   bh: f32,
                   color: Color| {
        surface.fill_ellipse(x(bx), y(by), w(bw), h(bh), color);
    };

    let label = |surface: &mut dyn RenderTarget,
                 bx: f32,
                 by: f32,
                 size: f32,
                 value: &str,
                 color: Color| {
        typeface.draw(surface, x(bx), y(by), height * size, value, color);
    };

    // ── Palette ──────────────────────────────────────────────────────────────
    let ink         = Color::rgb(30, 37, 35);       // near-black text
    let muted       = Color::rgb(99, 108, 104);     // secondary text / icons
    let teal        = Color::rgb(0, 126, 103);      // accent
    let teal_light  = Color::rgb(76, 216, 177);     // active indicator dot
    let icon_bg     = Color::rgb(225, 239, 234);    // soft-green icon backing
    let window      = Color::rgb(250, 249, 246);    // window / panel background
    let panel_hdr   = Color::rgb(242, 240, 235);    // title-bar / sidebar background
    let white       = Color::rgb(255, 255, 255);

    // ════════════════════════════════════════════════════════════════════════
    // 1. WALLPAPER — deep teal vertical gradient + two ambient glows
    // ════════════════════════════════════════════════════════════════════════
    surface.vertical_gradient(Color::rgb(8, 46, 49), Color::rgb(17, 91, 77));
    // Large diffuse glow, upper-centre-right
    ellipse(surface, 0.39, 0.08, 0.57, 0.79, Color::rgba(72, 132, 98, 24));
    // Smaller concentrated glow, lower-right
    ellipse(surface, 0.55, 0.42, 0.48, 0.53, Color::rgba(21, 134, 103, 22));

    // ════════════════════════════════════════════════════════════════════════
    // 2. DESKTOP SHORTCUTS — top-left: "Mes fichiers" and "Corbeille"
    //    Maquette: tile ~43×56 px @ 1366×868, so ~3.1 % wide × 6.5 % tall.
    //    First at y≈1.5 %, second at y≈13 %.
    // ════════════════════════════════════════════════════════════════════════
    for (index, ty) in [0.015_f32, 0.130].iter().enumerate() {
        let tx = 0.021_f32;
        let tw = 0.031_f32;   // ~43 px / 1366
        let th = 0.064_f32;   // ~56 px / 868

        // Translucent backing tile
        round(surface, tx, *ty, tw, th, 0.010, Color::rgba(226, 244, 238, 52));

        // Icon centred inside tile
        let icon_w = 0.018_f32;
        let icon_h = 0.034_f32;
        let icon_x = tx + (tw - icon_w) / 2.0;
        let icon_y = ty + 0.008;

        if index == 0 {
            draw_folder_icon(surface, x(icon_x), y(icon_y), w(icon_w), h(icon_h), icon_bg);
        } else {
            draw_trash_icon(surface, x(icon_x), y(icon_y), w(icon_w), h(icon_h), icon_bg);
        }

        // Label centred under the tile
        let name = if index == 0 { "Mes fichiers" } else { "Corbeille" };
        let lx = if index == 0 { tx - 0.003 } else { tx + 0.001 };
        label(surface, lx, ty + th + 0.005, 0.014, name, white);
    }

    // ════════════════════════════════════════════════════════════════════════
    // 3. FILE MANAGER WINDOW
    //    Position: x=25 %, y=5 %  size: 49 % × 73 %
    //    Structure: drop-shadow → window body → title bar → sidebar → list
    // ════════════════════════════════════════════════════════════════════════

    // Drop shadow (offset 2 px down/right, dark translucent)
    round(surface, 0.252, 0.054, 0.492, 0.731, 0.012, Color::rgba(0, 0, 0, 48));
    // Window body
    round(surface, 0.250, 0.050, 0.492, 0.731, 0.012, window);

    // ── Title bar (top 7.5 % of window height) ──────────────────────────────
    round(surface, 0.250, 0.050, 0.492, 0.075, 0.012, panel_hdr);
    // Square the bottom corners of the title bar (cover the rounding)
    rect(surface, 0.250, 0.113, 0.492, 0.012, panel_hdr);

    // Navigation arrows ← and → + breadcrumb + window controls
    // Tous alignés sur le MÊME centre vertical de la titlebar.
    //
    // Titlebar: y=0.050..0.102  → centre = 0.076
    // Bouton nav:  h=0.028 → top = 0.076 - 0.014 = 0.062
    // Breadcrumb:  h=0.032 → top = 0.076 - 0.016 = 0.060
    // Ctrl button: h=0.026 → top = 0.076 - 0.013 = 0.063
    let tb_cy  = 0.076_f32;   // centre vertical titlebar
    let btn_h  = 0.028_f32;   // hauteur bouton nav
    let btn_y  = tb_cy - btn_h / 2.0;
    let btn_w  = 0.016_f32;   // plus petits, plus discrets

    // ← bouton — pas de fond, juste le chevron ink
    draw_chevron_left(surface, x(0.261), y(btn_y), w(btn_w), h(btn_h), ink);

    // → bouton — pas de fond, chevron muted
    draw_chevron_right(surface, x(0.282), y(btn_y), w(btn_w), h(btn_h), muted);

    // Breadcrumb — même centre vertical, capsule blanche
    let bc_h   = 0.032_f32;
    let bc_y   = tb_cy - bc_h / 2.0;    // top = 0.060
    round(surface, 0.312, bc_y, 0.342, bc_h, 0.010, Color::rgb(255, 255, 253));
    // Texte centré verticalement dans la capsule
    label(surface, 0.319, bc_y + (bc_h - 0.016) / 2.0 - 0.002, 0.016,
          "Accueil  \u{203a}  Documents  \u{203a}  Projets", muted);

    // Boutons – □ × — même centre vertical, cercles
    // Diamètre = 0.022 (légèrement plus petit que les nav)
    let ctrl_d = 0.020_f32;   // diamètre
    let ctrl_y = tb_cy - ctrl_d / 2.0;
    let ctrl_configs = [
        (0.676_f32, Color::rgb(210, 207, 200), "\u{2212}"),
        (0.699_f32, Color::rgb(210, 207, 200), "\u{25FB}"),  // □ petit
        (0.722_f32, Color::rgb(220, 76,  60 ), "\u{00D7}"),
    ];
    for (bx, bg_color, symbol) in ctrl_configs {
        // Fond circulaire centré
        ellipse(surface, bx, ctrl_y, ctrl_d, ctrl_d, bg_color);
        // Symbole centré dans le cercle
        let sym_size = 0.012_f32;
        let sym_x = bx + ctrl_d / 2.0 - sym_size * 0.3;
        let sym_y = ctrl_y + ctrl_d / 2.0 - sym_size * 0.6;
        label(surface, sym_x, sym_y, sym_size, symbol,
              if symbol == "\u{00D7}" { white } else { Color::rgb(90, 88, 83) });
    }

    // ── Sidebar ──────────────────────────────────────────────────────────────
    rect(surface, 0.250, 0.125, 0.130, 0.656, Color::rgb(247, 246, 242));
    // Right border of sidebar
    rect(surface, 0.380, 0.125, 0.001, 0.656, Color::rgb(226, 225, 219));

    // Sidebar entries: (label, icon_kind)
    // icon_kind: 0=home, 1=doc, 2=download, 3=image, 4=music, 5=desktop
    let sidebar_items = [
        ("Accueil",         0_usize),
        ("Documents",       1),
        ("Téléchargements", 2),
        ("Images",          3),
        ("Musique",         4),
    ];
    for (index, (name, icon_kind)) in sidebar_items.iter().enumerate() {
        let sy = 0.148 + index as f32 * 0.041;
        let selected = index == 1; // "Documents" is active

        if selected {
            round(surface, 0.256, sy - 0.008, 0.117, 0.036, 0.008, icon_bg);
        }

        // Small sidebar icon (use ellipse as simplified glyph placeholder)
        let icon_color = if selected { teal } else { muted };
        draw_sidebar_icon(surface, x(0.263), y(sy), w(0.012), h(0.020), *icon_kind, icon_color);

        let label_color = if selected { ink } else { muted };
        label(surface, 0.282, sy - 0.004, 0.017, name, label_color);
    }

    // ── File list ─────────────────────────────────────────────────────────────
    // Column headers
    label(surface, 0.397, 0.130, 0.014, "Nom",     muted);
    label(surface, 0.558, 0.130, 0.014, "Modifié", muted);
    label(surface, 0.661, 0.130, 0.014, "Taille",  muted);
    // Separator below headers
    rect(surface, 0.392, 0.148, 0.337, 0.0015, Color::rgb(227, 226, 221));

    // File rows: (name, date, size, icon_type)
    // icon_type: 0=folder, 1=doc-blue, 2=spreadsheet, 3=presentation, 4=photo, 5=doc-blue
    let files = [
        ("Maquette-v3",          "Aujourd'hui, 14:12", "12 éléments", 0_usize),
        ("Notes-de-cours",       "Aujourd'hui, 13:40", "86 Ko",        1),
        ("Budget-2026",          "Aujourd'hui, 11:05", "214 Ko",       2),
        ("Présentation-soutenance","Hier, 18:22",      "3,4 Mo",       3),
        ("Photos-équipe",        "Hier, 09:15",        "28 Mo",        4),
        ("Rapport-final",        "18 nov., 16:48",     "1,1 Mo",       5),
    ];

    // Icon accent colours per file type (matches maquette roughly)
    let file_icon_colors = [
        Color::rgb(0, 126, 103),   // folder teal
        Color::rgb(50, 110, 200),  // doc blue
        Color::rgb(30, 140, 70),   // spreadsheet green
        Color::rgb(50, 110, 200),  // presentation blue
        Color::rgb(190, 50, 100),  // photo pink
        Color::rgb(50, 110, 200),  // doc blue
    ];

    for (index, (name, date, size, icon_type)) in files.iter().enumerate() {
        let ry = 0.178 + index as f32 * 0.054;
        let selected = index == 1;

        if selected {
            round(surface, 0.392, ry - 0.009, 0.337, 0.049, 0.008,
                  Color::rgb(230, 241, 236));
        }

        // File type icon — pas de fond, juste le glyphe coloré
        let ic = file_icon_colors[*icon_type];
        draw_file_icon(surface, x(0.398), y(ry - 0.001), w(0.021), h(0.032),
                       *icon_type, ic);

        label(surface, 0.427, ry - 0.003, 0.017, name,  ink);
        label(surface, 0.558, ry - 0.003, 0.015, date,  muted);
        label(surface, 0.661, ry - 0.003, 0.015, size,  muted);
    }

    // ════════════════════════════════════════════════════════════════════════
    // 4. ASSISTANT PANEL
    //    Maquette: x≈75.5 %, y≈8.5 %,  w≈24.5 %, h≈55 %
    //    Bottom edge ≈ y+h ≈ 55 %  (well above the taskbar)
    // ════════════════════════════════════════════════════════════════════════
    // Drop shadow
    round(surface, 0.758, 0.090, 0.236, 0.560, 0.012, Color::rgba(0, 0, 0, 46));
    // Panel body
    round(surface, 0.755, 0.085, 0.236, 0.560, 0.012, window);

    // ── Assistant title bar ──────────────────────────────────────────────────
    round(surface, 0.755, 0.085, 0.236, 0.055, 0.012, panel_hdr);
    rect(surface, 0.755, 0.130, 0.236, 0.010, panel_hdr);

    // Sparkle / assistant icon
    draw_sparkle(surface, x(0.769), y(0.096), w(0.014), h(0.023), teal);
    label(surface, 0.788, 0.093, 0.020, "Assistant", ink);

    // Close × button (right side, inside title bar)
    draw_close_x(surface, x(0.967), y(0.098), w(0.014), h(0.020), muted);

    // ── "Traitement local" badge ─────────────────────────────────────────────
    round(surface, 0.766, 0.153, 0.202, 0.031, 0.015, icon_bg);
    draw_shield(surface, x(0.773), y(0.157), w(0.010), h(0.018), teal);
    label(surface, 0.788, 0.158, 0.014, "Traitement local, rien n'est envoy\u{e9}", teal);

    // ── User message bubble (teal) ────────────────────────────────────────────
    round(surface, 0.797, 0.197, 0.172, 0.047, 0.012, teal);
    label(surface, 0.806, 0.204, 0.016,
          "R\u{e9}sume le fichier Notes-de-cours.", white);

    // ── Assistant reply card ──────────────────────────────────────────────────
    round(surface, 0.766, 0.256, 0.204, 0.098, 0.012, Color::rgb(240, 238, 233));
    label(surface, 0.779, 0.267, 0.016,
          "Trois chapitres, deux exercices \u{e0} rendre", ink);
    label(surface, 0.779, 0.287, 0.016,
          "jeudi et un point \u{e0} revoir sur les index.", ink);
    label(surface, 0.779, 0.307, 0.016,
          "Je peux pr\u{e9}parer une fiche de r\u{e9}vision.", ink);

    // ── Action buttons — side by side, single-line labels ────────────────────
    // Maquette: two equal-width buttons, text on one line each
    for (i, btn_label) in ["Ouvrir le fichier", "Cr\u{e9}er la fiche"].iter().enumerate() {
        let bx = 0.766 + i as f32 * 0.106;
        round(surface, bx, 0.368, 0.098, 0.040, 0.020, Color::rgb(255, 255, 253));
        label(surface, bx + 0.007, 0.377, 0.015, btn_label, ink);
    }

    // ── Separator + input field ───────────────────────────────────────────────
    rect(surface, 0.755, 0.425, 0.236, 0.0015, Color::rgb(226, 225, 220));
    round(surface, 0.766, 0.438, 0.192, 0.047, 0.009, Color::rgb(255, 255, 253));
    label(surface, 0.776, 0.446, 0.015, "Demander \u{e0} l'assistant", muted);
    // Send button (teal rounded square, right of input)
    round(surface, 0.960, 0.440, 0.022, 0.038, 0.007, teal);
    draw_arrow_right(surface, x(0.960), y(0.440), w(0.022), h(0.038), white);
    // ════════════════════════════════════════════════════════════════════════
    // 5. LAUNCHER OVERLAY — toutes coordonnées mesurées pixel par pixel
    //    depuis la référence 2880×1800 @2x (logique 1440×900).
    //    Fraction = coord_px_@2x / dimension_@2x.
    //
    //    Bord lanceur:   left=32   → 32/2880=0.0111
    //                    top=586   → 586/1800=0.3256
    //                    right=1115 → 1115/2880=0.3872 → w=0.3761
    //                    bot=1644  → 1644/1800=0.9133 → h=0.5877
    //    Barre recherche:top=620   → 0.3444  h=96px→0.0533
    //    "Épinglés":     y≈794     → 0.4411
    //    Icône app haut: y=860     → 0.4778  h=87px→0.0483
    //    Icône app w:    90px→0.0312  stride_x=170px→0.0590
    //    Stride vertical icônes: 178px→0.0989
    //    "Récents" titre:y≈1200   → 0.6667
    //    Items récents:  y≈1270   → 0.7056  stride=130px→0.0722
    //    Séparateur:     y=1580   → 0.8778
    //    Avatar top:     y=1566   → 0.8700
    //    Gap lanceur→taskbar: 11px@2x → 0.0061
    // ════════════════════════════════════════════════════════════════════════

    // Dimensions du panneau
    let lnch_x = 0.0111_f32;
    let lnch_y = 0.3256_f32;
    let lnch_w = 0.3761_f32;
    let lnch_h = 0.5878_f32;
    let pad    = 0.0188_f32;  // marge interne: 54px@2x/2880=0.0188

    // Ombre portée
    round(surface, lnch_x + 0.002, lnch_y + 0.003, lnch_w, lnch_h, 0.014,
          Color::rgba(0, 0, 0, 52));
    // Corps du panneau
    round(surface, lnch_x, lnch_y, lnch_w, lnch_h, 0.014, window);

    // ── Barre de recherche ──────────────────────────────────────────────────
    // top=0.3444, hauteur=0.0533, même marge pad des deux côtés
    let srch_y = 0.3444_f32;
    let srch_h = 0.0533_f32;
    let srch_x = lnch_x + pad;
    let srch_w = lnch_w - 2.0 * pad;
    round(surface, srch_x, srch_y, srch_w, srch_h, 0.013,
          Color::rgb(240, 238, 233));
    // Loupe : centre vertical de la barre, à pad du bord gauche
    let lupe_sz = srch_h * 0.55;
    draw_magnifier(surface,
                   x(srch_x + 0.012), y(srch_y + (srch_h - lupe_sz) / 2.0),
                   w(0.012), h(lupe_sz), muted);
    label(surface, srch_x + 0.030, srch_y + 0.012, 0.018,
          "Rechercher apps, fichiers, param\u{e8}tres, actions", muted);

    // ── En-tête de section "Épinglés" / "Toutes les apps" ───────────────────
    // y≈794/1800=0.4411 dans la référence
    let hdr_y = 0.4411_f32;
    label(surface, lnch_x + pad, hdr_y, 0.019, "\u{c9}pingl\u{e9}s", ink);
    label(surface, lnch_x + lnch_w - 0.100, hdr_y, 0.017, "Toutes les apps", teal);

    // ── Grille d'apps 6 × 2 ────────────────────────────────────────────────
    // Icône L1 top=0.4778, hauteur=0.0483, largeur=0.0312
    // stride_x=0.0590 (icône + gap), stride_y=0.0989
    let icon_x0 = 0.0382_f32;   // x bord gauche colonne 0 : 110/2880=0.0382
    let icon_y0 = 0.4778_f32;   // y haut ligne 0
    let icon_w  = 0.0312_f32;
    let icon_h  = 0.0483_f32;
    let icon_sx = 0.0590_f32;   // stride x
    let icon_sy = 0.0989_f32;   // stride y

    let app_names = [
        "Fichiers", "Web",    "Courrier", "Agenda",   "Terminal",   "Texte",
        "Tableur",  "Diapos", "Photos",   "Musique",  "Boutique",   "Param\u{e8}tres",
    ];
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

    for index in 0..12_usize {
        let col = (index % 6) as f32;
        let row = (index / 6) as f32;
        let bx = icon_x0 + col * icon_sx;
        let by = icon_y0 + row * icon_sy;

        round(surface, bx, by, icon_w, icon_h, 0.008, app_colors[index]);
        draw_app_icon(surface, x(bx), y(by), w(icon_w), h(icon_h),
                      index, app_colors[index]);

        // Label centré sous l'icône — taille 0.013 pour rester dans le stride
        let lbl_x = bx + icon_w / 2.0 - 0.013;
        label(surface, lbl_x, by + icon_h + 0.007, 0.013, app_names[index], ink);
    }

    // ── Section Récents ─────────────────────────────────────────────────────
    // "Récents" titre : après icone L2 (bot≈0.6250), espace, y≈0.6667
    // Items : y0≈0.7056, stride≈0.0722
    let rec_title_y = 0.6667_f32;
    let rec_y0      = 0.7233_f32;
    let rec_dy      = 0.0578_f32;  // stride vertical items récents
    let col_w       = (lnch_w - 2.0 * pad) / 2.0;
    let ri_w        = 0.021_f32;
    let ri_h        = 0.0356_f32;

    label(surface, lnch_x + pad, rec_title_y, 0.019, "R\u{e9}cents", ink);

    let recent_items = [
        ("Notes-de-cours",              "Il y a 12 min", 1_usize),
        ("Budget-2026",                 "Il y a 1 h",    2),
        ("Pr\u{e9}sentation-soutenance","Hier",          3),
        ("Photos-\u{e9}quipe",          "Hier",          4),
    ];

    for (index, (name, time, icon_type)) in recent_items.iter().enumerate() {
        let col = (index % 2) as f32;
        let row = (index / 2) as f32;
        let bx = lnch_x + pad + col * col_w;
        let by = rec_y0 + row * rec_dy;

        // Icône sans fond
        draw_file_icon(surface, x(bx), y(by), w(ri_w), h(ri_h), *icon_type,
                       file_icon_colors[*icon_type]);
        label(surface, bx + ri_w + 0.009, by + 0.004, 0.015, name, ink);
        label(surface, bx + ri_w + 0.009, by + 0.021, 0.013, time, muted);
    }

    // ── Pied de page ────────────────────────────────────────────────────────
    // Séparateur : y=1580/1800=0.8778
    // Avatar     : y=1566/1800=0.8700, h≈44px@2x=0.0244
    let sep_y  = 0.8439_f32;
    let foot_y = 0.8667_f32;
    let av_h   = 0.0311_f32;  // hauteur avatar ≈ 72px@2x/1800

    rect(surface, lnch_x + pad, sep_y, lnch_w - 2.0 * pad, 0.002,
         Color::rgb(226, 225, 220));

    let av_x = lnch_x + pad;
    let av_w = 0.025_f32;
    ellipse(surface, av_x, foot_y, av_w, av_h, teal);
    draw_person(surface, x(av_x), y(foot_y), w(av_w), h(av_h), white);
    label(surface, av_x + av_w + 0.012, foot_y + 0.010, 0.018, "Mon compte", ink);

    // Bouton power (bord droit, aligné verticalement avec avatar)
    let pw_x = lnch_x + lnch_w - pad - av_w;
    ellipse(surface, pw_x, foot_y + 0.001, av_w, av_h - 0.002,
            Color::rgb(220, 218, 213));
    draw_power(surface, x(pw_x), y(foot_y + 0.001), w(av_w), h(av_h - 0.002), ink);


    let tb_y = 0.9194_f32;
    let tb_h = 0.0806_f32;
    rect(surface, 0.0, tb_y, 1.0, tb_h, Color::rgb(9, 36, 34));

    // ── Launcher button: teal rounded square with 4-square grid icon ─────────
    let lb_x = 0.011_f32;
    let lb_y = tb_y + 0.009;
    let lb_w = 0.033_f32;
    let lb_h = 0.047_f32;
    round(surface, lb_x, lb_y, lb_w, lb_h, 0.009, teal);
    // 4 small white rounded squares inside the launcher button
    for (gr, gc) in [(0_i32, 0_i32), (0, 1), (1, 0), (1, 1)] {
        let gx = lb_x + 0.007 + gc as f32 * 0.011;
        let gy = lb_y + 0.009 + gr as f32 * 0.014;
        round(surface, gx, gy, 0.007, 0.010, 0.0015, white);
    }

    // ── Search capsule ────────────────────────────────────────────────────────
    round(surface, 0.050, lb_y, 0.158, lb_h, 0.023, Color::rgb(47, 70, 67));
    draw_magnifier(surface, x(0.063), y(tb_y + 0.025), w(0.011), h(0.018),
                   Color::rgb(200, 220, 215));
    label(surface, 0.080, tb_y + 0.021, 0.018, "Rechercher", white);

    // ── Dock (centre) — 6 app icons ──────────────────────────────────────────
    // Maquette dock icons (left to right): Files, Web/Globe, Mail, Sparkle/Star,
    // Calendar, Sliders. Active: index 0 (Files) and index 3 (Sparkle).
    let dock_colors = [
        teal,                       // Files
        Color::rgb(40, 105, 207),   // Web
        Color::rgb(184, 61, 13),    // Mail
        Color::rgb(0, 126, 103),    // Sparkle (teal)
        Color::rgb(112, 52, 214),   // Calendar
        Color::rgb(68, 81, 101),    // Sliders
    ];
    let dock_active = [true, false, false, true, false, false];
    let dock_start_x = 0.390_f32;
    let icon_stride  = 0.047_f32;
    let icon_w_f = 0.028_f32;
    let icon_h_f = 0.030_f32;

    for (i, (color, active)) in dock_colors.iter().zip(dock_active.iter()).enumerate() {
        let bx = dock_start_x + i as f32 * icon_stride;
        let by = lb_y + (lb_h - icon_h_f) / 2.0;

        // Active highlight tile (behind icon)
        if *active {
            round(surface, bx - 0.003, lb_y, icon_w_f + 0.006, lb_h, 0.009,
                  Color::rgb(39, 62, 59));
        }

        // Icon rounded square
        round(surface, bx, by, icon_w_f, icon_h_f, 0.005, *color);

        // Inner glyph
        draw_dock_glyph(surface, x(bx), y(by), w(icon_w_f), h(icon_h_f), i);

        // Active indicator dot (teal, below icon, centred)
        if *active {
            let dot_x = bx + icon_w_f / 2.0 - 0.007;
            round(surface, dot_x, tb_y + tb_h - 0.010, 0.014, 0.004, 0.002, teal_light);
        }
    }

    // ── System tray: wifi, volume/battery, divider ────────────────────────────
    for (i, _) in (0..3_usize).enumerate() {
        let ix = 0.868 + i as f32 * 0.018;
        ellipse(surface, ix, tb_y + 0.023, 0.010, 0.014,
                Color::rgb(239, 247, 244));
    }
    // Battery/status icon placeholder (small rounded rect)
    round(surface, 0.918, tb_y + 0.025, 0.020, 0.012, 0.003,
          Color::rgb(239, 247, 244));

    // ── Clock ─────────────────────────────────────────────────────────────────
    label(surface, 0.943, tb_y + 0.008, 0.019, "15:30",         white);
    label(surface, 0.940, tb_y + 0.030, 0.015, "21 nov. 2026",  white);
}

// ══════════════════════════════════════════════════════════════════════════════
// Icon drawing helpers — all coordinates are absolute pixels
// ══════════════════════════════════════════════════════════════════════════════

/// Folder icon: tab on top-left + rectangular body.
fn draw_folder_icon(
    surface: &mut dyn RenderTarget,
    x: i32, y: i32, w: u32, h: u32,
    color: Color,
) {
    let w = w as i32;
    let h = h as i32;
    // Body
    surface.fill_round_rect(x, y + h / 4, w as u32, (h * 3 / 4) as u32, 2, color);
    // Tab
    surface.fill_round_rect(x, y + h / 4 - 2, (w / 2) as u32, (h / 5) as u32, 2, color);
}

/// Trash icon: cylindrical bin with lid.
fn draw_trash_icon(
    surface: &mut dyn RenderTarget,
    x: i32, y: i32, w: u32, h: u32,
    color: Color,
) {
    let w = w as i32;
    let h = h as i32;
    // Bin body (slightly narrower than the full tile)
    let bx = x + w / 6;
    let by = y + h / 5;
    let bw = (w * 2 / 3) as u32;
    let bh = (h * 3 / 4) as u32;
    surface.fill_round_rect(bx, by, bw, bh, 2, color);
    // Lid (wider, thinner)
    surface.fill_rect(x, y + h / 6, w as u32, (h / 10).max(2) as u32, color);
    // Handle
    surface.fill_round_rect(x + w / 3, y, (w / 3) as u32, (h / 7) as u32, 1, color);
}

/// Left-pointing chevron < centré dans la boîte.
fn draw_chevron_left(
    surface: &mut dyn RenderTarget,
    x: i32, y: i32, w: u32, h: u32,
    color: Color,
) {
    let w = w as i32;
    let h = h as i32;
    let cx = x + w / 2;
    let cy = y + h / 2;
    // Bras à 35 % de la hauteur de la boîte, pointe décalée de 25 %
    let arm = (h * 35 / 100).max(3);
    let off = (w * 25 / 100).max(2);
    surface.draw_line((cx + off, cy - arm), (cx - off, cy), 1, color);
    surface.draw_line((cx - off, cy), (cx + off, cy + arm), 1, color);
}

/// Right-pointing chevron > centred in the given box.
fn draw_chevron_right(
    surface: &mut dyn RenderTarget,
    x: i32, y: i32, w: u32, h: u32,
    color: Color,
) {
    let w = w as i32;
    let h = h as i32;
    let cx = x + w / 2;
    let cy = y + h / 2;
    let arm = (h * 35 / 100).max(3);
    let off = (w * 25 / 100).max(2);
    surface.draw_line((cx - off, cy - arm), (cx + off, cy), 1, color);
    surface.draw_line((cx + off, cy), (cx - off, cy + arm), 1, color);
}

/// Sparkle / assistant star icon (4-pointed star-ish shape).
fn draw_sparkle(
    surface: &mut dyn RenderTarget,
    x: i32, y: i32, w: u32, h: u32,
    color: Color,
) {
    let w = w as i32;
    let h = h as i32;
    let cx = x + w / 2;
    let cy = y + h / 2;
    let r  = (w.min(h) / 2).max(2);
    // Outer diamond strokes
    surface.draw_line((cx, cy - r), (cx, cy + r), 1, color);
    surface.draw_line((cx - r, cy), (cx + r, cy), 1, color);
    // Diagonal shorter lines for the sparkle feel
    let d = r * 6 / 10;
    surface.draw_line((cx - d, cy - d), (cx + d, cy + d), 1, color);
    surface.draw_line((cx + d, cy - d), (cx - d, cy + d), 1, color);
    // Centre dot
    surface.fill_ellipse(cx - 1, cy - 1, 2, 2, color);
}

/// Close × button.
fn draw_close_x(
    surface: &mut dyn RenderTarget,
    x: i32, y: i32, w: u32, h: u32,
    color: Color,
) {
    let w = w as i32;
    let h = h as i32;
    let m = 2;
    surface.draw_line((x + m, y + m), (x + w - m, y + h - m), 1, color);
    surface.draw_line((x + w - m, y + m), (x + m, y + h - m), 1, color);
}

/// Magnifying glass icon.
fn draw_magnifier(
    surface: &mut dyn RenderTarget,
    x: i32, y: i32, w: u32, h: u32,
    color: Color,
) {
    let w = w as i32;
    let h = h as i32;
    let d = (w.min(h) * 2 / 3).max(4);
    surface.fill_ellipse(x, y, d as u32, d as u32, color);
    // Erase centre (approximate by darker fill — just leave filled for simplicity)
    let inset = d / 4;
    surface.fill_ellipse(
        x + inset, y + inset,
        (d - 2 * inset) as u32, (d - 2 * inset) as u32,
        Color::rgba(255, 255, 255, 180),
    );
    // Handle
    surface.draw_line((x + d, y + d), (x + w, y + h), 2, color);
}

/// Shield icon (security / privacy badge).
fn draw_shield(
    surface: &mut dyn RenderTarget,
    x: i32, y: i32, w: u32, h: u32,
    color: Color,
) {
    let w = w as i32;
    let h = h as i32;
    let top_r = w / 4;
    surface.fill_round_rect(x, y, w as u32, (h * 3 / 4) as u32, top_r as u32, color);
    // Pointed bottom
    surface.fill_ellipse(x + w / 4, y + h / 2, (w / 2) as u32, (h / 2) as u32, color);
}

/// Arrow → centred in a box (for the assistant send button).
fn draw_arrow_right(
    surface: &mut dyn RenderTarget,
    x: i32, y: i32, w: u32, h: u32,
    color: Color,
) {
    let w = w as i32;
    let h = h as i32;
    let cx = x + w / 2;
    let cy = y + h / 2;
    let hs = h / 5;
    surface.draw_line((cx - hs, cy), (cx + hs, cy), 1, color);
    surface.draw_line((cx, cy - hs), (cx + hs, cy), 1, color);
    surface.draw_line((cx, cy + hs), (cx + hs, cy), 1, color);
}

/// Power/on-off icon (circle outline + vertical line at top).
fn draw_power(
    surface: &mut dyn RenderTarget,
    x: i32, y: i32, w: u32, h: u32,
    color: Color,
) {
    let w = w as i32;
    let h = h as i32;
    let cx = x + w / 2;
    let cy = y + h / 2;
    let r = (w.min(h) / 3).max(2);
    // Outer ring
    surface.fill_ellipse(cx - r, cy - r, (r * 2) as u32, (r * 2) as u32, color);
    // Hollow centre (erase with the button background color)
    surface.fill_ellipse(cx - r + 1, cy - r + 1,
        (r * 2 - 2) as u32, (r * 2 - 2) as u32,
        Color::rgb(220, 218, 213));
    // Top vertical bar
    surface.draw_line((cx, cy - r - 2), (cx, cy - 1), 2, color);
}

/// Person silhouette: head (circle) + shoulder arc, for the account avatar.
fn draw_person(
    surface: &mut dyn RenderTarget,
    x: i32, y: i32, w: u32, h: u32,
    color: Color,
) {
    let w = w as i32;
    let h = h as i32;
    let cx = x + w / 2;
    let head_r = (w / 5).max(2);
    let head_cy = y + h / 3;
    surface.fill_ellipse(cx - head_r, head_cy - head_r,
                         (head_r * 2) as u32, (head_r * 2) as u32, color);
    let body_w = (w * 2 / 3) as u32;
    let body_h = (h / 3) as u32;
    surface.fill_ellipse(cx - w / 3, y + h * 3 / 5, body_w, body_h, color);
}

/// Small sidebar icon glyph (simplified).
/// kind: 0=home, 1=doc, 2=download, 3=image, 4=music
fn draw_sidebar_icon(
    surface: &mut dyn RenderTarget,
    x: i32, y: i32, w: u32, h: u32,
    kind: usize,
    color: Color,
) {
    let w = w as i32;
    let h = h as i32;
    let cx = x + w / 2;
    let cy = y + h / 2;
    match kind {
        0 => {
            // Home — simple house shape (triangle roof + rect body)
            surface.draw_line((x, cy), (cx, y), 1, color);
            surface.draw_line((cx, y), (x + w, cy), 1, color);
            surface.fill_rect(x + 1, cy, (w - 2) as u32, (h - cy + y) as u32, color);
        }
        1 => {
            // Document — folded corner rect
            surface.fill_round_rect(x, y, (w - 2) as u32, h as u32, 1, color);
        }
        2 => {
            // Download — arrow pointing down
            surface.draw_line((cx, y), (cx, y + h * 3 / 4), 1, color);
            surface.draw_line((x + 1, cy), (cx, y + h * 3 / 4), 1, color);
            surface.draw_line((x + w - 1, cy), (cx, y + h * 3 / 4), 1, color);
            surface.draw_line((x, y + h), (x + w, y + h), 1, color);
        }
        3 => {
            // Image — rectangle with mountain
            surface.draw_line((x, y), (x + w, y), 1, color);
            surface.draw_line((x + w, y), (x + w, y + h), 1, color);
            surface.draw_line((x + w, y + h), (x, y + h), 1, color);
            surface.draw_line((x, y + h), (x, y), 1, color);
            surface.draw_line((x, cy + 2), (cx, cy - 2), 1, color);
            surface.draw_line((cx, cy - 2), (x + w, cy + 4), 1, color);
        }
        _ => {
            // Music note
            surface.draw_line((cx, y), (cx, y + h * 3 / 4), 1, color);
            surface.fill_ellipse(x + 1, y + h * 3 / 4 - 2, (w / 2) as u32, (h / 4) as u32, color);
        }
    }
}

/// File row icon glyph.
/// kind: 0=folder, 1=doc-text, 2=spreadsheet, 3=presentation, 4=photo, 5=doc-text
fn draw_file_icon(
    surface: &mut dyn RenderTarget,
    x: i32, y: i32, w: u32, h: u32,
    kind: usize,
    accent: Color,
) {
    let w = w as i32;
    let h = h as i32;
    let pad_x = w / 5;
    let pad_y = h / 6;
    let ix = x + pad_x;
    let iy = y + pad_y;
    let iw = (w - 2 * pad_x).max(2) as u32;
    let ih = (h - 2 * pad_y).max(2) as u32;
    match kind {
        0 => {
            // Folder
            surface.fill_round_rect(ix, iy + ih as i32 / 4, iw, ih * 3 / 4, 1, accent);
            surface.fill_round_rect(ix, iy + ih as i32 / 4 - 2, iw / 2, ih / 4, 1, accent);
        }
        2 => {
            // Spreadsheet: grid lines inside a rect
            surface.fill_round_rect(ix, iy, iw, ih, 1, accent);
            let white = Color::rgb(255, 255, 255);
            surface.draw_line((ix, iy + ih as i32 / 2), (ix + iw as i32, iy + ih as i32 / 2), 1, white);
            surface.draw_line((ix + iw as i32 / 2, iy), (ix + iw as i32 / 2, iy + ih as i32), 1, white);
        }
        3 | 7 => {
            // Presentation: screen with stand
            surface.fill_round_rect(ix, iy, iw, ih * 3 / 4, 1, accent);
            surface.draw_line(
                (ix + iw as i32 / 2, iy + ih as i32 * 3 / 4),
                (ix + iw as i32 / 2, iy + ih as i32),
                1, accent,
            );
        }
        4 => {
            // Photo: rect + mountains silhouette
            surface.fill_round_rect(ix, iy, iw, ih, 1, accent);
            let white = Color::rgb(255, 255, 255);
            surface.fill_ellipse(ix + 1, iy + 1, iw / 4, ih / 4, white);
        }
        _ => {
            // Generic document with folded corner
            surface.fill_round_rect(ix, iy, iw, ih, 1, accent);
            let white = Color::rgb(255, 255, 255);
            let lw = (iw as i32 - 4).max(2);
            surface.draw_line((ix + 2, iy + ih as i32 / 2), (ix + 2 + lw, iy + ih as i32 / 2), 1, white);
            surface.draw_line((ix + 2, iy + ih as i32 * 2 / 3), (ix + 2 + lw, iy + ih as i32 * 2 / 3), 1, white);
        }
    }
}

/// Dock glyph: very small pictogram inside a dock icon tile.
fn draw_dock_glyph(
    surface: &mut dyn RenderTarget,
    x: i32, y: i32, w: u32, h: u32,
    index: usize,
) {
    let white = Color::rgb(255, 255, 255);
    let w = w as i32;
    let h = h as i32;
    let cx = x + w / 2;
    let cy = y + h / 2;
    let s = (w.min(h) / 4).max(1);
    match index {
        0 => {
            // Files — folder outline
            surface.draw_line((x + 1, cy - s), (cx, cy - s), 1, white);
            surface.draw_line((cx, cy - s), (cx + s / 2, cy), 1, white);
            surface.draw_line((cx + s / 2, cy), (x + w - 1, cy), 1, white);
            surface.draw_line((x + w - 1, cy), (x + w - 1, y + h - 1), 1, white);
            surface.draw_line((x + w - 1, y + h - 1), (x + 1, y + h - 1), 1, white);
            surface.draw_line((x + 1, y + h - 1), (x + 1, cy - s), 1, white);
        }
        1 => {
            // Web — globe circle with cross
            surface.fill_ellipse(cx - s, cy - s, (s * 2) as u32, (s * 2) as u32, white);
            surface.fill_ellipse(cx - s + 1, cy - s + 1, (s * 2 - 2) as u32, (s * 2 - 2) as u32,
                Color::rgba(0, 0, 0, 0)); // transparent — just outline effect
        }
        2 => {
            // Mail — envelope
            surface.draw_line((x + 1, cy - s), (x + w - 1, cy - s), 1, white);
            surface.draw_line((x + w - 1, cy - s), (x + w - 1, cy + s), 1, white);
            surface.draw_line((x + w - 1, cy + s), (x + 1, cy + s), 1, white);
            surface.draw_line((x + 1, cy + s), (x + 1, cy - s), 1, white);
            surface.draw_line((x + 1, cy - s + 1), (cx, cy), 1, white);
            surface.draw_line((cx, cy), (x + w - 1, cy - s + 1), 1, white);
        }
        4 => {
            // Sparkle (active / starred) — simple star
            surface.draw_line((cx, cy - s - 1), (cx, cy + s + 1), 1, white);
            surface.draw_line((cx - s - 1, cy), (cx + s + 1, cy), 1, white);
        }
        _ => {
            // Generic: small centred dot
            surface.fill_ellipse(cx - 1, cy - 1, 3, 3, white);
        }
    }
}

/// App grid icon — same as before but slightly adjusted proportions.
fn draw_app_icon(
    surface: &mut dyn RenderTarget,
    x: i32, y: i32, w: u32, h: u32,
    index: usize,
    tile: Color,
) {
    let w = w as i32;
    let h = h as i32;
    let left   = x + w / 4;
    let right  = x + w * 3 / 4;
    let top    = y + h / 4;
    let bottom = y + h * 3 / 4;
    let cx = x + w / 2;
    let cy = y + h / 2;
    let white  = Color::rgb(255, 255, 255);
    let stroke = (w / 13).max(1) as u32;
    let line   = |surface: &mut dyn RenderTarget, a, b| surface.draw_line(a, b, stroke, white);

    match index {
        0 => {
            // Folder
            line(surface, (left, top + 2), (cx - 1, top + 2));
            line(surface, (cx - 1, top + 2), (cx + 2, top + 5));
            line(surface, (cx + 2, top + 5), (right, top + 5));
            line(surface, (right, top + 5), (right, bottom));
            line(surface, (right, bottom), (left, bottom));
            line(surface, (left, bottom), (left, top + 2));
        }
        1 => {
            // Globe
            let d = (w / 2).max(3) as u32;
            let gx = cx - d as i32 / 2;
            let gy = cy - d as i32 / 2;
            surface.fill_ellipse(gx, gy, d, d, white);
            let inset = (d / 5).max(1);
            surface.fill_ellipse(gx + inset as i32, gy + inset as i32,
                                  d - 2 * inset, d - 2 * inset, tile);
            line(surface, (cx, gy), (cx, gy + d as i32));
            line(surface, (gx, cy), (gx + d as i32, cy));
        }
        2 => {
            // Envelope
            line(surface, (left, top + 2), (right, top + 2));
            line(surface, (right, top + 2), (right, bottom - 1));
            line(surface, (right, bottom - 1), (left, bottom - 1));
            line(surface, (left, bottom - 1), (left, top + 2));
            line(surface, (left, top + 3), (cx, cy + 1));
            line(surface, (cx, cy + 1), (right, top + 3));
        }
        3 => {
            // Calendar
            line(surface, (left, top + 3), (right, top + 3));
            line(surface, (right, top + 3), (right, bottom));
            line(surface, (right, bottom), (left, bottom));
            line(surface, (left, bottom), (left, top + 3));
            line(surface, (left, top + 1), (left, top + 5));
            line(surface, (right, top + 1), (right, top + 5));
            line(surface, (left, cy), (right, cy));
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
            line(surface, (left + 5, cy), (right - 3, cy));
            line(surface, (left + 5, cy + 4), (right - 3, cy + 4));
        }
        6 => {
            // Spreadsheet
            line(surface, (left, top), (right, top));
            line(surface, (right, top), (right, bottom));
            line(surface, (right, bottom), (left, bottom));
            line(surface, (left, bottom), (left, top));
            line(surface, (left, cy), (right, cy));
            line(surface, (cx, top), (cx, bottom));
        }
        7 => {
            // Presentation
            line(surface, (left, top), (right, top));
            line(surface, (right, top), (right, bottom - 3));
            line(surface, (right, bottom - 3), (left, bottom - 3));
            line(surface, (left, bottom - 3), (left, top));
            line(surface, (cx, bottom - 3), (cx, bottom + 2));
            line(surface, (cx - 3, bottom + 2), (cx + 3, bottom + 2));
        }
        8 => {
            // Photo
            line(surface, (left, top), (right, top));
            line(surface, (right, top), (right, bottom));
            line(surface, (right, bottom), (left, bottom));
            line(surface, (left, bottom), (left, top));
            surface.fill_ellipse(left + 3, top + 3, stroke + 1, stroke + 1, white);
            line(surface, (left + 2, bottom - 3), (cx, cy));
            line(surface, (cx, cy), (right - 2, bottom - 3));
        }
        9 => {
            // Music note
            line(surface, (cx + 3, top), (cx + 3, bottom - 3));
            line(surface, (cx + 3, top), (right, top + 2));
            line(surface, (right, top + 2), (right, bottom - 4));
            surface.fill_ellipse(left + 1, bottom - 5,
                w.max(3) as u32 / 5, h.max(3) as u32 / 6, white);
            surface.fill_ellipse(right - 3, bottom - 6,
                w.max(3) as u32 / 5, h.max(3) as u32 / 6, white);
        }
        10 => {
            // Boutique / shop bag
            line(surface, (left, top + 5), (right, top + 5));
            line(surface, (left + 2, top + 5), (left + 2, bottom));
            line(surface, (right - 2, top + 5), (right - 2, bottom));
            line(surface, (left + 2, bottom), (right - 2, bottom));
            line(surface, (cx, cy + 2), (cx, bottom));
            line(surface, (left - 1, top + 4), (left + 2, top));
            line(surface, (left + 2, top), (right - 2, top));
            line(surface, (right - 2, top), (right + 1, top + 4));
        }
        _ => {
            // Settings / gear (simplified concentric circles)
            let d = (w / 2).max(3) as u32;
            let gx = cx - d as i32 / 2;
            let gy = cy - d as i32 / 2;
            surface.fill_ellipse(gx, gy, d, d, white);
            let inset = (d / 3).max(1);
            surface.fill_ellipse(gx + inset as i32, gy + inset as i32,
                                  d - 2 * inset, d - 2 * inset, tile);
        }
    }
}
