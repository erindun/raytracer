use crate::{color::Color, scene_object::SceneObject};
use glam::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn cast(ray: Ray, objects: &[Box<dyn SceneObject>]) -> Option<Color> {
        let mut tmin = f32::INFINITY;
        let mut color = None;

        for obj in objects {
            let intersection = obj.intersect(&ray);
            match intersection {
                Some(t) => {
                    if t < tmin {
                        tmin = t;
                        color = Some(obj.color());
                    }
                }
                None => (),
            }
        }

        color
    }
}
