use crate::geometry::polygon::Polygon;

#[derive(Debug, Default)]
pub struct RigidBodyComponents {
    pub shapes: Vec<Polygon>,
    pub masses: Vec<f32>,

    pub velocities_x: Vec<f32>,
    pub velocities_y: Vec<f32>,
    pub velocities_z: Vec<f32>,

    pub accelerations_x: Vec<f32>,
    pub accelerations_y: Vec<f32>,
    pub accelerations_z: Vec<f32>,

    pub forces_x: Vec<f32>,
    pub forces_y: Vec<f32>,
    pub forces_z: Vec<f32>,
}

impl RigidBodyComponents {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, shape: Polygon, mass: f32) {
        assert!(mass > 0.0, "Mass must be greater than zero");
        self.shapes.push(shape);
        self.masses.push(mass);

        self.velocities_x.push(0.0);
        self.velocities_y.push(0.0);
        self.velocities_z.push(0.0);

        self.accelerations_x.push(0.0);
        self.accelerations_y.push(0.0);
        self.accelerations_z.push(0.0);

        self.forces_x.push(0.0);
        self.forces_y.push(0.0);
        self.forces_z.push(0.0);
    }

    pub fn len(&self) -> usize {
        self.masses.len()
    }

    pub fn is_empty(&self) -> bool {
        self.masses.is_empty()
    }
}
