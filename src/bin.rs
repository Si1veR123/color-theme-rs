use image::{DynamicImage, ImageError};

mod lib;
use lib::{get_theme_colour, median_cut_palette, format_rgb};

fn out_usage_and_quit() -> ! {
    println!("\nUsage: colour_theme.exe [ARGS]\n\t<filename/path>\n\t<# of palette colours> (power of 2, optional)\n\t<theme brightness> (0-255, optional)\n");
    panic!()
}

fn get_image_from_cli() -> Result<DynamicImage, ImageError> {
    let filename = std::env::args().nth(1).unwrap_or_else(|| out_usage_and_quit());
    Ok(image::open(filename)?)
}


fn main() {
    let image = get_image_from_cli().expect("Can't find file");
    let rgb_image = image.as_rgb8().expect("Invalid format");

    let palette_n: u8 = match std::env::args().nth(2) {
        Some(s) => s.parse().unwrap(),
        None => 16
    };

    let target_brightness: u8 = match std::env::args().nth(3) {
        Some(s) => s.parse().unwrap(),
        None => 200
    };

    let colours = median_cut_palette(rgb_image, palette_n);

    let theme = get_theme_colour(&colours, Some(target_brightness));
    let palette_formatted: Vec<String> = colours.iter().map(
        |colour| {
            format_rgb(colour)
        }
    ).collect();

    print!("Palette:");
    for c in palette_formatted {
        print!(" {} ", c);
    }
    println!("");

    println!("Theme colour: {}", format_rgb(&theme));
}
