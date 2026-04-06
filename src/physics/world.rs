use crate::{geometry::polygon::Polygon, physics::components::RigidBodyComponents};
use rayon::prelude::*;
use wide::f32x4;
use crate::geometry::vector::Vector3d;
use crate::physics::ccd::calculate_toi_sphere_sphere;

pub struct World {
    pub bodies: RigidBodyComponents,
    pub time_step: f32,
    pub next_entity: usize,

    // SoA (Struct of Arrays) layout for DOD is handled by bodies
    pub active_entities: Vec<bool>, // true if entity is active
}

impl World {
    pub fn new(time_step: f32) -> Self {
        Self {
            bodies: RigidBodyComponents::new(),
            time_step,
            next_entity: 0,
            active_entities: Vec::new(),
        }
    }

    pub fn add_body(&mut self, shape: Polygon, mass: f32) {
        self.bodies.push(shape, mass);
    }

    pub fn step(&mut self) {
        let dt = self.time_step;
        let dt_simd = f32x4::splat(dt);
        let gravity = crate::physics::constants::GRAVITY;
        let grav_x = f32x4::splat(gravity.x);
        let grav_y = f32x4::splat(gravity.y);
        let grav_z = f32x4::splat(gravity.z);

        let chunk_size = 4;
        let remainder = self.bodies.len() % chunk_size;
        let exact_len = self.bodies.len() - remainder;

        // Process in chunks of 4 for SIMD vectorization
        self.bodies.shapes[..exact_len].par_chunks_mut(chunk_size)
            .zip(self.bodies.masses[..exact_len].par_chunks_mut(chunk_size))
            .zip(self.bodies.velocities_x[..exact_len].par_chunks_mut(chunk_size))
            .zip(self.bodies.velocities_y[..exact_len].par_chunks_mut(chunk_size))
            .zip(self.bodies.velocities_z[..exact_len].par_chunks_mut(chunk_size))
            .zip(self.bodies.accelerations_x[..exact_len].par_chunks_mut(chunk_size))
            .zip(self.bodies.accelerations_y[..exact_len].par_chunks_mut(chunk_size))
            .zip(self.bodies.accelerations_z[..exact_len].par_chunks_mut(chunk_size))
            .zip(self.bodies.forces_x[..exact_len].par_chunks_mut(chunk_size))
            .zip(self.bodies.forces_y[..exact_len].par_chunks_mut(chunk_size))
            .zip(self.bodies.forces_z[..exact_len].par_chunks_mut(chunk_size))
            .for_each(|((((((((((shapes, masses), v_x_arr), v_y_arr), v_z_arr), a_x_arr), a_y_arr), a_z_arr), f_x_arr), f_y_arr), f_z_arr)| {

                let mass_simd = f32x4::from([masses[0], masses[1], masses[2], masses[3]]);
                let inv_mass = f32x4::splat(1.0) / mass_simd;

                // Load forces
                let mut f_x = f32x4::from([f_x_arr[0], f_x_arr[1], f_x_arr[2], f_x_arr[3]]);
                let mut f_y = f32x4::from([f_y_arr[0], f_y_arr[1], f_y_arr[2], f_y_arr[3]]);
                let mut f_z = f32x4::from([f_z_arr[0], f_z_arr[1], f_z_arr[2], f_z_arr[3]]);

                // Apply gravity (F = F + mg)
                f_x = f_x + (grav_x * mass_simd);
                f_y = f_y + (grav_y * mass_simd);
                f_z = f_z + (grav_z * mass_simd);

                // Calculate acceleration (a = F / m)
                let a_x = f_x * inv_mass;
                let a_y = f_y * inv_mass;
                let a_z = f_z * inv_mass;

                // Load velocities
                let mut v_x = f32x4::from([v_x_arr[0], v_x_arr[1], v_x_arr[2], v_x_arr[3]]);
                let mut v_y = f32x4::from([v_y_arr[0], v_y_arr[1], v_y_arr[2], v_y_arr[3]]);
                let mut v_z = f32x4::from([v_z_arr[0], v_z_arr[1], v_z_arr[2], v_z_arr[3]]);

                // Update velocities (v = v + a * dt)
                v_x = v_x + (a_x * dt_simd);
                v_y = v_y + (a_y * dt_simd);
                v_z = v_z + (a_z * dt_simd);

                // Store back
                let a_x_res: [f32; 4] = a_x.into();
                let a_y_res: [f32; 4] = a_y.into();
                let a_z_res: [f32; 4] = a_z.into();

                let v_x_res: [f32; 4] = v_x.into();
                let v_y_res: [f32; 4] = v_y.into();
                let v_z_res: [f32; 4] = v_z.into();

                for i in 0..4 {
                    a_x_arr[i] = a_x_res[i];
                    a_y_arr[i] = a_y_res[i];
                    a_z_arr[i] = a_z_res[i];

                    v_x_arr[i] = v_x_res[i];
                    v_y_arr[i] = v_y_res[i];
                    v_z_arr[i] = v_z_res[i];

                    f_x_arr[i] = 0.0;
                    f_y_arr[i] = 0.0;
                    f_z_arr[i] = 0.0;

                    let vel = Vector3d::new(v_x_arr[i], v_y_arr[i], v_z_arr[i]);

                    // Update vertices
                    for vertex in &mut shapes[i].vertices {
                        *vertex = vertex.add(&vel.scale(dt));
                    }
                }
            });

        // Handle remainder sequentially
        if remainder > 0 {
            let start = exact_len;
            let end = self.bodies.len();

            for i in start..end {
                let mass = self.bodies.masses[i];
                let force_x = self.bodies.forces_x[i] + (gravity.x * mass);
                let force_y = self.bodies.forces_y[i] + (gravity.y * mass);
                let force_z = self.bodies.forces_z[i] + (gravity.z * mass);

                let accel_x = force_x / mass;
                let accel_y = force_y / mass;
                let accel_z = force_z / mass;

                self.bodies.accelerations_x[i] = accel_x;
                self.bodies.accelerations_y[i] = accel_y;
                self.bodies.accelerations_z[i] = accel_z;

                let vel_x = self.bodies.velocities_x[i] + (accel_x * dt);
                let vel_y = self.bodies.velocities_y[i] + (accel_y * dt);
                let vel_z = self.bodies.velocities_z[i] + (accel_z * dt);

                self.bodies.velocities_x[i] = vel_x;
                self.bodies.velocities_y[i] = vel_y;
                self.bodies.velocities_z[i] = vel_z;

                let vel = Vector3d::new(vel_x, vel_y, vel_z);

                for vertex in &mut self.bodies.shapes[i].vertices {
                    *vertex = vertex.add(&vel.scale(dt));
                }

                self.bodies.forces_x[i] = 0.0;
                self.bodies.forces_y[i] = 0.0;
                self.bodies.forces_z[i] = 0.0;
            }
        }

        // CCD phase (Basic n^2 implementation for spheres for demonstration, normally would use broadphase)
        let num_bodies = self.bodies.len();
        for i in 0..num_bodies {
            for j in (i + 1)..num_bodies {
                // To do exact sphere-sphere CCD we need a center and radius.
                // Since bodies use Polygons, we'll approximate using the first vertex as center and a fixed radius.
                // In a full implementation, the components would store a bounding sphere radius.
                if self.bodies.shapes[i].vertices.is_empty() || self.bodies.shapes[j].vertices.is_empty() { continue; }

                let p1 = self.bodies.shapes[i].vertices[0];
                let v1 = Vector3d::new(self.bodies.velocities_x[i], self.bodies.velocities_y[i], self.bodies.velocities_z[i]);
                let r1 = 1.0; // Approximation

                let p2 = self.bodies.shapes[j].vertices[0];
                let v2 = Vector3d::new(self.bodies.velocities_x[j], self.bodies.velocities_y[j], self.bodies.velocities_z[j]);
                let r2 = 1.0; // Approximation

                if let Some(toi) = calculate_toi_sphere_sphere(p1, v1, r1, p2, v2, r2, dt) {
                    // Stop the bodies exactly at the time of impact to prevent tunneling.
                    // This is a basic response - zero out velocities along the collision normal.
                    // A full solver would compute collision response at TOI.
                    let collision_normal = (p1.add(&v1.scale(toi * dt))).subtract(&p2.add(&v2.scale(toi * dt))).normalize();

                    let v1_proj = v1.dot(&collision_normal);
                    let v2_proj = v2.dot(&collision_normal);

                    if v1_proj < 0.0 {
                        let new_v1 = v1.subtract(&collision_normal.scale(v1_proj));
                        self.bodies.velocities_x[i] = new_v1.x;
                        self.bodies.velocities_y[i] = new_v1.y;
                        self.bodies.velocities_z[i] = new_v1.z;
                    }
                    if v2_proj > 0.0 {
                        let new_v2 = v2.subtract(&collision_normal.scale(v2_proj));
                        self.bodies.velocities_x[j] = new_v2.x;
                        self.bodies.velocities_y[j] = new_v2.y;
                        self.bodies.velocities_z[j] = new_v2.z;
                    }
                }
            }
        }
    }
}
