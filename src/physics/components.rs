use crate::geometry::{vector::Vector3d, polygon::Polygon};

#[derive(Default)]
pub struct RigidBodyComponents {
    pub shapes: Vec<Polygon>,
    pub masses: Vec<f32>,
    pub inverse_masses: Vec<f32>,
    pub velocities: Vec<Vector3d>,
    pub accelerations: Vec<Vector3d>,
    pub forces: Vec<Vector3d>,
}

impl RigidBodyComponents {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, shape: Polygon, mass: f32) -> usize {
        assert!(mass > 0.0, "Mass must be greater than zero");

        let id = self.masses.len();
        self.shapes.push(shape);
        self.masses.push(mass);
        self.inverse_masses.push(1.0 / mass);
        self.velocities.push(Vector3d::zero());
        self.accelerations.push(Vector3d::zero());
        self.forces.push(Vector3d::zero());

        id
    }
}
