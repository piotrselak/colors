use image::{self, GenericImageView, ImageBuffer, Pixel, Rgb};
use rand::Rng;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let image_path = &args[1];
    println!("Chosen image: {}", image_path);

    let image = image::open(image_path).expect("Could not parse/find given image.");

    let (width, height) = image.dimensions();
    let mut output: ImageBuffer<Rgb<u8>, Vec<_>> = ImageBuffer::new(width, height);

    for (x, y, pixel) in image.pixels() {
        let new_pixel = pixel.to_rgb().map(|channel| {
            let random: u8 = rand::thread_rng().gen_range(0..=255);
            if random > channel {
                0
            } else {
                255
            }
        });
        output.put_pixel(x, y, new_pixel)
    }

    output
        .save("./output.png")
        .expect("Could not save the image.");
}
