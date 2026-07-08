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
    let f_x = components.forces_x[index];
    let f_y = components.forces_y[index];
    let f_z = components.forces_z[index];

    let inv_mass = 1.0 / mass;
    let a_x = f_x * inv_mass;
    let a_y = f_y * inv_mass;
    let a_z = f_z * inv_mass;

    components.accelerations_x[index] = a_x;
    components.accelerations_y[index] = a_y;
    components.accelerations_z[index] = a_z;

    let mut v_x = components.velocities_x[index];
    let mut v_y = components.velocities_y[index];
    let mut v_z = components.velocities_z[index];

    v_x += a_x * dt;
    v_y += a_y * dt;
    v_z += a_z * dt;

    components.velocities_x[index] = v_x;
    components.velocities_y[index] = v_y;
    components.velocities_z[index] = v_z;

    for vertex in &mut components.shapes[index].vertices {
        vertex.x += v_x * dt;
        vertex.y += v_y * dt;
        vertex.z += v_z * dt;
    }

    components.forces_x[index] = 0.0;
    components.forces_y[index] = 0.0;
    components.forces_z[index] = 0.0;
}
