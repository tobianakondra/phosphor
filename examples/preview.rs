use std::env;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::PathBuf;

use phosphor::{Color, Surface, scene::render_desktop};

fn main() -> io::Result<()> {
    let output = env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("phosphor-preview.ppm"));

    let mut surface = Surface::new(1280, 720, Color::rgb(0, 0, 0))
        .ok_or_else(|| io::Error::other("could not allocate preview surface"))?;
    render_desktop(&mut surface);

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
