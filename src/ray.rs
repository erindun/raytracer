use crate::color::Color;
use crate::vec3::Vec3;

struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    fn at(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}

fn gen_ray_color(ray: &Ray) -> Color {
    let unit_direction = ray.direction.normal();
    let t = 0.5 * Vec3::new(unit_direction.x, unit_direction.y + 1.0, unit_direction.z);

    let r = (1.0 * (1.0 - t.x)) + t.x * 0.5;
    let g = (1.0 * (1.0 - t.y)) + t.y * 0.7;
    let b = (1.0 * (1.0 - t.z)) + t.z * 1.0;

    Color::new(r, g, b)
}
