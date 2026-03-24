use crate::{geometry::vector::Vector3d, physics::rigid_body::RigidBody};
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

    pub fn step(&mut self) {
        // Implement rayon parallel iterators for physics step resolution (Task 3.2)
        let dt = self.time_step;
        self.bodies.par_iter_mut().for_each(|body| {
            body.apply_gravity();
            body.update(dt);
        });
    }
}
