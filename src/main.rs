use std::env;
use std::path;
use image::{io::Reader, GenericImageView, ImageBuffer, GrayImage, Luma};
use array2d::Array2D;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return;
    }

    // Format filenames
    let input_filename: String = String::from(&args[1]);
    let output_filename: String = match args.len() {
        2 => {
            let mut path = path::PathBuf::from(&input_filename);
            path.set_extension("dithered.png");
            path.to_string_lossy().into_owned()
        },
        _ => String::from(&args[2])
    };

    let input = Reader::open(&input_filename).unwrap().decode().unwrap();
    let mut output : GrayImage = ImageBuffer::new(input.width(), input.height());
    let mut buffer = Array2D::<f32>::filled_with(0.0, input.width() as usize, input.height() as usize);

    // Fill buffer
    for y in 0..input.height() {
        for x in 0..input.width() {
            let pixel = input.get_pixel(x, y);
            let greyscale: f32 = ((((77 * (pixel[0] as i32)) + (150 * (pixel[1] as i32)) + (29 * (pixel[2] as i32)) + 128) >> 8) as f32) / 255.0;
            buffer[(x as usize, y as usize)] = greyscale;
        }
    }

    // Dither
    for y in 0..input.height() {
        for x in 0..input.width() {
            let old_pixel : f32 = buffer[(x as usize, y as usize)];
            let new_pixel : f32 = if old_pixel > 0.5 { 1.0 } else { 0.0 };
            let error : f32 = old_pixel - new_pixel;

            if (x + 1) < input.width() {
                buffer[((x + 1) as usize, y as usize)] += error * 7.0 / 16.0;
            }

            if (x > 0) && (y + 1) < input.height() {
                buffer[((x - 1) as usize, (y + 1) as usize)] += error * 3.0 / 16.0;
            }

            if (y + 1) < input.height() {
                buffer[(x as usize, (y + 1) as usize)] += error * 5.0 / 16.0;
            }

            if (x + 1) < input.width() && (y + 1) < input.height() {
                buffer[((x + 1) as usize, (y + 1) as usize)] += error * 1.0 / 16.0;
            }

            let pixel : u8 = if new_pixel == 1.0 { 255 } else { 0 };
            output.put_pixel(x, y, Luma([pixel]));
        }
    }

    // Write
    output.save(&output_filename).unwrap();
}
