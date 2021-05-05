use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// A 3-dimensional vector.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    /// Returns a new `Vec3`.
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    /// Computes the dot product of two vectors.
    pub fn dot(&self, other: Vec3) -> f32 {
        let x = self.x * other.x;
        let y = self.y * other.y;
        let z = self.z * other.z;
        x + y + z
    }

    /// Computes the cross product of two vectors.
    pub fn cross(&self, other: Vec3) -> Vec3 {
        let x = (self.y * other.z) - (self.z * other.y);
        let y = (self.z * other.x) - (self.x * other.z);
        let z = (self.x * other.y) - (self.y * other.z);
        Vec3::new(x, y, z)
    }

    /// Returns the magnitude of the vector.
    pub fn magnitude(&self) -> f32 {
        self.dot(*self).sqrt()
    }

    /// Returns the normal of the vector.
    pub fn normal(&self) -> Vec3 {
        *self / self.magnitude()
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self::Output {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, vec: Vec3) -> Vec3 {
        Vec3 {
            x: self * vec.x,
            y: self * vec.y,
            z: self * vec.z,
        }
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, scalar: f32) {
        *self = Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, scalar: f32) -> Self::Output {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl Div<Vec3> for f32 {
    type Output = Vec3;

    fn div(self, vec: Vec3) -> Vec3 {
        Vec3 {
            x: self / vec.x,
            y: self / vec.y,
            z: self / vec.z,
        }
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, scalar: f32) {
        *self = Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let vec1 = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let vec2 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(vec1, vec2);
    }

    #[test]
    fn test_add() {
        let vec1 = Vec3::new(1.0, 1.0, 3.0);
        let vec2 = Vec3::new(3.0, 2.0, 4.0);
        let expected = Vec3::new(4.0, 3.0, 7.0);
        let result = vec1 + vec2;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_add_assign() {
        let mut vec1 = Vec3::new(1.0, 1.0, 3.0);
        let vec2 = Vec3::new(3.0, 2.0, 4.0);
        vec1 += vec2;
        let expected = Vec3::new(4.0, 3.0, 7.0);
        assert_eq!(vec1, expected);
    }

    #[test]
    fn test_sub() {
        let vec1 = Vec3::new(1.0, 1.0, 3.0);
        let vec2 = Vec3::new(3.0, 2.0, 4.0);
        let expected = Vec3::new(-2.0, -1.0, -1.0);
        let result = vec1 - vec2;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_sub_assign() {
        let mut vec1 = Vec3::new(1.0, 1.0, 3.0);
        let vec2 = Vec3::new(3.0, 2.0, 4.0);
        let expected = Vec3::new(-2.0, -1.0, -1.0);
        vec1 -= vec2;
        assert_eq!(vec1, expected);
    }

    #[test]
    fn test_dot() {
        let vec1 = Vec3::new(1.0, 2.0, 3.0);
        let vec2 = Vec3::new(3.0, 8.0, 1.0);
        let expected = 22.0;
        let result = vec1.dot(vec2);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_normal() {
        let vec = Vec3::new(0.52, -1.2, 0.0);
        let expected = Vec3::new(1.0, -1.0, 0.0);
        let vec = vec.normal();
        assert_eq!(vec, expected);
    }

    // TODO test `cross`, `Mul`, `MulAssign`, `Div`, `DivAssign`, `normal`, `magnitude`
}
