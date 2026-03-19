use crate::geometry::vector::Vector3d;
use crate::physics::rigid_body::{RigidBody, Shape};
use crate::physics::constraint::{Constraint, SpringJoint};

pub struct SoftBody {
    pub nodes: Vec<RigidBody>,
    pub springs: Vec<SpringJoint>,
}

impl SoftBody {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            springs: Vec::new(),
        }
    }

    pub fn add_node(&mut self, position: Vector3d, mass: f32) -> usize {
        let index = self.nodes.len();
        // Soft bodies often use small point masses (small circles)
        self.nodes.push(RigidBody::new(position, mass, Shape::Circle(0.1)));
        index
    }

    pub fn add_spring(&mut self, index_a: usize, index_b: usize, rest_length: f32, stiffness: f32, damping: f32) {
        self.springs.push(SpringJoint::new(index_a, index_b, rest_length, stiffness, damping));
    }

    pub fn apply_forces(&mut self, dt: f32) {
        for spring in &self.springs {
            spring.apply_forces(&mut self.nodes, dt);
        }
    }

    pub fn solve_constraints(&mut self, dt: f32) {
        for spring in &self.springs {
            spring.solve(&mut self.nodes, dt);
        }
    }
}