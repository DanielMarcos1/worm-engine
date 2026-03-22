use crate::{geometry::vector::Vector3d, physics::rigid_body::RigidBody};
use rayon::prelude::*;

pub struct World {
    pub bodies: Vec<RigidBody>,
    pub time_step: f32,
}

impl World {
    pub fn new(time_step: f32) -> Self {
        Self {
            bodies: Vec::new(),
            time_step,
        }
    }

    pub fn add_body(&mut self, body: RigidBody) {
        self.bodies.push(body);
    }

    pub fn step(&mut self) {
        // Implement rayon parallel iterators for physics step resolution (Task 3.2)
        let dt = self.time_step;
        self.bodies.par_iter_mut().for_each(|body| {
            body.apply_gravity();
            body.update(dt);
        });
    }
}