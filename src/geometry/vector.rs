use bytemuck::{Pod, Zeroable};
use crate::physics::math::DeterministicMath;

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Vector3d {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3d {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn add(&self, other: &Vector3d) -> Vector3d {
        Vector3d::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    pub fn subtract(&self, other: &Vector3d) -> Vector3d {
        Vector3d::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    pub fn scale(&self, scalar: f32) -> Vector3d {
        Vector3d::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).d_sqrt()
    }

    pub fn normalize(&self) -> Vector3d {
        let mag = self.magnitude();
        if mag == 0.0 {
            *self
        } else {
            self.scale(1.0 / mag)
        }
    }

    pub fn dot(&self, other: &Vector3d) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vector3d) -> Vector3d {
        Vector3d {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn length_squared(&self) -> f32 {
        self.dot(self)
    }

    pub fn project_onto(&self, other: &Vector3d) -> Vector3d {
        let scalar = self.dot(other) / other.length_squared();
        other.scale(scalar)
    }

    pub fn angle_between(&self, other: &Vector3d) -> f32 {
        let dot = self.dot(other);
        let magnitudes = self.magnitude() * other.magnitude();
        (dot / magnitudes).d_acos()
    }

    pub fn distance_to(&self, other: &Vector3d) -> f32 {
        self.subtract(other).magnitude()
    }

    pub fn reflect(&self, normal: &Vector3d) -> Vector3d {
        let scalar = 2.0 * self.dot(normal);
        self.subtract(&normal.scale(scalar))
    }

    pub fn lerp(&self, other: &Vector3d, t: f32) -> Vector3d {
        self.scale(1.0 - t).add(&other.scale(t))
    }
}
