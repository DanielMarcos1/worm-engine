use crate::{geometry::vector::Vector3d, physics::{constants::GRAVITY, components::RigidBodyComponents}};

pub fn apply_force(components: &mut RigidBodyComponents, index: usize, force: Vector3d) {
    components.forces_x[index] += force.x;
    components.forces_y[index] += force.y;
    components.forces_z[index] += force.z;
}

pub fn apply_gravity(components: &mut RigidBodyComponents, index: usize) {
    let force = GRAVITY.scale(components.masses[index]);
    components.forces_x[index] += force.x;
    components.forces_y[index] += force.y;
    components.forces_z[index] += force.z;
}

pub fn update(components: &mut RigidBodyComponents, index: usize, dt: f32) {
    let mass = components.masses[index];
    let force_x = components.forces_x[index];
    let force_y = components.forces_y[index];
    let force_z = components.forces_z[index];

    let accel_x = force_x / mass;
    let accel_y = force_y / mass;
    let accel_z = force_z / mass;

    components.accelerations_x[index] = accel_x;
    components.accelerations_y[index] = accel_y;
    components.accelerations_z[index] = accel_z;

    let vel_x = components.velocities_x[index] + accel_x * dt;
    let vel_y = components.velocities_y[index] + accel_y * dt;
    let vel_z = components.velocities_z[index] + accel_z * dt;

    components.velocities_x[index] = vel_x;
    components.velocities_y[index] = vel_y;
    components.velocities_z[index] = vel_z;

    let vel = Vector3d::new(vel_x, vel_y, vel_z);
    for vertex in &mut components.shapes[index].vertices {
        *vertex = vertex.add(&vel.scale(dt));
    }

    components.forces_x[index] = 0.0;
    components.forces_y[index] = 0.0;
    components.forces_z[index] = 0.0;
}
