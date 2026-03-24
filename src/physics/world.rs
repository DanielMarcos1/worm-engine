use crate::{
    geometry::vector::Vector3d,
    physics::{
        components::{Acceleration, Entity, Force, Mass, Position, Shape, Velocity},
        constants::GRAVITY,
    },
};
use rayon::prelude::*;

pub struct World {
    pub time_step: f32,
    pub next_entity: usize,

    // SoA (Struct of Arrays) layout for DOD
    pub positions: Vec<Position>,
    pub velocities: Vec<Velocity>,
    pub accelerations: Vec<Acceleration>,
    pub forces: Vec<Force>,
    pub masses: Vec<Mass>,
    pub shapes: Vec<Shape>,
    pub active_entities: Vec<bool>, // true if entity is active
}

impl World {
    pub fn new(time_step: f32) -> Self {
        Self {
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

    pub fn add_body(&mut self, shape: Shape, mass: f32) -> Entity {
        let entity = Entity(self.next_entity);
        self.next_entity += 1;

        self.positions.push(Position(Vector3d::zero()));
        self.velocities.push(Velocity(Vector3d::zero()));
        self.accelerations.push(Acceleration(Vector3d::zero()));
        self.forces.push(Force(Vector3d::zero()));
        self.masses.push(Mass(mass));
        self.shapes.push(shape);
        self.active_entities.push(true);

        entity
    }

    pub fn apply_force(&mut self, entity: Entity, force: Vector3d) {
        if self.active_entities[entity.0] {
            self.forces[entity.0].0 = self.forces[entity.0].0.add(&force);
        }
    }

    pub fn step(&mut self) {
        let dt = self.time_step;

        // Apply gravity and update physics using Rayon for multithreading
        let zip_iter = self
            .active_entities
            .par_iter()
            .zip(self.positions.par_iter_mut())
            .zip(self.velocities.par_iter_mut())
            .zip(self.accelerations.par_iter_mut())
            .zip(self.forces.par_iter_mut())
            .zip(self.masses.par_iter())
            .zip(self.shapes.par_iter_mut());

        zip_iter.for_each(|((((((&active, pos), vel), acc), force), mass), shape)| {
            if active {
                // Apply Gravity
                let gravity_force = GRAVITY.scale(mass.0);
                force.0 = force.0.add(&gravity_force);

                // F = ma -> a = F/m
                acc.0 = force.0.scale(1.0 / mass.0);

                // v = v0 + at
                vel.0 = vel.0.add(&acc.0.scale(dt));

                // update position - simple Euler integration for now, or updating vertices
                let displacement = vel.0.scale(dt);
                pos.0 = pos.0.add(&displacement);

                // Update shape vertices
                for vertex in &mut shape.0.vertices {
                    *vertex = vertex.add(&displacement);
                }

                // Reset forces
                force.0 = Vector3d::zero();
            }
        });
    }
}
