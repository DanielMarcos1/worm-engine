use crate::geometry::vector::Vector3d;
use crate::physics::math::DeterministicMath;

/// Computes the Time of Impact (TOI) between two moving spheres.
/// Returns `Some(t)` where `0.0 <= t <= 1.0` representing the fraction of the timestep
/// at which the collision occurs. If no collision occurs within the timestep, returns `None`.
pub fn calculate_toi_sphere_sphere(
    p1: Vector3d,
    v1: Vector3d,
    r1: f32,
    p2: Vector3d,
    v2: Vector3d,
    r2: f32,
    dt: f32,
) -> Option<f32> {
    // Relative velocity
    let dv = v1.subtract(&v2).scale(dt);
    // Relative position
    let dp = p1.subtract(&p2);

    let r_sum = r1 + r2;

    let a = dv.length_squared();
    let b = 2.0 * dp.dot(&dv);
    let c = dp.length_squared() - r_sum * r_sum;

    // Check if already intersecting
    if c <= 0.0 {
        return Some(0.0);
    }

    // If bodies are moving apart or stationary, no collision
    if a == 0.0 || b >= 0.0 {
        return None;
    }

    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        return None; // No real roots, no collision
    }

    // We want the smallest positive root
    let t = (-b - discriminant.d_sqrt()) / (2.0 * a);

    if t >= 0.0 && t <= 1.0 {
        Some(t)
    } else {
        None
    }
}
