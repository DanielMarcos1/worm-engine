use crate::{geometry::vector::Vector3d, physics::rigid_body::RigidBody};


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
        for body in self.bodies.iter_mut() {
            body.apply_gravity();
            body.update(self.time_step);
        }
    }


}