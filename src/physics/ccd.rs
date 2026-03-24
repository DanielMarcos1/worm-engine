use crate::geometry::vector::Vector3d;
use crate::physics::aabb_collision::AABB;
use crate::physics::math;

/// Represents a Time of Impact (TOI) result for CCD
pub struct TOIResult {
    pub is_colliding: bool,
    pub toi: f32, // Time of impact, usually between 0.0 and 1.0
}

/// Computes the Time of Impact (TOI) between two moving AABBs using swept AABB collision detection.
/// This prevents tunneling at high velocities.
pub fn swept_aabb(
    box1: &AABB,
    vel1: &Vector3d,
    box2: &AABB,
    vel2: &Vector3d,
    dt: f32,
) -> TOIResult {
    // Relative velocity of box1 with respect to box2
    let v = vel1.subtract(vel2).scale(dt);

    let mut t_first = 0.0f32;
    let mut t_last = 1.0f32;

    // Check X axis
    if v.x == 0.0 {
        if box1.max.x < box2.min.x || box1.min.x > box2.max.x {
            return TOIResult { is_colliding: false, toi: 1.0 };
        }
    } else {
        let t1 = (box2.min.x - box1.max.x) / v.x;
        let t2 = (box2.max.x - box1.min.x) / v.x;

        let t_entry = t1.min(t2);
        let t_exit = t1.max(t2);

        if t_entry > t_first { t_first = t_entry; }
        if t_exit < t_last { t_last = t_exit; }
        if t_first > t_last {
            return TOIResult { is_colliding: false, toi: 1.0 };
        }
    }

    // Check Y axis
    if v.y == 0.0 {
        if box1.max.y < box2.min.y || box1.min.y > box2.max.y {
            return TOIResult { is_colliding: false, toi: 1.0 };
        }
    } else {
        let t1 = (box2.min.y - box1.max.y) / v.y;
        let t2 = (box2.max.y - box1.min.y) / v.y;

        let t_entry = t1.min(t2);
        let t_exit = t1.max(t2);

        if t_entry > t_first { t_first = t_entry; }
        if t_exit < t_last { t_last = t_exit; }
        if t_first > t_last {
            return TOIResult { is_colliding: false, toi: 1.0 };
        }
    }

    // Check Z axis
    if v.z == 0.0 {
        if box1.max.z < box2.min.z || box1.min.z > box2.max.z {
            return TOIResult { is_colliding: false, toi: 1.0 };
        }
    } else {
        let t1 = (box2.min.z - box1.max.z) / v.z;
        let t2 = (box2.max.z - box1.min.z) / v.z;

        let t_entry = t1.min(t2);
        let t_exit = t1.max(t2);

        if t_entry > t_first { t_first = t_entry; }
        if t_exit < t_last { t_last = t_exit; }
        if t_first > t_last {
            return TOIResult { is_colliding: false, toi: 1.0 };
        }
    }

    // Valid collision if entry time is within [0.0, 1.0]
    if t_first >= 0.0 && t_first <= 1.0 {
        return TOIResult {
            is_colliding: true,
            toi: t_first,
        };
    }

    TOIResult { is_colliding: false, toi: 1.0 }
}
