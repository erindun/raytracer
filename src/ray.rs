use crate::color::Color;
use crate::sphere::Sphere;
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
    if sphere.hit(ray).is_some() {
        return Color::new(1.0, 0.0, 0.0);
    }

    let unit_direction = ray.direction;
    let t = 0.5 * (unit_direction.y + 1.0);

    let colors: Vec<f32> = vec![0.5, 0.7, 1.0]
        .iter()
        .map(|c| 1.0 * (1.0 - t) + t * c)
        .collect();

    Color::new(colors[0], colors[1], colors[2])
}
