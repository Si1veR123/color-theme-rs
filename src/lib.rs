use image::{RgbImage, Rgb};

/// Sort the pixels by the colour channel with the most range
fn median_cut_sort_bucket(pixel_bucket: &mut [Rgb<u8>]) {
    let mut r_minmax = (u8::MAX, 0);
    let mut b_minmax = (u8::MAX, 0);
    let mut g_minmax = (u8::MAX, 0);

    for pixel in pixel_bucket.iter() {
        let r_val = pixel.0[0];
        let b_val = pixel.0[1];
        let g_val = pixel.0[2];

        r_minmax.0 = r_minmax.0.min(r_val);
        r_minmax.1 = r_minmax.1.max(r_val);

        b_minmax.0 = b_minmax.0.min(b_val);
        b_minmax.1 = b_minmax.1.max(b_val);

        g_minmax.0 = g_minmax.0.min(g_val);
        g_minmax.1 = g_minmax.1.max(g_val);
    }


    let ranges = [
        r_minmax.1 - r_minmax.0,
        g_minmax.1 - g_minmax.0,
        b_minmax.1 - b_minmax.0
    ];

    let largest_range_channel = ranges.iter()
        .enumerate()
        .max_by_key(|x| x.1)
        .expect("Guaranteed to be a max")
        .0;
    
    pixel_bucket.sort_by_key(|x| x.0[largest_range_channel]);
}

fn average_pixels(pixels: &[Rgb<u8>]) -> Rgb<u8> {
    let pixels_size = pixels.len();

    let mut total_pixel_values = (0, 0, 0);
    for pixel in pixels {
        total_pixel_values.0 += pixel.0[0] as usize;
        total_pixel_values.1 += pixel.0[1] as usize;
        total_pixel_values.2 += pixel.0[2] as usize;
    }

    let pixel_values_average = [
        (total_pixel_values.0/pixels_size) as u8,
        (total_pixel_values.1/pixels_size) as u8,
        (total_pixel_values.2/pixels_size) as u8
    ];

    Rgb::from(pixel_values_average)
}

/// SAFETY: `image` length must be a multiple of 3
unsafe fn bucket_from_image_mut(image: &mut [u8], bucket_size: usize, bucket_index: usize) -> &mut [Rgb<u8>] {
    debug_assert_eq!(image.len() % 3, 0);
    let slice_start = bucket_size*bucket_index*3;
    let slice_end = bucket_size*(bucket_index+1)*3;
    let bucket = image.get_mut(slice_start..slice_end).expect("Guaranteed to be in bounds");

    // SAFETY: `bucket` should consist of chunks of 3 contiguous `u8`s,
    // which is the same representation as `Rgb<u8>` ([u8; 3])
    std::slice::from_raw_parts_mut(bucket.as_mut_ptr().cast(), bucket.len()/3)
}

/// https://en.wikipedia.org/wiki/Median_cut
/// 
/// Some pixels at the end of the image may not be included.
pub fn median_cut_palette(rgb_image: &mut RgbImage, palette_n: u8) -> Vec<Rgb<u8>> {
    let pixel_count = rgb_image.len() / 3;

    let iterations = (palette_n as f32).log2().ceil() as u32 + 1;
    let mut bucket_count = 1;
    let mut bucket_size = pixel_count;
    for i in 0..iterations {
        bucket_count = 2_u32.pow(i) as usize;
        bucket_size = pixel_count / bucket_count;

        for bucket_index in 0..bucket_count {
            // SAFETY: `rgb_image` derefs to `[u8]` which has a length of a multiple of 3
            let bucket_pixels = unsafe { bucket_from_image_mut(rgb_image, bucket_size, bucket_index) };
            median_cut_sort_bucket(bucket_pixels);
        }
    }

    debug_assert!(bucket_count >= palette_n as usize);
    let mut colours = Vec::with_capacity(palette_n as usize);
    for bucket_index in 0..palette_n as usize {
        // SAFETY: same as previous usage
        let bucket_pixels = unsafe { bucket_from_image_mut(rgb_image, bucket_size, bucket_index) };
        colours.push(average_pixels(&bucket_pixels));
    }

    colours
}

fn calculate_saturation(pixel: Rgb<u8>) -> u8 {
    let max = *pixel.0.iter().max().expect("Impossible as colour array is not empty");
    let min = *pixel.0.iter().min().expect("Impossible as colour array is not empty");
    (255.0 * (1.0 - min as f32/max as f32)) as u8
}

fn change_brightness(pixel: Rgb<u8>, target_brightness: u8) -> Rgb<u8> {
    let max_brightness = *pixel.0.iter().max().expect("Impossible as colour array is not empty");
    let multiplier = target_brightness as f32 / max_brightness as f32;

    let new_values = [
        (pixel.0[0] as f32 * multiplier).min(255.0) as u8,
        (pixel.0[1] as f32 * multiplier).min(255.0) as u8,
        (pixel.0[2] as f32 * multiplier).min(255.0) as u8
    ];

    Rgb::from(new_values)
}

pub fn get_theme_colour(palette: &[Rgb<u8>], brightness: Option<u8>) -> Option<Rgb<u8>> {
    // choose most saturated colour in palette

    let theme = *palette.iter().max_by_key(|&&x| calculate_saturation(x))?;

    Some(match brightness {
        Some(b) => change_brightness(theme, b),
        None => theme
    })
}
