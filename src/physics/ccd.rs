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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::vector::Vector3d;

    #[test]
    fn test_ccd_prevents_tunneling_high_velocity() {
        // Sphere 1 moving extremely fast (1000m/s) towards Sphere 2
        let p1 = Vector3d::new(0.0, 0.0, 0.0);
        let v1 = Vector3d::new(1000.0, 0.0, 0.0); // 1000m/s
        let r1 = 1.0;

        // Sphere 2 is stationary
        let p2 = Vector3d::new(10.0, 0.0, 0.0);
        let v2 = Vector3d::new(0.0, 0.0, 0.0);
        let r2 = 1.0;

        let dt = 1.0 / 60.0; // 60 FPS frame

        // Without CCD, Sphere 1 would be at x=16.6 by next frame, completely passing through Sphere 2 (x=10.0)
        let toi = calculate_toi_sphere_sphere(p1, v1, r1, p2, v2, r2, dt);

        assert!(toi.is_some(), "CCD should detect a collision");

        // At what fraction of dt does impact occur?
        // Distance to cover is 10.0 - (r1 + r2) = 8.0 meters.
        // Speed is 1000 m/s.
        // Time to impact = 8.0 / 1000 = 0.008 seconds.
        // dt is 0.01666...
        // TOI fraction = 0.008 / 0.01666... = 0.48

        let expected_time = 8.0 / 1000.0;
        let expected_toi = expected_time / dt;

        let actual_toi = toi.unwrap();

        assert!((actual_toi - expected_toi).abs() < 0.001, "Expected TOI approx {}, got {}", expected_toi, actual_toi);
    }
}
