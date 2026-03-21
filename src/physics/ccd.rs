use crate::geometry::{polygon::Polygon, vector::Vector3d};
use crate::physics::components::{Position, Shape, Velocity};

/// Continuous Collision Detection to prevent high velocity tunneling.
/// Calculates the Time Of Impact (TOI) between two bodies.
pub fn calculate_toi(
    pos_a: &Position,
    vel_a: &Velocity,
    shape_a: &Shape,
    pos_b: &Position,
    vel_b: &Velocity,
    shape_b: &Shape,
    dt: f32,
) -> Option<f32> {
    // Basic Ray-Cast/Swept approach for AABB or bounding spheres.
    // For simplicity and performance, we'll approximate using bounding spheres.

    let center_a = get_center(&shape_a.0).add(&pos_a.0);
    let center_b = get_center(&shape_b.0).add(&pos_b.0);

    let radius_a = get_radius(&shape_a.0, &center_a);
    let radius_b = get_radius(&shape_b.0, &center_b);

    // Relative velocity
    let v_rel = vel_a.0.subtract(&vel_b.0);

    // Relative position
    let s_rel = center_a.subtract(&center_b);

    let r_total = radius_a + radius_b;

    // Quadratic equation parameters: a*t^2 + b*t + c = 0
    let a = v_rel.dot(&v_rel);
    let b = 2.0 * s_rel.dot(&v_rel);
    let c = s_rel.dot(&s_rel) - (r_total * r_total);

    // If they are already intersecting
    if c <= 0.0 {
        return Some(0.0);
    }

    // If objects are not moving relative to each other
    if a == 0.0 {
        return None;
    }

    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        return None; // No collision
    }

    let sqrt_d = discriminant.sqrt();
    let t1 = (-b - sqrt_d) / (2.0 * a);
    let t2 = (-b + sqrt_d) / (2.0 * a);

    let toi = if t1 >= 0.0 && t1 <= dt {
        Some(t1)
    } else if t2 >= 0.0 && t2 <= dt {
        Some(t2)
    } else {
        None
    };

    toi
}

fn get_center(polygon: &Polygon) -> Vector3d {
    if polygon.vertices.is_empty() {
        return Vector3d::zero();
    }
    let mut sum = Vector3d::zero();
    for v in &polygon.vertices {
        sum = sum.add(v);
    }
    sum.scale(1.0 / polygon.vertices.len() as f32)
}

fn get_radius(polygon: &Polygon, center: &Vector3d) -> f32 {
    let mut max_sq = 0.0_f32;
    for v in &polygon.vertices {
        let dist_sq = v.subtract(center).length_squared();
        if dist_sq > max_sq {
            max_sq = dist_sq;
        }
    }
    max_sq.sqrt()
}
