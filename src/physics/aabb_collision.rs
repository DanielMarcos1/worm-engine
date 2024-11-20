use crate::vector::Vector3d;

#[derive(Debug)]
pub struct AABB {
    pub min: Vector3d,
    pub max: Vector3d,
}

impl AABB {

    pub fn new(min: Vector3d, max: Vector3d) -> Self {
        Self { min, max }
    }

    pub fn collides_with(&self, other: &AABB) -> bool {
        // Check overlap
        self.min.x <= other.max.x && self.max.x >= other.min.x &&
        self.min.y <= other.max.y && self.max.y >= other.min.y &&
        self.min.z <= other.max.z && self.max.z >= other.min.z
    }

    /// Creates an AABB given a center and size.
    /// The size is split evenly in all directions from the center to determine the min and max.
    pub fn from_center_and_size(center: Vector3d, size: Vector3d) -> Self {
        assert!(size.x >= 0.0 && size.y >= 0.0 && size.z >= 0.0, "Size components must be non-negative");
        let half_size = size.scale(0.5); // Assuming scale is implemented for Vector3d
        Self {
            min: center.subtract(half_size),
            max: center.add(half_size),
        }
    }
}