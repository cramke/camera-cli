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

    #[arg(long)]
    no_color: bool,
}

fn main() {
    let args = Args::parse();

    if let (Some(pixels), Some(angle), Some(_distance)) = (args.pixels, args.angle, args.distance) {
        let detection = camera::detection_distance(angle, pixels);
        let identification = camera::identification_distance(angle, pixels);
        let recognition = camera::recognition_distance(angle, pixels);

        print_results_table(
            "Camera Ranges",
            &[
                ("Detection", detection.into()),
                ("Identification", identification.into()),
                ("Recognition", recognition.into()),
            ],
            /* use_color = */ color_enabled(!args.no_color),
        );
    } else {
        eprintln!(
            "Please provide --angle, --distance, and --pixels to calculate spatial resolution."
        );
        // (Optional) print a short usage hint
        eprintln!("Example: mytool --angle 60 --distance 30 --pixels 1200");
    }
}

fn color_enabled(cli_allows: bool) -> bool {
    if !cli_allows {
        return false;
    }
    // Respect NO_COLOR: https://no-color.org
    std::env::var_os("NO_COLOR").is_none()
}

fn print_results_table(title: &str, rows: &[(&str, f64)], use_color: bool) {
    let label_width = rows.iter().map(|(l, _)| l.len()).max().unwrap_or(8).max(10);
    let num_width = 12; // room for digits + decimals + unit
    let total_width = 2 + label_width + 3 + num_width + 2; // borders + padding

    let mut hr = |ch: char| -> String { std::iter::repeat(ch).take(total_width).collect() };

    // Optional color helpers
    let style_title = |s: &str| {
        if use_color {
            #[cfg(feature = "color")]
            {
                s.bold().to_string()
            }
            #[cfg(not(feature = "color"))]
            {
                s.to_string()
            }
        } else {
            s.to_string()
        }
    };
    let style_label = |s: &str| {
        if use_color {
            #[cfg(feature = "color")]
            {
                s.bold().to_string()
            }
            #[cfg(not(feature = "color"))]
            {
                s.to_string()
            }
        } else {
            s.to_string()
        }
    };

    println!("{}", hr('═'));
    println!(
        "║ {:^width$} ║",
        style_title(title),
        width = label_width + 3 + num_width
    );
    println!("{}", hr('─'));

    println!(
        "║ {:<lw$} │ {:>nw$} ║",
        "Metric",
        "Value (m)",
        lw = label_width,
        nw = num_width
    );
    println!("{}", hr('─'));

    for (label, value) in rows {
        println!(
            "║ {:<lw$} │ {:>nw$.2} ║",
            style_label(label),
            value,
            lw = label_width,
            nw = num_width
        );
    }

    println!("{}", hr('═'));
}
