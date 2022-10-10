use image::{Rgb, RgbImage};

fn main() {
    const WIDTH: u32 = 360;
    const HEIGHT:u32 = 240;
    let mut img = RgbImage::new(WIDTH, HEIGHT);

    for i in 0..WIDTH {
        for j in 0..HEIGHT {
            img.put_pixel(i, j, Rgb([51,51,51]));
        }
    }
    

    img.save("./output/output.png").expect("Writing image to png");
}
