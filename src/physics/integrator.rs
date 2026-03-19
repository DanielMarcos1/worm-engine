use crate::geometry::vector::Vector3d;
use crate::physics::rigid_body::RigidBody;

pub trait Integrator {
    fn integrate(&self, body: &mut RigidBody, dt: f32, gravity: &Vector3d);
}

pub struct Euler;

impl Integrator for Euler {
    fn integrate(&self, body: &mut RigidBody, dt: f32, gravity: &Vector3d) {
        if body.inverse_mass <= 0.0 {
            return;
        }

        let gravity_force = gravity.scale(body.mass * body.gravity_scale);
        let total_force = body.force_accumulator.add(&gravity_force);

        // a = F / m
        body.acceleration = total_force.scale(body.inverse_mass);

        // p = p + v * dt
        body.position = body.position.add(&body.velocity.scale(dt));

        // v = v + a * dt
        body.velocity = body.velocity.add(&body.acceleration.scale(dt));

        body.velocity = body.velocity.scale(body.damping.powf(dt));

        body.clear_forces();
    }
}

pub struct SemiImplicitEuler;

impl Integrator for SemiImplicitEuler {
    fn integrate(&self, body: &mut RigidBody, dt: f32, gravity: &Vector3d) {
        if body.inverse_mass <= 0.0 {
            return;
        }

        let gravity_force = gravity.scale(body.mass * body.gravity_scale);
        let total_force = body.force_accumulator.add(&gravity_force);

        // a = F / m
        body.acceleration = total_force.scale(body.inverse_mass);

        // v = v + a * dt
        body.velocity = body.velocity.add(&body.acceleration.scale(dt));

        body.velocity = body.velocity.scale(body.damping.powf(dt));

        // p = p + v * dt
        body.position = body.position.add(&body.velocity.scale(dt));

        body.clear_forces();
    }
}