use crate::geometry::vector::Vector3d;
use crate::physics::rigid_body::RigidBody;

pub fn resolve_collision(
    body1: &mut RigidBody,
    body2: &mut RigidBody,
    normal: Vector3d,
    restitution: f32,
    friction: f32,
) {
    if body1.inverse_mass + body2.inverse_mass == 0.0 {
        return; // Both are static bodies
    }

    let relative_velocity = body2.velocity.subtract(&body1.velocity);

    // Calculate velocity along the normal
    let velocity_along_normal = relative_velocity.dot(&normal);

    // Do not resolve if velocities are separating
    if velocity_along_normal > 0.0 {
        return;
    }

    // Calculate restitution
    let e = restitution.min(body1.restitution.min(body2.restitution));

    // Calculate impulse scalar
    let mut j = -(1.0 + e) * velocity_along_normal;
    j /= body1.inverse_mass + body2.inverse_mass;

    // Apply impulse
    let impulse = normal.scale(j);

    body1.velocity = body1.velocity.subtract(&impulse.scale(body1.inverse_mass));
    body2.velocity = body2.velocity.add(&impulse.scale(body2.inverse_mass));

    // Friction
    let relative_velocity_post_impulse = body2.velocity.subtract(&body1.velocity);

    let tangent = relative_velocity_post_impulse.subtract(&normal.scale(relative_velocity_post_impulse.dot(&normal)));
    if tangent.magnitude() > 0.0 {
        let tangent = tangent.normalize();

        // Calculate tangent velocity
        let jt = -relative_velocity_post_impulse.dot(&tangent);
        let mut jt = jt / (body1.inverse_mass + body2.inverse_mass);

        // Calculate friction
        let mu = friction.min(body1.friction.min(body2.friction));

        // Clamp friction
        let friction_impulse = if jt.abs() < j * mu {
            tangent.scale(jt)
        } else {
            tangent.scale(-j * mu)
        };

        // Apply friction impulse
        body1.velocity = body1.velocity.subtract(&friction_impulse.scale(body1.inverse_mass));
        body2.velocity = body2.velocity.add(&friction_impulse.scale(body2.inverse_mass));
    }
}