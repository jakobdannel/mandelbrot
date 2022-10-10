use image::{Rgb, RgbImage};

fn main() {
    const WIDTH: u32 = 500;
    const HEIGHT:u32 = 500;
    let mut img = RgbImage::new(WIDTH, HEIGHT);

    img.put_pixel(250, 250, Rgb([255,255,255]));

    img.save("./output/output.png").expect("Writing image to png");
}
