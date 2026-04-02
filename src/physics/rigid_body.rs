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

    let a_x = components.forces_x[index] / mass;
    let a_y = components.forces_y[index] / mass;
    let a_z = components.forces_z[index] / mass;

    components.accelerations_x[index] = a_x;
    components.accelerations_y[index] = a_y;
    components.accelerations_z[index] = a_z;

    components.velocities_x[index] += a_x * dt;
    components.velocities_y[index] += a_y * dt;
    components.velocities_z[index] += a_z * dt;

    let vel = Vector3d::new(
        components.velocities_x[index],
        components.velocities_y[index],
        components.velocities_z[index]
    );

    for vertex in &mut components.shapes[index].vertices {
        *vertex = vertex.add(&vel.scale(dt));
    }

    components.forces_x[index] = 0.0;
    components.forces_y[index] = 0.0;
    components.forces_z[index] = 0.0;
}
