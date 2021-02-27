mod camera;
mod ray;
mod scene_object;

use camera::Camera;
use color::Color;
use glam::Vec3;
use ray::Ray;
use scene_object::{plane::Plane, sphere::Sphere, SceneObject};

use std::fs::File;
use std::io::prelude::*;

const NX: i32 = 1024;
const NY: i32 = 768;

mod color {
    #[derive(Copy, Clone)]
    pub struct Color {
        pub r: f32,
        pub g: f32,
        pub b: f32,
    }
}

fn main() {
    let camera = Camera::new();

    // Setup scene.
    let objects: Vec<Box<dyn SceneObject>> = vec![
        Box::new(Sphere::new(
            Vec3::new(1.0, 2.0, 15.0),
            3.0,
            Color {
                r: 0.5,
                g: 1.0,
                b: 0.0,
            },
        )),
        Box::new(Plane::new(
            Vec3::new(0.0, 2.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Color {
                r: 0.0,
                g: 0.5,
                b: 1.0,
            },
        )),
    ];

    // Create ppm file.
    let mut file = File::create("output.ppm").expect("Error creating file.");
    write!(file, "P3\n{} {}\n255\n", NX, NY).expect("Error writing to file.");

    for j in 0..NY {
        for i in 0..NX {
            let x: f32 = i as f32 / NX as f32;
            let y: f32 = j as f32 / NY as f32;
            let direction = camera.calculate_ray_dir(x, y);

            let ray = Ray::new(camera.eye(), direction);
            let color = ray::cast(ray, &objects);

            let buffer;
            match color {
                Some(c) => {
                    let r = (c.r * 255.0) as i32;
                    let g = (c.g * 255.0) as i32;
                    let b = (c.b * 255.0) as i32;
                    buffer = format!("{} {} {}", r, g, b);
                }
                None => {
                    buffer = String::from("0 0 0");
                }
            }
            writeln!(file, "{}", buffer).expect("Error writing to file.");
        }
    }
}
