use clap::Parser;
use comfy_table::{ContentArrangement, Table, presets::UTF8_FULL};

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

    #[arg(long)]
    no_color: bool,
}

fn main() {
    let args: Args = Args::parse();

    if let (Some(pixels), Some(angle)) = (args.pixels, args.angle) {
        let detection_distance = camera::detection_distance(angle, pixels);
        let identification_distance = camera::identification_distance(angle, pixels);
        let recognition_distance = camera::recognition_distance(angle, pixels);

        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL) // pretty borders
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec!["Metric", "Value (m)"]);

        table.add_row(vec![
            "Detection (25 ppm)",
            &format!("{:.2}", detection_distance),
        ]);
        table.add_row(vec![
            "Recognition (125 ppm)",
            &format!("{:.2}", recognition_distance),
        ]);
        table.add_row(vec![
            "Identification (250 ppm)",
            &format!("{:.2}", identification_distance),
        ]);

        println!("{table}");
    } else {
        eprintln!(
            "Please provide --angle, --distance, and --pixels to calculate spatial resolution."
        );
        eprintln!("Example: mytool --angle 60 --distance 30 --pixels 1200");
    }

    println!("\n");

    if let (Some(pixels), Some(angle), Some(distance)) = (args.pixels, args.angle, args.distance) {
        let spatial_resolution: f32 = camera::spatial::spatial_resolution(angle, pixels, distance);
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL) // pretty borders
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec!["Metric", "Value (ppm)"]);
        table.add_row(vec![
            "Spatial Resolution",
            &format!("{:.2}", spatial_resolution),
        ]);
        println!("{table}");
    }
}
