use crate::geometry::vector::Vector3d;
use crate::physics::rigid_body::{RigidBody, Shape};
use crate::physics::integrator::Integrator;
use crate::physics::aabb_collision::AABB;
use crate::physics::collision_resolution::resolve_collision;
use crate::physics::constraint::Constraint;
use crate::physics::soft_body::SoftBody;

pub struct PhysicsWorld {
    pub bodies: Vec<RigidBody>,
    pub soft_bodies: Vec<SoftBody>,
    pub constraints: Vec<Box<dyn Constraint>>,
    pub integrator: Box<dyn Integrator>,
    pub fixed_time_step: f32,
    pub accumulator: f32,
    pub gravity: Vector3d,
}

impl PhysicsWorld {
    pub fn new(integrator: Box<dyn Integrator>, fixed_time_step: f32) -> Self {
        Self {
            bodies: Vec::new(),
            soft_bodies: Vec::new(),
            constraints: Vec::new(),
            integrator,
            fixed_time_step,
            accumulator: 0.0,
            gravity: Vector3d::new(0.0, -9.81, 0.0), // Default gravity
        }
    }

    pub fn add_constraint(&mut self, constraint: Box<dyn Constraint>) {
        self.constraints.push(constraint);
    }

    pub fn add_body(&mut self, body: RigidBody) {
        self.bodies.push(body);
    }

    pub fn add_soft_body(&mut self, soft_body: SoftBody) {
        self.soft_bodies.push(soft_body);
    }

    pub fn step(&mut self, dt: f32) {
        self.accumulator += dt;

        while self.accumulator >= self.fixed_time_step {
            // 1. Apply Constraint Forces (Once per step)
            for constraint in &self.constraints {
                constraint.apply_forces(&mut self.bodies, self.fixed_time_step);
            }

            // Also for SoftBodies
            for soft_body in &mut self.soft_bodies {
                soft_body.apply_forces(self.fixed_time_step);
            }

            // 2. Integration (Update velocities and positions)
            for body in &mut self.bodies {
                self.integrator.integrate(body, self.fixed_time_step, &self.gravity);
            }

            for soft_body in &mut self.soft_bodies {
                for node in &mut soft_body.nodes {
                    self.integrator.integrate(node, self.fixed_time_step, &self.gravity);
                }
            }

            // 3. Collision Detection and Resolution
            let potential_collisions = self.broad_phase();
            self.narrow_phase(&potential_collisions);

            // 4. Positional Constraint Solving (Iterative)
            for _ in 0..10 {
                for constraint in &self.constraints {
                    constraint.solve(&mut self.bodies, self.fixed_time_step);
                }

                for soft_body in &mut self.soft_bodies {
                    soft_body.solve_constraints(self.fixed_time_step);
                }
            }

            self.accumulator -= self.fixed_time_step;
        }
    }

    pub fn broad_phase(&mut self) -> Vec<(usize, usize)> {
        let mut potential_collisions = Vec::new();

        // Calculate AABBs for all bodies
        let aabbs: Vec<AABB> = self.bodies.iter().map(|body| {
            match body.shape {
                Shape::Circle(radius) => {
                    AABB::from_center_and_size(
                        body.position,
                        Vector3d::new(radius * 2.0, radius * 2.0, radius * 2.0),
                    )
                },
                Shape::AABB(half_extents) => {
                    AABB::from_center_and_size(
                        body.position,
                        half_extents.scale(2.0),
                    )
                }
            }
        }).collect();

        // O(N^2) naive check - could be optimized with Sweep and Prune or Spatial Partitioning
        for i in 0..aabbs.len() {
            for j in (i + 1)..aabbs.len() {
                if aabbs[i].collides_with(&aabbs[j]) {
                    potential_collisions.push((i, j));
                }
            }
        }

        potential_collisions
    }

    pub fn narrow_phase(&mut self, potential_collisions: &[(usize, usize)]) {
        for &(i, j) in potential_collisions {
            let body1 = &self.bodies[i];
            let body2 = &self.bodies[j];

            // Temporary way to extract data needed for narrow phase, to get around mutability borrow checks
            let shape1 = body1.shape.clone();
            let shape2 = body2.shape.clone();
            let pos1 = body1.position;
            let pos2 = body2.position;

            let collision_data = match (&shape1, &shape2) {
                (Shape::Circle(r1), Shape::Circle(r2)) => {
                    let distance_squared = pos1.distance_to(&pos2).powi(2);
                    let radius_sum = r1 + r2;
                    if distance_squared <= radius_sum.powi(2) {
                        let distance = distance_squared.sqrt();
                        let normal = if distance == 0.0 {
                            Vector3d::new(1.0, 0.0, 0.0) // Arbitrary normal if same position
                        } else {
                            pos2.subtract(&pos1).normalize()
                        };
                        Some(normal)
                    } else {
                        None
                    }
                },
                (Shape::AABB(he1), Shape::AABB(he2)) => {
                    // SAT or simpler AABB vs AABB overlap logic returning penetration vector
                    let aabb1 = AABB::from_center_and_size(pos1, he1.scale(2.0));
                    let aabb2 = AABB::from_center_and_size(pos2, he2.scale(2.0));
                    if aabb1.collides_with(&aabb2) {
                        // Calculate a simplistic resolution normal for demonstration
                        Some(pos2.subtract(&pos1).normalize())
                    } else {
                        None
                    }
                },
                (Shape::Circle(r), Shape::AABB(he)) => {
                    let aabb = AABB::from_center_and_size(pos2, he.scale(2.0));
                    if aabb.intersects_circle(&pos1, *r) {
                        Some(pos1.subtract(&pos2).normalize())
                    } else {
                        None
                    }
                },
                (Shape::AABB(he), Shape::Circle(r)) => {
                    let aabb = AABB::from_center_and_size(pos1, he.scale(2.0));
                    if aabb.intersects_circle(&pos2, *r) {
                        Some(pos2.subtract(&pos1).normalize())
                    } else {
                        None
                    }
                }
            };

            if let Some(normal) = collision_data {
                // To resolve collision we need to borrow mutably
                // We split the borrow to get two mut references
                if i < j {
                    let (left, right) = self.bodies.split_at_mut(j);
                    let b1 = &mut left[i];
                    let b2 = &mut right[0];
                    resolve_collision(b1, b2, normal, 0.5, 0.5); // Default restitution and friction for now
                } else {
                    let (left, right) = self.bodies.split_at_mut(i);
                    let b2 = &mut left[j];
                    let b1 = &mut right[0];
                    resolve_collision(b1, b2, normal, 0.5, 0.5);
                }
            }
        }
    }
}