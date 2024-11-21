use crate::geometry::vector::Vector3d;

#[derive(Debug)]
pub struct Circle {
    pub center: Vector3d,
    pub radius: f32, 
    pub velocity: Vector3d,
    pub mass: f32,
}

#[derive(Debug)]
pub struct Point {
    pub position: Vector3d
}

impl Circle {
    pub fn contains_point(&self, point: &Point) -> bool {
        // Calculate the squared distance from the point to the circle's center
        let distance_squared = (self.center.x - point.position.x).powi(2)
            + (self.center.y - point.position.y).powi(2)
            + (self.center.z - point.position.z).powi(2);

        // Compare with square radius to avoid expensive square root computation
        distance_squared <= self.radius.powi(2)
    }

    pub fn collides_with(&self, other: &Circle) -> bool {
        let distance_squared = (self.center.x - other.center.x).powi(2) 
            + (self.center.y - other.center.y).powi(2)
            + (self.center.z - other.center.z).powi(2);
    
        distance_squared <= (self.radius + other.radius).powi(2)
    }

    pub fn resolve_collision(&mut self, other: &mut Circle, restitution: f32) {
        
        // Handle 0 mass objects
        if self.mass <= 0.0 || other.mass <= 0.0 {
            return
        }
        
        // Normal vector (Line connecting the centers)
        let normal = (self.center.subtract(&other.center)).normalize();

        if normal.magnitude() == 0.0 {
            return
        }
        
        let relative_velocity = self.velocity.subtract(&other.velocity);

        let velocity_along_normal = relative_velocity.dot(&normal);

        if velocity_along_normal > 0.0 {
            return
        }

        let impulse_scalar = -(1.0 + restitution) * velocity_along_normal
            / (1.0 / self.mass + 1.0 / other.mass);

        
        // Apply impulse
        let impulse = normal.scale(impulse_scalar);
        self.velocity = self.velocity.add(&impulse.scale(1.0 / self.mass));
        other.velocity = other.velocity.subtract(&impulse.scale(1.0 / other.mass));
        
    }
}