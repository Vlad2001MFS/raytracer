use crate::Vec3;
use crate::Sphere;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {
            origin,
            direction,
        }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction*t
    }

    pub fn hit_sphere(&self, sphere: &Sphere) -> f64 {
        // dot((t^2)*b, b) + 2*t*dot(b, A - C) + dot(A - C, A - C) - r^2 = 0
        // (t^2)*dot(b, b) + 2*t*dot(b, A - C) + dot(A - C, A - C) - r^2 = 0
        //  dot((t^2)*b, b) = (t^2)*bX*bX + (t^2)*bY*bY + (t^2)*bZ*bZ = (t^2)*(bX*bX + bY*bY + bZ*bZ)
        // dot(b, b)*(t^2) + 2*dot(b, A - C)*t + dot(A - C, A - C) - r^2 = 0
        // A        *(t^2) + B              *t + C                       = 0
        // x = (-B +- sqrt(B^2 - 4*A*C)) / (2*A)
        //   = (-2*half_B +- sqrt((2*half_B)^2 - 4*A*C)) / (2*A)
        //   = (-2*half_B +- 2*sqrt(half_B^2 - A*C)) / (2*A)
        //   = (-half_B +- sqrt(half_B^2 - A*C)) / A
        let a = self.direction.length_sq();
        let half_b = self.direction.dot(self.origin - sphere.origin);
        let c = (self.origin - sphere.origin).length_sq() - sphere.radius*sphere.radius;
        let d = half_b*half_b - a*c;
        if d < 0.0 {
            -1.0
        }
        else {
            (-half_b - d.sqrt()) / a
        }
    }
}