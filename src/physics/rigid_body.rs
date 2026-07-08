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
    let force = Vector3d::new(components.forces_x[index], components.forces_y[index], components.forces_z[index]);

    let accel = force.scale(1.0 / mass);
    components.accelerations_x[index] = accel.x;
    components.accelerations_y[index] = accel.y;
    components.accelerations_z[index] = accel.z;

    let vel = Vector3d::new(components.velocities_x[index], components.velocities_y[index], components.velocities_z[index]);
    let new_vel = vel.add(&accel.scale(dt));
    components.velocities_x[index] = new_vel.x;
    components.velocities_y[index] = new_vel.y;
    components.velocities_z[index] = new_vel.z;

    for vertex in &mut components.shapes[index].vertices {
        *vertex = vertex.add(&new_vel.scale(dt));
    }

    components.forces_x[index] = 0.0;
    components.forces_y[index] = 0.0;
    components.forces_z[index] = 0.0;
}
