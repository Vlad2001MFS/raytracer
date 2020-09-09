use crate::Vec3;
use crate::Ray;

pub struct Sphere {
    pub origin: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(origin: Vec3, radius: f64) -> Sphere {
        Sphere {
            origin,
            radius,
        }
    }

    pub fn hit_ray(&self, ray: &Ray) -> f64 {
        // dot((t^2)*b, b) + 2*t*dot(b, A - C) + dot(A - C, A - C) - r^2 = 0
        // (t^2)*dot(b, b) + 2*t*dot(b, A - C) + dot(A - C, A - C) - r^2 = 0
        //  dot((t^2)*b, b) = (t^2)*bX*bX + (t^2)*bY*bY + (t^2)*bZ*bZ = (t^2)*(bX*bX + bY*bY + bZ*bZ)
        // dot(b, b)*(t^2) + 2*dot(b, A - C)*t + dot(A - C, A - C) - r^2 = 0
        // A        *(t^2) + B              *t + C                       = 0
        // x = (-B +- sqrt(B^2 - 4*A*C)) / (2*A)
        //   = (-2*half_B +- sqrt((2*half_B)^2 - 4*A*C)) / (2*A)
        //   = (-2*half_B +- 2*sqrt(half_B^2 - A*C)) / (2*A)
        //   = (-half_B +- sqrt(half_B^2 - A*C)) / A

        let a = ray.direction.length_sq();
        let half_b = ray.direction.dot(ray.origin - self.origin);
        let c = (ray.origin - self.origin).length_sq() - self.radius*self.radius;
        let d = half_b*half_b - a*c;
        if d < 0.0 {
            -1.0
        }
        else {
            (-half_b - d.sqrt()) / a
        }
    }
}