mod color;
mod ray;
mod hittable;
mod vec3;

use color::Color;
use ray::Ray;
use hittable::sphere::Sphere;
use std::fs::File;
use std::io::{self, Write};
use vec3::Vec3;

fn main() {
    // Set the image size.
    let aspect = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect) as i32;

    // Set the camera.
    let viewport_height = 2.0;
    let viewport_width = aspect * viewport_height;
    let focal_length: f32 = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    let sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, Color::new(1.0, 0.0, 0.0));

    let mut f = File::create("output.ppm").expect("Error creating file.");

    // PPM file header.
    writeln!(f, "P3\n{} {}\n255", image_width, image_height).expect("Error writing to file.");

    // Render the image.
    for j in (0..image_height).rev() {
        print!("\rScanlines remaining: {}", j);
        print!(" ");
        io::stdout().flush().unwrap();
        for i in 0..image_width {
            let u = i as f32 / ((image_width - 1) as f32);
            let v = j as f32 / ((image_height - 1) as f32);
            let ray = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_color = ray::gen_ray_color(&ray, &sphere);

            writeln!(f, "{}", pixel_color).expect("Error writing to file.");
        }
    }
}
