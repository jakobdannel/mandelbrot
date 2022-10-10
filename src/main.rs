use image::{Rgb, RgbImage};

fn main() {
    const WIDTH: u32 = 5000;
    const HEIGHT: u32 = 5000;

    let mut img = RgbImage::new(WIDTH, HEIGHT);

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let mut a: f32 = map(x as f32, 0.0, WIDTH as f32, -2.0, 1.0);
            let mut b: f32 = map(y as f32, 0.0, WIDTH as f32, -1.5, 1.5);
            let mut n = 0;
            let mut brightness: u8;

            let a_start = a;
            let b_start = b;

            while n < 255 {
                let aa = a * a - b * b;
                let bb = 2.0 * a * b;

                a = aa + a_start;
                b = bb + b_start;

                if (a + b).abs() as u32 > 32 {
                    break;
                }
                n += 1;
            }
            brightness = n;

            if n == 255 {
                brightness = 0;
            }

            img.put_pixel(x, y, Rgb([brightness, brightness, brightness]));
        }
    }
    img.save("./output/output.png")
        .expect("Writing image to png");
}

fn map(x: f32, min: f32, max: f32, a: f32, b: f32) -> f32 {
    (x - min) / (max - min) * (b - a) + a
}
