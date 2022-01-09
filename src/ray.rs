use crate::color::Color;
use crate::hittable::{sphere::Sphere, Hittable};
use crate::vec3::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}

pub fn gen_ray_color(ray: &Ray, sphere: &Sphere) -> Color {
    if let Some(hit) = sphere.hit(&ray, f32::MIN, f32::MAX) {
        let n = (ray.at(hit.t) - Vec3::new(0.0, 0.0, -1.0)).normal();
        return 0.5 * Color::new(n.x + 1.0, n.y + 1.0, n.z + 1.0);
    }

    let unit_direction = ray.direction.normal();
    let t = 0.5 * (unit_direction.y + 1.0);

    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}
