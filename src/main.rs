use rand::prelude::*;
use image::GenericImage;
use image::GenericImageView;

fn main() {
    // image size
    const GX: u32 = 2048;
    const GY: u32 = 2048;

    let img = image::DynamicImage::new_rgba8(GX, GY);
    let mut out = image::DynamicImage::new_rgba8(GX, GY);
 
    let mut rng = rand::thread_rng();

    for (x, y, mut pixel) in img.pixels() {
        let random: [u8; 4] = [rng.gen(), rng.gen(), rng.gen(), rng.gen(),];
        pixel.0 = random;

        out.put_pixel(x, y, pixel)
    }

    out.save("image.png");
}