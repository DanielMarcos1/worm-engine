// This file is kept for backwards compatibility or high-level entity management
// but data is now stored in `World`'s `RigidBodyComponents` using SoA layout.

use crate::geometry::polygon::Polygon;

#[derive(Debug, Clone)]
pub struct RigidBodyDescriptor {
    pub shape: Polygon,
    pub mass: f32,
}

impl RigidBodyDescriptor {
    pub fn new(shape: Polygon, mass: f32) -> Self {
        assert!(mass > 0.0, "Mass must be greater than zero");
        Self { shape, mass }
    }
}
