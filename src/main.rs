use image::{Rgb, RgbImage};
const WIDTH: u32 = 1080;
const HEIGHT: u32 = 1080;
const FRAMES: u32 = 250;
fn main() {
    let x_start = -0.9335;
    let y_start = 0.2432;
    let x_end = -0.9335;
    let y_end = 0.2432;

    let zoom_start = 0.01;
    let zoom_end = 0.000001;

    generate_zoom(FRAMES, x_start, y_start, x_end, y_end, zoom_start, zoom_end);
}

fn generate_zoom(
    frames: u32,
    x_start: f64,
    y_start: f64,
    x_end: f64,
    y_end: f64,
    zoom_start: f64,
    zoom_end: f64,
) {
    let x_min_start: f64 = x_start - zoom_start;
    let x_max_start: f64 = x_start + zoom_start;
    let y_min_start: f64 = y_start - zoom_start;
    let y_max_start: f64 = y_start + zoom_start;
    let x_min_end: f64 = x_end - zoom_end;
    let x_max_end: f64 = x_end + zoom_end;
    let y_min_end: f64 = y_end - zoom_end;
    let y_max_end: f64 = y_end + zoom_end;
    let mut x_min;
    let mut x_max;
    let mut y_min;
    let mut y_max;

    for i in 0..frames {
        x_min = x_min_start - ((i as f64 / frames as f64) * (x_min_start - x_min_end));
        x_max = x_max_start - ((i as f64 / frames as f64) * (x_max_start - x_max_end));
        y_min = y_min_start - ((i as f64 / frames as f64) * (y_min_start - y_min_end));
        y_max = y_max_start - ((i as f64 / frames as f64) * (y_max_start - y_max_end));
        println!("{}", i);
        generate_image(x_min, x_max, y_min, y_max, i);
    }
}

fn generate_image(x_min: f64, x_max: f64, y_min: f64, y_max: f64, img_number: u32) {
    let mut img = RgbImage::new(WIDTH, HEIGHT);
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let mut a: f64 = map(x as f64, 0.0, WIDTH as f64, x_min, x_max);
            let mut b: f64 = map(y as f64, 0.0, HEIGHT as f64, y_min, y_max);
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
    let outputpath: String =
        "./output/".to_string() + &img_number.to_string() + &".png".to_string();
    img.save(outputpath).expect("Writing image to png");
}

fn map(x: f64, min: f64, max: f64, a: f64, b: f64) -> f64 {
    (x - min) / (max - min) * (b - a) + a
}
