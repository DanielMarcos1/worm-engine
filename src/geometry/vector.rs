#[derive(Debug, Clone, Copy)]
pub struct Vector3d {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3d {

    // Constructor of the trait
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {x, y, z}
    }

    pub fn zero() -> Self {
        Self {x: 0.0, y: 0.0, z: 0.0}
    }

    // A vector of 3d must have basic arithmetic calculations to represent it's location on the environment
    pub fn add(&self, other: &Vector3d) -> Vector3d {
        Vector3d {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn subtract(&self, other: &Vector3d) -> Vector3d {
        Vector3d {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    // Scalar multiplication has the purpose to control vector speed without changing its direction
    pub fn scale(&self, scalar: f32) -> Vector3d {
        Vector3d {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    // This represents the length or size of the vector
    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    // It points the direction of the vector without the magnitude
    pub fn normalize(&self) -> Vector3d {
        let mag = self.magnitude();
        if mag == 0.0 {
            *self // Return original
        } else {
            Vector3d {
                x: self.x / mag,
                y: self.y / mag,
                z: self.z / mag,
            }
        }
    }

    // Look at where the vector is pointing at and it's relative direction compared to other vectors
    pub fn dot(&self, other: &Vector3d) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
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
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn project_onto(&self, other: &Vector3d) -> Vector3d {
        let scalar = self.dot(other) / other.length_squared();
        other.scale(scalar)
    }

    pub fn angle_between(&self, other: &Vector3d) -> f32 {
        let dot = self.dot(other);
        let magnitudes = self.magnitude() * other.magnitude();
        (dot / magnitudes).acos()
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