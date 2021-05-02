mod color;
mod ray;
mod vec3;

use color::Color;
use vec3::Vec3;
use std::fs::File;
use std::io::{self, Write};

fn main() {
    // Set the image size.
    let aspect: f32 = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect) as i32;

    // Set the camera.
    let viewport_height: f32 = 2.0;
    let viewport_width: f32 = aspect * viewport_height;
    let focal_length: f32 = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    let mut f = File::create("output.ppm").expect("Error creating file.");

    // PPM file header.
    writeln!(f, "P3\n{} {}\n255", image_width, image_height).expect("Error writing to file.");

    // Render the image.
    for j in (0..image_height).rev() {
        for i in 0..image_width {
            print!("\rScanlines remaining: {}", j);
            print!(" ");
            io::stdout().flush().unwrap();

            let color = Color::new(
                i as f32 / (image_width - 1) as f32,
                j as f32 / (image_height - 1) as f32,
                0.25,
            );

            writeln!(f, "{}", color).expect("Error writing to file.");
        }
    }
}
