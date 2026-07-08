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

    let ax = components.forces_x[index] / mass;
    let ay = components.forces_y[index] / mass;
    let az = components.forces_z[index] / mass;

    components.accelerations_x[index] = ax;
    components.accelerations_y[index] = ay;
    components.accelerations_z[index] = az;

    components.velocities_x[index] += ax * dt;
    components.velocities_y[index] += ay * dt;
    components.velocities_z[index] += az * dt;

    let vel = Vector3d::new(
        components.velocities_x[index],
        components.velocities_y[index],
        components.velocities_z[index],
    );

    for vertex in &mut components.shapes[index].vertices {
        *vertex = vertex.add(&vel.scale(dt));
    }

    components.forces_x[index] = 0.0;
    components.forces_y[index] = 0.0;
    components.forces_z[index] = 0.0;
}
