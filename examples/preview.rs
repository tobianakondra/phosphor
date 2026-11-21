use std::env;
use std::fs::{self, File};
use std::io::{self, BufWriter, Write};
use std::path::{Path, PathBuf};

use phosphor::{Color, Surface, scene::render_desktop, text::Typeface};

fn main() -> io::Result<()> {
    let output = env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("phosphor-preview.ppm"));
    let font_path = env::args_os().nth(2).map(PathBuf::from);
    let typeface = load_typeface(font_path.as_deref())?;

    let mut surface = Surface::new(1280, 720, Color::rgb(0, 0, 0))
        .ok_or_else(|| io::Error::other("could not allocate preview surface"))?;
    render_desktop(&mut surface, &typeface);

    let file = File::create(&output)?;
    let mut writer = BufWriter::new(file);
    writeln!(writer, "P6\n{} {}\n255", surface.width(), surface.height())?;
    for pixel in surface.pixels() {
        writer.write_all(&[pixel.red, pixel.green, pixel.blue])?;
    }
    writer.flush()?;

    println!("Preview written to {}", output.display());
    Ok(())
}

fn load_typeface(path: Option<&Path>) -> io::Result<Typeface> {
    const DEFAULT_FONTS: &[&str] = &[
        "/usr/share/fonts/TTF/DejaVuSans.ttf",
        "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
        "/usr/share/fonts/Sans/DejaVu/DejaVuSans.ttf",
    ];

    let candidates: Vec<&Path> = match path {
        Some(path) => vec![path],
        None => DEFAULT_FONTS.iter().map(Path::new).collect(),
    };

    for candidate in candidates {
        if let Ok(bytes) = fs::read(candidate) {
            return Typeface::from_bytes(&bytes).map_err(|_| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("could not parse font {}", candidate.display()),
                )
            });
        }
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "DejaVu Sans not found; pass its .ttf path as the second argument",
    ))
}
