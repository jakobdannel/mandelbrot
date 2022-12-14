use std::fs;

use image::RgbImage;
use rayon::prelude::*;
use structopt::StructOpt;

#[derive(Clone, Copy)]
struct Rgb {
    red: u8,
    green: u8,
    blue: u8,
}

impl From<Rgb> for image::Rgb<u8> {
    fn from(rgb: Rgb) -> Self {
        image::Rgb([rgb.red, rgb.green, rgb.blue])
    }
}

#[derive(StructOpt)]
#[structopt(global_setting = structopt::clap::AppSettings::AllowNegativeNumbers)]
struct Args {
    #[structopt(long, short, default_value = "5000")]
    width: u32,
    #[structopt(long, short, default_value = "5000")]
    height: u32,
    #[structopt(long, short, default_value = "1")]
    frames: usize,
    #[structopt(long, short)]
    colorful: bool,
    #[structopt(long, default_value = "-0.5")]
    x_start: f64, //Coordinate x on the mandelbrot set where the zoom starts
    #[structopt(long, default_value = "0.0")]
    y_start: f64, //Coordinate y on the mandelbrot set where the zoom starts
    #[structopt(long, default_value = "-0.5")]
    x_end: f64, //Coordinate x on the mandelbrot set where the zoom ends
    #[structopt(long, default_value = "0.0")]
    y_end: f64, //Coordinate y on the mandelbrot set where the zoom ends
    #[structopt(long, default_value = "1.0")]
    zoom_start: f64, //Zoom factor at the start, the smaller the number, the closer it is
    #[structopt(long, default_value = "1.0")]
    zoom_end: f64, //Zoom factor at the end
}

fn main() {
    let args = Args::from_args();

    generate_zoom(
        &args,
        args.x_start,
        args.y_start,
        args.x_end,
        args.y_end,
        args.zoom_start,
        args.zoom_end,
    );
}

///Function that generates a series of images, from a start to an end point with zooms
fn generate_zoom(
    args: &Args,
    x_start: f64,
    y_start: f64,
    x_end: f64,
    y_end: f64,
    zoom_start: f64,
    zoom_end: f64,
) {
    let frames = args.frames;

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
        x_min = x_min_start
            - ((f64::log10(1.0 + (9.0 * (i as f64 / frames as f64)))) * (x_min_start - x_min_end));
        x_max = x_max_start
            - ((f64::log10(1.0 + (9.0 * (i as f64 / frames as f64)))) * (x_max_start - x_max_end));
        y_min = y_min_start
            - ((f64::log10(1.0 + (9.0 * (i as f64 / frames as f64)))) * (y_min_start - y_min_end));
        y_max = y_max_start
            - ((f64::log10(1.0 + (9.0 * (i as f64 / frames as f64)))) * (y_max_start - y_max_end));
        println!("{}", i);
        generate_image(args, x_min, x_max, y_min, y_max, i);
    }
}

///Generates an image of the mandelbrot set
fn generate_image(args: &Args, x_min: f64, x_max: f64, y_min: f64, y_max: f64, img_number: usize) {
    let Args {
        width,
        height,
        colorful,
        ..
    } = *args;

    fs::create_dir_all("./output/").expect("Creating output folder");

    let pixels: Vec<((u32, u32), Rgb)> = (0..height)
        .into_par_iter()
        .flat_map(|h| (0..width).into_par_iter().map(move |w| (w, h)))
        .map(|(x, y)| {
            let mut a: f64 = map(x as f64, 0.0, width as f64, x_min, x_max);
            let mut b: f64 = map(y as f64, 0.0, height as f64, y_min, y_max);
            let mut n = 0;

            let a_start = a;
            let b_start = b;
            let c_abs = (a.powf(2.0) + b.powf(2.0)).sqrt();

            if (a + 1.0).powf(2.0) * b.powf(2.0) >= 0.0625
                || c_abs.powf(2.0) * (8.0 * c_abs.powf(2.0) - 3.0) > 0.09375 - a
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
            } else {
                n = 255;
            }

            let rgb = if n == 255 {
                Rgb {
                    red: 0,
                    green: 0,
                    blue: 0,
                }
            } else if colorful {
                hsl_to_rgb(n as f32 / 255.0, 1.0, 0.5)
            } else {
                Rgb {
                    red: n,
                    green: n,
                    blue: n,
                }
            };
            ((x, y), rgb)
        })
        .collect();

    let mut img = RgbImage::new(width, height);
    for ((x, y), rgb) in pixels {
        img.put_pixel(x, y, rgb.into());
    }
    let outputpath: String = format!("./output/{}.png", img_number);
    img.save(outputpath).expect("Writing image to png");
}

///Maps min and max values to different min and max values
fn map(x: f64, min: f64, max: f64, a: f64, b: f64) -> f64 {
    (x - min) / (max - min) * (b - a) + a
}

///Generates Rgb values from HSL values
fn hsl_to_rgb(hue: f32, saturation: f32, luminance: f32) -> Rgb {
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

    Rgb {
        red: ((r + m) * 255.0) as u8,
        green: ((g + m) * 255.0) as u8,
        blue: ((b + m) * 255.0) as u8,
    }
}
