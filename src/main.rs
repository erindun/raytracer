mod color;
mod ray;
mod vec3;

use color::Color;
use std::fs::File;
use std::io::{self, Write};

fn main() {
    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;

    let mut f = File::create("output.ppm").expect("Error creating file.");

    // PPM file header.
    writeln!(f, "P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT).expect("Error writing to file.");

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            print!("\rScanlines remaining: {}", j);
            print!(" ");
            io::stdout().flush().unwrap();

            let color = Color {
                r: (i as f32 / (IMAGE_WIDTH - 1) as f32),
                g: (j as f32 / (IMAGE_HEIGHT - 1) as f32),
                b: 0.25,
            };

            writeln!(f, "{}", color).expect("Error writing to file.");
        }
    }
}
