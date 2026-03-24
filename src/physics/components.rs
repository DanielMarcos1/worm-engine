use crate::geometry::{vector::Vector3d, polygon::Polygon};

#[derive(Debug, Default)]
pub struct RigidBodyComponents {
    pub shapes: Vec<Polygon>,
    pub masses: Vec<f32>,
    pub velocities: Vec<Vector3d>,
    pub accelerations: Vec<Vector3d>,
    pub forces: Vec<Vector3d>,
}

impl RigidBodyComponents {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, shape: Polygon, mass: f32) {
        assert!(mass > 0.0, "Mass must be greater than zero");
        self.shapes.push(shape);
        self.masses.push(mass);
        self.velocities.push(Vector3d::zero());
        self.accelerations.push(Vector3d::zero());
        self.forces.push(Vector3d::zero());
    }

    pub fn len(&self) -> usize {
        self.masses.len()
    }

    pub fn is_empty(&self) -> bool {
        self.masses.is_empty()
    }
}
