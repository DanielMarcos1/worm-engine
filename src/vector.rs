#[derive(Debug, Clone, Copy)]
struct Vector3d {
    x: f32,
    y: f32,
    z: f32,
}

impl Vector3d {

    // Constructor of the trait
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self {x, y, z}
    }

    // A vector of 3d must have basic arithmetic calculations to represent it's location on the environment
    fn add(&self, other: Vector3d) -> Vector3d {
        Vector3d {
            x: &self.x + other.x,
            y: &self.y + other.y,
            z: &self.z + other.z,
        }
    }

    fn subtract(&self, other: Vector3d) -> Vector3d {
        Vector3d {
            x: &self.x + other.x,
            y: &self.y + other.y,
            z: &self.z + other.z,
        }
    }

    // Scalar multiplication has the purpose to control vector speed without changing its direction
    fn multiplication(&self, scalar: f32) -> Vector3d {
        Vector3d {
            x: &self.x * scalar,
            y: &self.y * scalar,
            z: &self.z * scalar,
        }
    }

    // This represents the length or size of the vector
    fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    // It points the direction of the vector without the magnitude
    fn normalize(&self) -> Vector3d {
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
    fn dot(&self, other: Vector3d) -> f32 {
        self.x * other.x + self.y * other.y + self.z + other.z
    }

    // This can be used to calculate many things like perpendicular directions,
    // surface normals, torque, rotational force and orthogonal basis vectors.
    fn cross(&self, other: Vector3d) -> Vector3d {
        Vector3d {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}