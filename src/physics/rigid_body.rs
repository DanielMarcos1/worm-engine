use crate::{geometry::{vector::Vector3d, polygon::Polygon}, physics::constants::GRAVITY};

#[derive(Debug)]
pub struct RigidBody {
    pub shape: Polygon,
    pub mass: f32,
    pub velocity: Vector3d,
    pub acceleration: Vector3d,
    pub forces: Vector3d,
}

impl RigidBody {
    
    pub fn new(shape: Polygon, mass: f32) -> RigidBody {
        assert!(mass > 0.0, "Mass must be greater than zero");
        RigidBody {
            shape,
            mass,
            velocity: Vector3d::zero(),
            acceleration: Vector3d::zero(),
            forces: Vector3d::zero(),
        }
    }

    pub fn apply_force(&mut self, force: Vector3d) {
        self.forces = self.forces.add(&force)
    }

    pub fn apply_gravity(&mut self) {
        self.forces = self.forces.add(&GRAVITY.scale(self.mass));
    }

    pub fn update(&mut self, dt: f32) {

        self.acceleration = self.forces.scale(1.0 / self.mass);

        self.velocity = self.velocity.add(&self.acceleration.scale(dt));

        for vertex in &mut self.shape.vertices {
            *vertex = vertex.add(&self.velocity.scale(dt));
        }

        self.forces = Vector3d::zero();
    }
}