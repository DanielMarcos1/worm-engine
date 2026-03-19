use crate::physics::rigid_body::RigidBody;

pub trait Constraint {
    // Solves positional errors. Called multiple times per frame.
    fn solve(&self, bodies: &mut [RigidBody], dt: f32);

    // Calculates and applies forces. Called once per frame, before integration.
    fn apply_forces(&self, _bodies: &mut [RigidBody], _dt: f32) {}
}

pub struct DistanceJoint {
    pub body_a_index: usize,
    pub body_b_index: usize,
    pub target_distance: f32,
    pub stiffness: f32, // Allows for both rigid and soft distance joints
}

impl DistanceJoint {
    pub fn new(body_a_index: usize, body_b_index: usize, target_distance: f32, stiffness: f32) -> Self {
        Self {
            body_a_index,
            body_b_index,
            target_distance,
            stiffness,
        }
    }
}

impl Constraint for DistanceJoint {
    fn solve(&self, bodies: &mut [RigidBody], _dt: f32) {
        if self.body_a_index == self.body_b_index {
            return;
        }

        let body_a_pos = bodies[self.body_a_index].position;
        let body_b_pos = bodies[self.body_b_index].position;
        let mass_a_inv = bodies[self.body_a_index].inverse_mass;
        let mass_b_inv = bodies[self.body_b_index].inverse_mass;

        if mass_a_inv + mass_b_inv == 0.0 {
            return;
        }

        let direction = body_b_pos.subtract(&body_a_pos);
        let distance = direction.magnitude();

        if distance == 0.0 {
            return;
        }

        let normal = direction.normalize();
        let error = distance - self.target_distance;

        let correction_magnitude = error * self.stiffness / (mass_a_inv + mass_b_inv);
        let correction = normal.scale(correction_magnitude);

        bodies[self.body_a_index].position = bodies[self.body_a_index].position.add(&correction.scale(mass_a_inv));
        bodies[self.body_b_index].position = bodies[self.body_b_index].position.subtract(&correction.scale(mass_b_inv));
    }
}

pub struct SpringJoint {
    pub body_a_index: usize,
    pub body_b_index: usize,
    pub rest_length: f32,
    pub spring_constant: f32,
    pub damping: f32,
}

impl SpringJoint {
    pub fn new(body_a_index: usize, body_b_index: usize, rest_length: f32, spring_constant: f32, damping: f32) -> Self {
        Self {
            body_a_index,
            body_b_index,
            rest_length,
            spring_constant,
            damping,
        }
    }
}

impl Constraint for SpringJoint {
    fn solve(&self, _bodies: &mut [RigidBody], _dt: f32) {
        // Spring joints apply forces, not direct positional corrections
    }

    fn apply_forces(&self, bodies: &mut [RigidBody], _dt: f32) {
        if self.body_a_index == self.body_b_index {
            return;
        }

        let body_a_pos = bodies[self.body_a_index].position;
        let body_b_pos = bodies[self.body_b_index].position;
        let body_a_vel = bodies[self.body_a_index].velocity;
        let body_b_vel = bodies[self.body_b_index].velocity;

        let direction = body_b_pos.subtract(&body_a_pos);
        let distance = direction.magnitude();

        if distance == 0.0 {
            return;
        }

        let normal = direction.normalize();

        // Spring force: F = -k * x
        let displacement = distance - self.rest_length;
        let spring_force = self.spring_constant * displacement;

        // Damping force: F_d = -c * v_rel
        let rel_vel = body_b_vel.subtract(&body_a_vel);
        let vel_along_normal = rel_vel.dot(&normal);
        let damping_force = self.damping * vel_along_normal;

        let total_force_mag = spring_force + damping_force;
        let total_force = normal.scale(total_force_mag);

        // Apply forces
        bodies[self.body_a_index].apply_force(&total_force);
        bodies[self.body_b_index].apply_force(&total_force.scale(-1.0));
    }
}