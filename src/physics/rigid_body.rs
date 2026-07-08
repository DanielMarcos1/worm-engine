use crate::{geometry::vector::Vector3d, physics::{constants::GRAVITY, components::RigidBodyComponents}};

pub fn apply_force(components: &mut RigidBodyComponents, index: usize, force: Vector3d) {
    components.forces_x[index] += force.x;
    components.forces_y[index] += force.y;
    components.forces_z[index] += force.z;
}

pub fn apply_gravity(components: &mut RigidBodyComponents, index: usize) {
    let mass = components.masses[index];
    components.forces_x[index] += GRAVITY.x * mass;
    components.forces_y[index] += GRAVITY.y * mass;
    components.forces_z[index] += GRAVITY.z * mass;
}

pub fn update(components: &mut RigidBodyComponents, index: usize, dt: f32) {
    let mass = components.masses[index];
    let inv_mass = 1.0 / mass;

    let fx = components.forces_x[index];
    let fy = components.forces_y[index];
    let fz = components.forces_z[index];

    let ax = fx * inv_mass;
    let ay = fy * inv_mass;
    let az = fz * inv_mass;

    components.accelerations_x[index] = ax;
    components.accelerations_y[index] = ay;
    components.accelerations_z[index] = az;

    let mut vx = components.velocities_x[index];
    let mut vy = components.velocities_y[index];
    let mut vz = components.velocities_z[index];

    vx += ax * dt;
    vy += ay * dt;
    vz += az * dt;

    components.velocities_x[index] = vx;
    components.velocities_y[index] = vy;
    components.velocities_z[index] = vz;

    let vel = Vector3d::new(vx, vy, vz);

    for vertex in &mut components.shapes[index].vertices {
        *vertex = vertex.add(&vel.scale(dt));
    }

    components.forces_x[index] = 0.0;
    components.forces_y[index] = 0.0;
    components.forces_z[index] = 0.0;
}
