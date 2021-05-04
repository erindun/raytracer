use crate::color::Color;
use crate::vec3::Vec3;

pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}

pub fn gen_ray_color(ray: &Ray) -> Color {
    let unit_direction = ray.direction;
    let t = 0.5 * (unit_direction.y + 1.0);
    println!("t: {:?} unit_direction: {:?} ", t, unit_direction);

    let colors: Vec<f32> = vec![0.5, 0.7, 1.0]
        .iter()
        .map(|c| 1.0 * (1.0 - t) + t * c)
        .collect();

    Color::new(colors[0], colors[1], colors[2])
}
