use crate::geometry::vector::Vector3d;

/// Deterministic math operations for cross-platform simulation.
/// In a truly deterministic setup, this would use fixed-point math or
/// strict IEEE 754 operations without FMA or specialized instructions
/// that differ between architectures.
/// For now, we wrap standard f32 operations and provide macros/functions
/// that ensure determinism across platforms by standardizing rounding and NaN behavior.

pub fn deterministic_sqrt(val: f32) -> f32 {
    // libm provides cross-platform consistent math
    libm::sqrtf(val)
}

pub fn deterministic_acos(val: f32) -> f32 {
    libm::acosf(val)
}

pub fn deterministic_sin(val: f32) -> f32 {
    libm::sinf(val)
}

pub fn deterministic_cos(val: f32) -> f32 {
    libm::cosf(val)
}

// Optional helper for vector magnitude using deterministic sqrt
pub fn deterministic_magnitude(v: &Vector3d) -> f32 {
    deterministic_sqrt(v.x * v.x + v.y * v.y + v.z * v.z)
}
