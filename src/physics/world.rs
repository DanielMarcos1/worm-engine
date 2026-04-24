use crate::{geometry::polygon::Polygon, physics::components::RigidBodyComponents};
use rayon::prelude::*;
use wide::f32x4;
use crate::geometry::vector::Vector3d;
use crate::physics::ccd::calculate_toi_sphere_sphere;

pub struct World {
    pub bodies: RigidBodyComponents,
    pub time_step: f32,
    pub next_entity: usize,

    // SoA (Struct of Arrays) layout for DOD
    pub positions: Vec<Vector3d>,
    pub velocities: Vec<Vector3d>,
    pub accelerations: Vec<Vector3d>,
    pub forces: Vec<Vector3d>,
    pub masses: Vec<f32>,
    pub shapes: Vec<Polygon>,
    pub active_entities: Vec<bool>, // true if entity is active
}

impl World {
    pub fn new(time_step: f32) -> Self {
        Self {
            bodies: RigidBodyComponents::new(),
            time_step,
            next_entity: 0,
            positions: Vec::new(),
            velocities: Vec::new(),
            accelerations: Vec::new(),
            forces: Vec::new(),
            masses: Vec::new(),
            shapes: Vec::new(),
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
            .for_each(|((((((((((shapes, masses), v_x_chunks), v_y_chunks), v_z_chunks), a_x_chunks), a_y_chunks), a_z_chunks), f_x_chunks), f_y_chunks), f_z_chunks)| {

                let mass_simd = f32x4::from([masses[0], masses[1], masses[2], masses[3]]);
                let inv_mass = f32x4::splat(1.0) / mass_simd;

                // Load forces
                let mut f_x = f32x4::from([f_x_chunks[0], f_x_chunks[1], f_x_chunks[2], f_x_chunks[3]]);
                let mut f_y = f32x4::from([f_y_chunks[0], f_y_chunks[1], f_y_chunks[2], f_y_chunks[3]]);
                let mut f_z = f32x4::from([f_z_chunks[0], f_z_chunks[1], f_z_chunks[2], f_z_chunks[3]]);

                // Apply gravity (F = F + mg)
                f_x = f_x + (grav_x * mass_simd);
                f_y = f_y + (grav_y * mass_simd);
                f_z = f_z + (grav_z * mass_simd);

                // Calculate acceleration (a = F / m)
                let a_x = f_x * inv_mass;
                let a_y = f_y * inv_mass;
                let a_z = f_z * inv_mass;

                // Load velocities
                let mut v_x = f32x4::from([v_x_chunks[0], v_x_chunks[1], v_x_chunks[2], v_x_chunks[3]]);
                let mut v_y = f32x4::from([v_y_chunks[0], v_y_chunks[1], v_y_chunks[2], v_y_chunks[3]]);
                let mut v_z = f32x4::from([v_z_chunks[0], v_z_chunks[1], v_z_chunks[2], v_z_chunks[3]]);

                // Update velocities (v = v + a * dt)
                v_x = v_x + (a_x * dt_simd);
                v_y = v_y + (a_y * dt_simd);
                v_z = v_z + (a_z * dt_simd);

                // Store back
                let a_x_arr: [f32; 4] = a_x.into();
                let a_y_arr: [f32; 4] = a_y.into();
                let a_z_arr: [f32; 4] = a_z.into();

                let v_x_arr: [f32; 4] = v_x.into();
                let v_y_arr: [f32; 4] = v_y.into();
                let v_z_arr: [f32; 4] = v_z.into();

                for i in 0..4 {
                    a_x_chunks[i] = a_x_arr[i];
                    a_y_chunks[i] = a_y_arr[i];
                    a_z_chunks[i] = a_z_arr[i];

                    v_x_chunks[i] = v_x_arr[i];
                    v_y_chunks[i] = v_y_arr[i];
                    v_z_chunks[i] = v_z_arr[i];

                    f_x_chunks[i] = 0.0;
                    f_y_chunks[i] = 0.0;
                    f_z_chunks[i] = 0.0;

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
                let gravity_force = crate::physics::constants::GRAVITY.scale(mass);
                let current_force = Vector3d::new(self.bodies.forces_x[i], self.bodies.forces_y[i], self.bodies.forces_z[i]);
                let force = current_force.add(&gravity_force);

                let accel = force.scale(1.0 / mass);
                self.bodies.accelerations_x[i] = accel.x;
                self.bodies.accelerations_y[i] = accel.y;
                self.bodies.accelerations_z[i] = accel.z;

                let mut velocity = Vector3d::new(self.bodies.velocities_x[i], self.bodies.velocities_y[i], self.bodies.velocities_z[i]);
                velocity = velocity.add(&accel.scale(dt));

                self.bodies.velocities_x[i] = velocity.x;
                self.bodies.velocities_y[i] = velocity.y;
                self.bodies.velocities_z[i] = velocity.z;

                for vertex in &mut self.bodies.shapes[i].vertices {
                    *vertex = vertex.add(&velocity.scale(dt));
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
