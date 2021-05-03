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
    let unit_direction = ray.direction.normal();
    let t = 0.5 * (unit_direction.y + 1.0);

    let r = (1.0 * (1.0 - t)) + t * 0.5;
    let g = (1.0 * (1.0 - t)) + t * 0.7;
    let b = (1.0 * (1.0 - t)) + t * 1.0;
    Color::new(r, g, b)
}
