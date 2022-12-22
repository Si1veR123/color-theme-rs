use image::{RgbImage, Rgb};


fn median_cut_sort_bucket(pixel_bucket: &mut [Rgb<u8>]) {
    let mut r_minmax: (u8, u8) = (u8::MAX, 0);
    let mut b_minmax: (u8, u8) = (u8::MAX, 0);
    let mut g_minmax: (u8, u8) = (u8::MAX, 0);

    for pixel in pixel_bucket.iter() {
        let r_val = pixel.0.get(0).unwrap();
        let b_val = pixel.0.get(1).unwrap();
        let g_val = pixel.0.get(2).unwrap();

        r_minmax.0 = r_minmax.0.min(*r_val);
        r_minmax.1 = r_minmax.1.max(*r_val);

        b_minmax.0 = b_minmax.0.min(*b_val);
        b_minmax.1 = b_minmax.1.max(*b_val);

        g_minmax.0 = g_minmax.0.min(*g_val);
        g_minmax.1 = g_minmax.1.max(*g_val);
    }

    let largest_range_channel: usize;
    let r_range = r_minmax.1 - r_minmax.0;
    let g_range = g_minmax.1 - g_minmax.0;
    let b_range = b_minmax.1 - b_minmax.0;

    if r_range > g_range {
        // r > g
        if r_range > b_range {
            // r > b and r > g
            largest_range_channel = 0;
        }
        else {
            // g < r < b
            largest_range_channel = 2;
        }
    } else {
        // r < g
        if g_range > b_range {
            // g > r and g > b
            largest_range_channel = 1;
        } else {
            // r < g < b
            largest_range_channel = 2;
        }
    }
    
    pixel_bucket.sort_by(|a, b| a.0.get(largest_range_channel).unwrap().cmp(b.0.get(largest_range_channel).unwrap()));
}

fn average_pixels(pixels: &[Rgb<u8>]) -> Rgb<u8> {
    let pixels_size = pixels.len();
    let mut total_pixel_values: (usize, usize, usize) = (0, 0, 0);
    for pixel in pixels {
        total_pixel_values.0 += *pixel.0.get(0).unwrap() as usize;
        total_pixel_values.1 += *pixel.0.get(1).unwrap() as usize;
        total_pixel_values.2 += *pixel.0.get(2).unwrap() as usize;
    }
    let pixel_values_average = [(total_pixel_values.0/pixels_size) as u8, (total_pixel_values.1/pixels_size) as u8, (total_pixel_values.2/pixels_size) as u8];
    Rgb::from(pixel_values_average)
}

pub fn median_cut_palette(rgb_image: &RgbImage, palette_n: u8) -> Vec<Rgb<u8>> {
    // https://en.wikipedia.org/wiki/Median_cut
    // may not always include some end pixels
    let mut pixels: Vec<Rgb<u8>> = rgb_image.pixels().cloned().collect();
    let pixel_count = pixels.len();
    let pixels_boxed = pixels.as_mut_slice();

    let iterations = f32::log2(palette_n as f32) as u32 + 1;
    let mut colours = vec![];
    for i in 0..iterations {
        colours = vec![];
        let buckets = 2_u8.pow(i) as usize;
        let bucket_size = pixel_count / buckets;
        for bucket_index in 0..buckets {
            let slice_start = bucket_size*bucket_index;
            let slice_end = bucket_size*(bucket_index+1);

            let bucket = &mut pixels_boxed[slice_start..slice_end];
            median_cut_sort_bucket(bucket);

            if i == (iterations-1) {
                let colour = average_pixels(bucket);
                colours.push(colour)
            }
        }
    }
    colours
}

fn calculate_saturation(pixel: &Rgb<u8>) -> u8 {
    let maxi = *pixel.0.iter().max().unwrap() as f32;
    let mini = *pixel.0.iter().min().unwrap() as f32;
    (255f32 * (1f32 - mini/maxi)) as u8
}

pub fn format_rgb(pixel: &Rgb<u8>) -> String {
    format!("rgb({}, {}, {})", pixel.0.get(0).unwrap(), pixel.0.get(1).unwrap(), pixel.0.get(2).unwrap())
}

fn change_brightness(pixel: &Rgb<u8>, target_brightness: u8) -> Rgb<u8> {
    let max_brightness = pixel.0.iter().max().unwrap().clone();
    let multiplier = target_brightness as f32 / max_brightness as f32;

    let pixel_values = (*pixel.0.get(0).unwrap() as f32, *pixel.0.get(1).unwrap() as f32, *pixel.0.get(2).unwrap() as f32);
    let new_values = [255f32.min(pixel_values.0*multiplier) as u8, 255f32.min(pixel_values.1*multiplier) as u8, 255f32.min(pixel_values.2*multiplier) as u8];

    Rgb::from(new_values)
}

pub fn get_theme_colour(palette: &Vec<Rgb<u8>>, brightness: Option<u8>) -> Rgb<u8> {
    // choose most saturated colour in palette
    let mut palette_cloned = palette.clone();

    palette_cloned.sort_by(
        |a, b| {
            let a_sat = calculate_saturation(a);
            let b_sat = calculate_saturation(b);
            b_sat.cmp(&a_sat)
        }
    );

    let theme = palette_cloned.get(0).unwrap();

    if brightness.is_some() {
        return change_brightness(theme, brightness.unwrap())
    } else {
        return theme.clone()
    }
}
