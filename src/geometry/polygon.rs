use super::vector::Vector3d;


// TODO: Check if the polygon is 3d or 2d
#[derive(Debug)]
pub struct Polygon {
    pub vertices: Vec<Vector3d>, // List of vertices of the polygon
}

impl Polygon {
    pub fn new(vertices: Vec<Vector3d>) -> Self {
        Self { vertices }
    }

    pub fn edges(&self) -> Vec<Vector3d> {
        let mut edges = Vec::new();
        let len = self.vertices.len();
        for i in 0..len {
            let start = self.vertices[i];
            let end = self.vertices[(i + 1) % len];
            edges.push(end.subtract(&start));
        }
        edges
    }
}

