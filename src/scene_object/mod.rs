pub mod plane;
pub mod sphere;
use crate::{color::Color, ray::Ray};

pub trait SceneObject {
    fn color(&self) -> Color;
    fn intersect(&self, ray: &Ray) -> Option<f32>;
}
