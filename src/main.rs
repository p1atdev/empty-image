use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Image height to generate
    #[arg(long, default_value_t = 512)]
    height: u32,

    /// Image width to generate
    #[arg(long, default_value_t = 512)]
    width: u32,

    /// Output file name
    #[arg(long, default_value = "empty.png")]
    output: String,
}

fn main() {
    let args = Args::parse();

    // size check
    if args.height <= 0 || args.width <= 0 {
        println!("Invalid image size. Please specify a positive integer.");
        return;
    }

    let mut img = image::RgbaImage::new(args.width, args.height);

    for (_x, _y, pixel) in img.enumerate_pixels_mut() {
        *pixel = image::Rgba([0, 0, 0, 0]);
    }

    img.save(args.output).unwrap();
}
