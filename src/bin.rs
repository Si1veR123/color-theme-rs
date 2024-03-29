use std::{env, error::Error, fmt::Display};
use image::Rgb;
use color_theme::{get_theme_color, median_cut_palette};


macro_rules! usage_err {
    ($err: expr) => {
        {
            println!("\nUsage: color-theme <FILENAME> <PALETTE COLOR COUNT (1-255, optional)>  <BRIGHTNESS (0-255, optional)>");
            $err
        }
    };
}

#[derive(Debug)]
enum UsageError {
    NoFilename,
    FileNotFound,
    InvalidPaletteCount,
    InvalidBrightness
}

impl Display for UsageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for UsageError {}

pub fn format_rgb(pixel: Rgb<u8>) -> String {
    format!("rgb({}, {}, {})", pixel.0[0], pixel.0[1], pixel.0[2])
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut cli_args = env::args().skip(1);

    let filename = cli_args.next()
        .ok_or_else(|| usage_err!(UsageError::NoFilename) )?;
    let image = image::open(filename)
        .map_err(|_| usage_err!(UsageError::FileNotFound))?;

    let mut rgb_image = image.into_rgb8();

    let palette_n = match cli_args.next() {
        Some(s) => s.parse::<u8>().map_err(|_| usage_err!(UsageError::InvalidPaletteCount))?,
        None => 16
    };

    let target_brightness = match cli_args.next() {
        Some(s) => s.parse::<u8>().map_err(|_| usage_err!(UsageError::InvalidBrightness))?,
        None => 200
    };

    let colors = median_cut_palette(&mut rgb_image, palette_n);

    // Only None if `palette_n` is 0
    let theme = get_theme_color(&colors, Some(target_brightness))
        .ok_or_else(|| usage_err!(UsageError::InvalidPaletteCount))?;

    let palette_formatted: Vec<String> = colors.iter().copied().map(format_rgb).collect();
    let palette_string = palette_formatted.join(", ");

    println!("Palette: {palette_string}");
    println!("Theme color: {}", format_rgb(theme));

    Ok(())
}
