use crate::{geometry::vector::Vector3d, physics::constants::GRAVITY, physics::world::World};

pub fn apply_force(world: &mut World, index: usize, force: Vector3d) {
    world.forces[index] = world.forces[index].add(&force);
}

pub fn apply_gravity(world: &mut World, index: usize) {
    let force = GRAVITY.scale(world.masses[index]);
    world.forces[index] = world.forces[index].add(&force);
}

pub fn update(world: &mut World, index: usize, dt: f32) {
    let mass = world.masses[index];
    let force = world.forces[index];

    let accel = force.scale(1.0 / mass);
    world.accelerations[index] = accel;

    let mut vel = world.velocities[index];
    vel = vel.add(&accel.scale(dt));
    world.velocities[index] = vel;

    for vertex in &mut world.shapes[index].vertices {
        *vertex = vertex.add(&vel.scale(dt));
    }

    world.forces[index] = Vector3d::zero();
}
