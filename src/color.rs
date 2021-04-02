use std::fmt;

#[derive(Copy, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            (self.r * 255.999) as i32,
            (self.g * 255.999) as i32,
            (self.b * 255.999) as i32,
        )
    }
}
