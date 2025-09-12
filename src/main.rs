use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Horizontal pixels
    #[arg(short, long)]
    pixels: Option<u32>,

    /// Angle in degrees
    #[arg(short, long)]
    angle: Option<f32>,

    /// Working Distance in meters
    #[arg(short, long)]
    distance: Option<f32>,
}

fn main() {
    let args: Args = Args::parse();

    if let (Some(pixels), Some(angle), Some(_distance)) = (args.pixels, args.angle, args.distance) {
        let detection_distance = camera::detection_distance(angle, pixels);
        println!("Detection Distance: {:.2}", detection_distance);
        let identification_distance = camera::identification_distance(angle, pixels);
        println!("Identification Distance: {:.2}", identification_distance);
        let recognition_distance = camera::recognition_distance(angle, pixels);
        println!("Recognition Distance: {:.2}", recognition_distance);
    } else {
        println!("Please provide angle, distance, and pixels to calculate spatial resolution.");
    }
}
