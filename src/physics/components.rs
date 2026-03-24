use crate::geometry::{polygon::Polygon, vector::Vector3d};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity(pub usize);

#[derive(Debug, Clone, Copy)]
pub struct Position(pub Vector3d);

#[derive(Debug, Clone, Copy)]
pub struct Velocity(pub Vector3d);

#[derive(Debug, Clone, Copy)]
pub struct Acceleration(pub Vector3d);

#[derive(Debug, Clone, Copy)]
pub struct Force(pub Vector3d);

#[derive(Debug, Clone, Copy)]
pub struct Mass(pub f32);

#[derive(Debug, Clone)]
pub struct Shape(pub Polygon);
