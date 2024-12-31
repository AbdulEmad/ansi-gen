use clap::Parser;
use image::{DynamicImage, GenericImageView, imageops::FilterType};

#[derive(Parser)]
struct Cli {
    path: String,
    width: u32
}

fn main() {
    let cli = Cli::parse();
    let img = load_image(&cli.path);
    let resized_img = resize_image(&img, cli.width);
    print_ansi_image(&resized_img);
}

fn load_image(path: &str) -> DynamicImage{
    image::io::Reader::open(path)
        .expect("Could not open image")
        .decode()
        .expect("Failed to decode image")
}

fn resize_image(img: &DynamicImage, width: u32) -> DynamicImage {
    let (w,h) = img.dimensions();
    let aspect_ratio = h as f64 / w as f64;
    let new_height = (width as f64 * aspect_ratio) as u32;
    img.resize_exact(width, new_height, FilterType::Triangle)
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
