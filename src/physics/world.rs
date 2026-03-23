use crate::physics::{components::RigidBodyComponents, constants::GRAVITY};
use rayon::prelude::*;

pub struct World {
    pub rigid_bodies: RigidBodyComponents,
    pub time_step: f32,
}

impl World {
    pub fn new(time_step: f32) -> Self {
        Self {
            rigid_bodies: RigidBodyComponents::new(),
            time_step,
        }
    }

    pub fn add_body(&mut self, descriptor: crate::physics::rigid_body::RigidBodyDescriptor) -> usize {
        self.rigid_bodies.add(descriptor.shape, descriptor.mass)
    }

    pub async fn step_gpu(&mut self) {
        let dt = self.time_step;
        let count = self.rigid_bodies.masses.len();

        // 1. Apply gravity and compute initial accelerations on CPU
        for i in 0..count {
            let gravity_force = GRAVITY.scale(self.rigid_bodies.masses[i]);
            self.rigid_bodies.forces[i] = self.rigid_bodies.forces[i].add(&gravity_force);
            self.rigid_bodies.accelerations[i] = self.rigid_bodies.forces[i].scale(self.rigid_bodies.inverse_masses[i]);
        }

        // 2. Offload velocity updates to the GPU
        crate::physics::gpu::run_compute_shader(
            &mut self.rigid_bodies.velocities,
            &self.rigid_bodies.accelerations,
            dt
        ).await;

        // 3. Update Positions (Vertices) on CPU based on new velocities
        for i in 0..count {
            let velocity = self.rigid_bodies.velocities[i];
            for vertex in &mut self.rigid_bodies.shapes[i].vertices {
                *vertex = vertex.add(&velocity.scale(dt));
            }
            self.rigid_bodies.forces[i] = crate::geometry::vector::Vector3d::zero();
        }
    }

    pub fn step(&mut self) {
        let dt = self.time_step;
        let count = self.rigid_bodies.masses.len();

        // Multithreaded update using rayon and wide SIMD.
        // For simplicity and correctness with SoA, we use `par_iter_mut` with zip.
        // In the future, this loop can be further chunked for f32x4 wide operations.

        // 1. Apply gravity and update properties in parallel using Rayon
        self.rigid_bodies.forces.par_iter_mut()
            .zip(self.rigid_bodies.accelerations.par_iter_mut())
            .zip(self.rigid_bodies.velocities.par_iter_mut())
            .zip(self.rigid_bodies.shapes.par_iter_mut())
            .zip(self.rigid_bodies.masses.par_iter())
            .zip(self.rigid_bodies.inverse_masses.par_iter())
            .for_each(|(((((force, acceleration), velocity), shape), mass), inv_mass)| {
                // Apply Gravity
                let gravity_force = GRAVITY.scale(*mass);
                *force = force.add(&gravity_force);

                // Update Accelerations
                *acceleration = force.scale(*inv_mass);

                // Update Velocities
                *velocity = velocity.add(&acceleration.scale(dt));

                // Update Positions (Vertices)
                let current_vel = *velocity;
                for vertex in &mut shape.vertices {
                    *vertex = vertex.add(&current_vel.scale(dt));
                }

                // Reset force
                *force = crate::geometry::vector::Vector3d::zero();
            });
    }
}