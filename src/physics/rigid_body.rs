use crate::{geometry::{vector::Vector3d, polygon::Polygon}, physics::constants::GRAVITY};

pub fn apply_force(forces: &mut [Vector3d], index: usize, force: Vector3d) {
    forces[index] = forces[index].add(&force);
}

pub fn apply_gravity(forces: &mut [Vector3d], masses: &[f32], index: usize) {
    let force = GRAVITY.scale(masses[index]);
    forces[index] = forces[index].add(&force);
}

pub fn update(
    shapes: &mut [Polygon],
    masses: &[f32],
    velocities: &mut [Vector3d],
    accelerations: &mut [Vector3d],
    forces: &mut [Vector3d],
    index: usize,
    dt: f32,
) {
    let mass = masses[index];
    let force = forces[index];

    let accel = force.scale(1.0 / mass);
    accelerations[index] = accel;

    let mut vel = velocities[index];
    vel = vel.add(&accel.scale(dt));
    velocities[index] = vel;

    for vertex in &mut shapes[index].vertices {
        *vertex = vertex.add(&vel.scale(dt));
    }

    forces[index] = Vector3d::zero();
}
