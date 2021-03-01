mod camera;
mod color;
mod light;
mod ray;
mod scene_object;

use camera::Camera;
use color::Color;
use glam::Vec3;
use light::Light;
use ray::Ray;
use scene_object::{plane::Plane, sphere::Sphere, SceneObject};

use std::fs::File;
use std::io::prelude::*;

const NX: i32 = 1024;
const NY: i32 = 768;

fn main() {
    let camera = Camera {
        origin: Vec3::new(0.0, 0.0, 0.0),
        forward: Vec3::new(0.0, 0.0, 1.0),
        up: Vec3::new(0.0, 1.0, 0.0),
    };

    // Setup scene.
    let objects: Vec<Box<dyn SceneObject>> = vec![
        Box::new(Sphere {
            origin: Vec3::new(1.1, 1.25, 7.0),
            radius: 1.0,
            color: Color {
                r: 0.5,
                g: 0.5,
                b: 1.0,
            },
        }),
        Box::new(Plane {
            origin: Vec3::new(0.0, 2.0, 0.0),
            normal: Vec3::new(0.0, -1.0, 0.0),
            color: Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
            },
        }),
        Box::new(Plane {
            origin: Vec3::new(0.0, -2.0, 0.0),
            normal: Vec3::new(0.0, 1.0, 0.0),
            color: Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
            },
        }),
        Box::new(Plane {
            origin: Vec3::new(-2.0, 0.0, 0.0),
            normal: Vec3::new(1.0, 0.0, 0.0),
            color: Color {
                r: 1.0,
                g: 0.0,
                b: 0.0,
            },
        }),
        Box::new(Plane {
            origin: Vec3::new(2.0, 0.0, 0.0),
            normal: Vec3::new(-1.0, 0.0, 0.0),
            color: Color {
                r: 0.0,
                g: 1.0,
                b: 0.0,
            },
        }),
        Box::new(Plane {
            origin: Vec3::new(0.0, 0.0, 10.0),
            normal: Vec3::new(0.0, 0.0, -1.0),
            color: Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
            },
        }),
    ];

    let _light = Light {
        origin: Vec3::new(-1.0, -1.0, 7.0),
        color: Color {
            r: 2.0,
            g: 2.0,
            b: 2.0,
        },
    };

    // Create ppm file.
    let mut file = File::create("output.ppm").expect("Error creating file.");
    write!(file, "P3\n{} {}\n255\n", NX, NY).expect("Error writing to file.");

    for j in 0..NY {
        for i in 0..NX {
            let x: f32 = i as f32 / NX as f32;
            let y: f32 = j as f32 / NY as f32;
            let direction = camera.calculate_ray_dir(x, y);

            let ray = Ray::new(camera.origin, direction);
            let color = Ray::cast(ray, &objects);

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
