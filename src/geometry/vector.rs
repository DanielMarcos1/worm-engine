use wide::f32x4;
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
    // Constructor of the trait
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

    // Internal helper to get SIMD representation (padding with 0.0)
    #[inline(always)]
    fn to_simd(&self) -> f32x4 {
        f32x4::new([self.x, self.y, self.z, 0.0])
    }

    // Internal helper to create Vector3d from SIMD
    #[inline(always)]
    fn from_simd(simd: f32x4) -> Self {
        let arr = simd.to_array();
        Self {
            x: arr[0],
            y: arr[1],
            z: arr[2],
        }
    }

    // A vector of 3d must have basic arithmetic calculations to represent it's location on the environment
    pub fn add(&self, other: &Vector3d) -> Vector3d {
        Self::from_simd(self.to_simd() + other.to_simd())
    }

    pub fn subtract(&self, other: &Vector3d) -> Vector3d {
        Self::from_simd(self.to_simd() - other.to_simd())
    }

    // Scalar multiplication has the purpose to control vector speed without changing its direction
    pub fn scale(&self, scalar: f32) -> Vector3d {
        let scalar_simd = f32x4::splat(scalar);
        Self::from_simd(self.to_simd() * scalar_simd)
    }

    // This represents the length or size of the vector
    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).d_sqrt()
    }

    // It points the direction of the vector without the magnitude
    pub fn normalize(&self) -> Vector3d {
        let mag = self.magnitude();
        if mag == 0.0 {
            *self // Return original
        } else {
            self.scale(1.0 / mag)
        }
    }

    // Look at where the vector is pointing at and it's relative direction compared to other vectors
    pub fn dot(&self, other: &Vector3d) -> f32 {
        let mul = self.to_simd() * other.to_simd();
        let arr = mul.to_array();
        arr[0] + arr[1] + arr[2]
    }

    // This can be used to calculate many things like perpendicular directions,
    // surface normals, torque, rotational force and orthogonal basis vectors.
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
