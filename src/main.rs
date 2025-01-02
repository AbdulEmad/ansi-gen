use clap::Parser;
use image::{DynamicImage, GenericImageView, imageops::FilterType, imageops};

#[derive(Parser)]
struct Cli {
    path: String,
    width: Option<u32>,
    height: Option<u32>
}

fn main() {
    let cli = Cli::parse();
    let img = load_image(&cli.path);
    let resized_img = resize_image(&img, cli.width, cli.height);
    let sharpened_img = sharpen_image(&resized_img);
    print_ansi_image(&sharpened_img);
}

fn load_image(path: &str) -> DynamicImage{
    image::io::Reader::open(path)
        .expect("Could not open image")
        .decode()
        .expect("Failed to decode image")
}

fn resize_image(img: &DynamicImage, width: Option<u32>, height: Option<u32>) -> DynamicImage {
    match (width, height) {
        (Some(w), Some(h)) => {img.resize_exact(w, h, FilterType::Lanczos3)},
        (Some(w), None) => {
            let (orig_w, orig_h) = img.dimensions();
            let aspect_ratio = orig_h as f64 / orig_w as f64;
            let new_height = (w as f64 * aspect_ratio) as u32;
            img.resize_exact(w, new_height, FilterType::Lanczos3)
            },
        (None, Some(h)) => {
            let (orig_w, orig_h) = img.dimensions();
            let aspect_ratio = orig_w as f64 / orig_h as f64;
            let new_width = (h as f64 * aspect_ratio) as u32;
            img.resize_exact(new_width, h, FilterType::Lanczos3)
        }
        (None, None) => img.clone(),
        }
}

fn sharpen_image(img: &DynamicImage) -> DynamicImage {
    let mut rgb = img.to_rgb8();

    imageops::unsharpen(&mut rgb, 2.0, 1);
    DynamicImage::ImageRgb8(rgb)
}

fn print_ansi_image(img: &DynamicImage){
    let (w, h) = img.dimensions();
    let rgb_image = img.to_rgb8();

    for y in 0..h {
        for x in 0..w {
            let pixel = rgb_image.get_pixel(x, y);
            let [r, g, b] = pixel.0;
            print!("{}█", ansi_color_fg(r, g, b));
        }
        print!("{}", ansi_reset());
        println!();
    }
}

fn ansi_color_fg(r: u8, g: u8, b: u8) -> String {
    format!("\x1b[38;2;{};{};{}m", r, g, b)
}

fn ansi_reset() -> &'static str {
    "\x1b[0m"
}
