use crate::geometry::polygon::Polygon;
use crate::geometry::vector::Vector3d;

pub fn are_polygons_colliding(poly1: &Polygon, poly2: &Polygon) -> bool {

    fn project_onto_axis(vertices: &Vec<Vector3d>, axis: &Vector3d) -> (f32, f32) {
        let mut min = vertices[0].dot(axis);
        let mut max = min;

        for vertex in vertices.iter().skip(1) {
            let projection = vertex.dot(axis);
            if projection < min {
                min = projection;
            }
            if projection > max {
                max = projection;
            }
        }

        (min, max)
    }

    fn intervals_overlap(min1: f32, max1: f32, min2: f32, max2: f32) -> bool {
        max1 >= min2 && max2 >= min1
    }

    let edges1 = poly1.edges();
    let edges2 = poly2.edges();

    for edge in edges1.iter().chain(edges2.iter()) {
        let axis = Vector3d::new(-edge.y, edge.x, 0.0).normalize();

        let (min1, max1) = project_onto_axis(&poly1.vertices, &axis);
        let (min2, max2) = project_onto_axis(&poly2.vertices, &axis);

        // Check overlap
        if !intervals_overlap(min1, max1, min2, max2) {
            return false;
        }
    }

    true
}