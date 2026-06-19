use crate::{geometry::vector::Vector3d, physics::{constants::GRAVITY, components::RigidBodyComponents}};

pub fn apply_force(components: &mut RigidBodyComponents, index: usize, force: Vector3d) {
    components.forces[index] = components.forces[index].add(&force);
}

pub fn apply_gravity(components: &mut RigidBodyComponents, index: usize) {
    let force = GRAVITY.scale(components.masses[index]);
    components.forces[index] = components.forces[index].add(&force);
}

pub fn update(components: &mut RigidBodyComponents, index: usize, dt: f32) {
    let mass = components.masses[index];
    let force = components.forces[index];

    let accel = force.scale(1.0 / mass);
    components.accelerations[index] = accel;

    let mut vel = components.velocities[index];
    vel = vel.add(&accel.scale(dt));
    components.velocities[index] = vel;

    for vertex in &mut components.shapes[index].vertices {
        *vertex = vertex.add(&vel.scale(dt));
    }

    components.forces[index] = Vector3d::zero();
}
