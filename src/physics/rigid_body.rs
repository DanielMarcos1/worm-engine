use crate::geometry::vector::Vector3d;

#[derive(Debug, Clone)]
pub enum Shape {
    Circle(f32), // radius
    AABB(Vector3d), // half_extents
}

#[derive(Debug, Clone)]
pub struct RigidBody {
    pub position: Vector3d,
    pub velocity: Vector3d,
    pub acceleration: Vector3d,
    pub mass: f32,
    pub inverse_mass: f32,
    pub force_accumulator: Vector3d,
    pub gravity_scale: f32,
    pub damping: f32, // Air resistance
    pub friction: f32,
    pub restitution: f32,
    pub shape: Shape,
}

impl RigidBody {
    pub fn new(position: Vector3d, mass: f32, shape: Shape) -> Self {
        let inverse_mass = if mass > 0.0 { 1.0 / mass } else { 0.0 };
        Self {
            position,
            velocity: Vector3d::new(0.0, 0.0, 0.0),
            acceleration: Vector3d::new(0.0, 0.0, 0.0),
            mass,
            inverse_mass,
            force_accumulator: Vector3d::new(0.0, 0.0, 0.0),
            gravity_scale: 1.0,
            damping: 0.98,
            friction: 0.5,
            restitution: 0.5,
            shape,
        }
    }

    pub fn apply_force(&mut self, force: &Vector3d) {
        self.force_accumulator = self.force_accumulator.add(force);
    }

    pub fn clear_forces(&mut self) {
        self.force_accumulator = Vector3d::new(0.0, 0.0, 0.0);
    }

    pub fn update(&mut self, dt: f32, gravity: &Vector3d) {
        if self.inverse_mass <= 0.0 {
            return; // Static body
        }

        // Add gravity to the force accumulator
        let gravity_force = gravity.scale(self.mass * self.gravity_scale);
        let total_force = self.force_accumulator.add(&gravity_force);

        // a = F / m
        self.acceleration = total_force.scale(self.inverse_mass);

        // v = v + a * dt
        self.velocity = self.velocity.add(&self.acceleration.scale(dt));

        // Apply damping (air resistance)
        self.velocity = self.velocity.scale(self.damping.powf(dt));

        // p = p + v * dt
        self.position = self.position.add(&self.velocity.scale(dt));

        // Clear forces after applying them
        self.clear_forces();
    }
}