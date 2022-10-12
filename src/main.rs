use std::fs;

use image::{Rgb, RgbImage};

struct RGB {
    red: u8,
    green: u8,
    blue: u8,
}

const WIDTH: u32 = 5000; //Image width
const HEIGHT: u32 = 5000; //Image height
const FRAMES: u32 = 1; //Amount of frames generated, set to 1 to render a single image
const COLORFUL: bool = false; //Set colorful mode

fn main() {
    let x_start = -0.5; //Coordinate x on the mandelbrot set where the zoom starts
    let y_start = 0.0; //Coordinate y on the mandelbrot set where the zoom starts
    let x_end = x_start; //Coordinate x on the mandelbrot set where the zoom ends
    let y_end = y_start; //Coordinate y on the mandelbrot set where the zoom ends

    let zoom_start = 1.0; //Zoom factor at the start, the smaller the number, the closer it is
    let zoom_end = zoom_start; //Zoom factor at the end

    generate_zoom(FRAMES, x_start, y_start, x_end, y_end, zoom_start, zoom_end);
}

///Function that generates a series of images, from a start to an end point with zooms
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

///Generates an image of the mandelbrot set
fn generate_image(x_min: f64, x_max: f64, y_min: f64, y_max: f64, img_number: u32) {
    fs::create_dir_all("./output/").expect("Creating output folder");

    let mut img = RgbImage::new(WIDTH, HEIGHT);
    let mut rgb: RGB = RGB {
        red: 0,
        green: 0,
        blue: 0,
    };
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let mut a: f64 = map(x as f64, 0.0, WIDTH as f64, x_min, x_max);
            let mut b: f64 = map(y as f64, 0.0, HEIGHT as f64, y_min, y_max);
            let mut n = 0;

            let a_start = a;
            let b_start = b;
            let c_abs = (a.powf(2.0) + b.powf(2.0)).sqrt();

            if !((a + 1.0).powf(2.0) * b.powf(2.0) < 0.0625)
                || !(c_abs.powf(2.0) * (8.0 * c_abs.powf(2.0) - 3.0) <= 0.09375 - a)
            {
                while n < 255 {
                    let aa = a * a - b * b;
                    let bb = 2.0 * a * b;

                    a = aa + a_start;
                    b = bb + b_start;

                    if (a + b).abs() > 16.0 {
                        break;
                    }
                    n += 1;
                }
            }

            if COLORFUL {
                rgb = hsl_to_rgb(n as f32 / 255.0, 1.0, 0.5);
            } else if !COLORFUL {
                rgb.red = n;
                rgb.green = n;
                rgb.blue = n;
            }

            if n == 255 {
                rgb.red = 0;
                rgb.green = 0;
                rgb.blue = 0;
            }
            
            img.put_pixel(x, y, Rgb([rgb.red, rgb.green, rgb.blue]));
        }
    }
    let outputpath: String =
        "./output/".to_string() + &img_number.to_string() + &".png".to_string();
    img.save(outputpath).expect("Writing image to png");
}

///Maps min and max values to different min and max values
fn map(x: f64, min: f64, max: f64, a: f64, b: f64) -> f64 {
    (x - min) / (max - min) * (b - a) + a
}

///Generates RGB values from HSL values
fn hsl_to_rgb(hue: f32, saturation: f32, luminance: f32) -> RGB {
    let c = (1.0 - (2.0 * luminance - 1.0).abs()) * saturation;
    let h = hue * 6.0;
    let x = c * (1.0 - (h % 2.0 - 1.0).abs());
    let m = luminance - (c / 2.0);

    let i = h.floor() as usize;
    let mut rgb_table = [c, x, 0.0];
    if i & 1 == 1 {
        rgb_table.swap(0, 1);
    }
    let (r, g, b) = (
        rgb_table[(i / 2) % 3],
        rgb_table[(i / 2 + 1) % 3],
        rgb_table[(i / 2 + 2) % 3],
    );

    RGB {
        red: ((r + m) * 255.0) as u8,
        green: ((g + m) * 255.0) as u8,
        blue: ((b + m) * 255.0) as u8,
    }
}
