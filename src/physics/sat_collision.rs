use core::f32;

use crate::geometry::polygon::Polygon;
use crate::geometry::vector::Vector3d;

pub fn are_polygons_colliding(poly1: &Polygon, poly2: &Polygon) -> Option<Vector3d> {
    
    fn project_onto_axis(vertices: &[Vector3d], axis: &Vector3d) -> (f32, f32) {
        vertices.iter().map(|v| v.dot(axis)).fold(
            (f32::INFINITY, f32::NEG_INFINITY),
            |(min, max), projection| (min.min(projection), max.max(projection)),
        )
    }

    fn intervals_overlap(min1: f32, max1: f32, min2: f32, max2: f32) -> Option<f32> {
        if max1 < min2 || max2 < min1 {
            None
        } else {
            Some((max1.min(max2) - min1.max(min2)).abs())
        }
    }

    fn get_face_normals(edges: &[Vector3d]) -> Vec<Vector3d> {
        edges.iter().map(|edge| edge.normalize()).collect()
    }

    fn get_cross_products(edges1: &[Vector3d], edges2: &[Vector3d]) -> Vec<Vector3d> {
        let mut axes = Vec::new();
        for edge1 in edges1 {
            for edge2 in edges2 {
                let axis = edge1.cross(edge2);
                if axis.magnitude() > 0.0 {
                    axes.push(axis.normalize());
                }
            }
        }
        axes
    }

    // Ensure valid polygons
    if poly1.vertices.len() < 3 || poly2.vertices.len() < 3 {
        panic!("Both polygons must have at least 3 vertices");
    }

    let edges1 = poly1.edges();
    let edges2 = poly2.edges();

    let mut axes = Vec::new();
    axes.extend(get_face_normals(&edges1));
    axes.extend(get_face_normals(&edges2));

    // For 3D polygons
    axes.extend(get_cross_products(&edges1, &edges2));

    let mut mtv_axis = None;
    let mut min_overlap = f32::INFINITY;

    for axis in axes {
        let (min1, max1) = project_onto_axis(&poly1.vertices, &axis);
        let (min2, max2) = project_onto_axis(&poly2.vertices, &axis);

        if let Some(overlap) = intervals_overlap(min1, max1, min2, max2) {
            if overlap < min_overlap {
                min_overlap = overlap;
                mtv_axis = Some(axis);
            }
        } else {
            return None;
        }
    }

    mtv_axis.map(|axis| {
        let direction = poly2.vertices[0].subtract(&poly1.vertices[0]);
        if axis.dot(&direction) < 0.0 {
            axis.scale(-min_overlap) // Flip the axis direction
        } else {
            axis.scale(min_overlap)
        }
    })
}