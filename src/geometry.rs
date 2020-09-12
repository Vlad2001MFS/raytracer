use crate::{
    Vec3, Ray, Material,
};
use std::rc::Rc;

#[derive(Clone)]
pub struct HitInfo {
    point: Vec3,
    normal: Vec3,
    t: f64,
    is_front_face: bool,
    material: Rc<Material>,
}

impl HitInfo {
    pub fn point(&self) -> &Vec3 {
        &self.point
    }

    pub fn normal(&self) -> &Vec3 {
        &self.normal
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn is_front_face(&self) -> bool {
        self.is_front_face
    }
    
    pub fn material(&self) -> &Rc<Material> {
        &self.material
    }
}

#[derive(Clone)]
pub enum Geometry {
    Sphere(Sphere),
}

impl Geometry {
    pub fn hit_ray(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitInfo> {
        match self {
            Geometry::Sphere(g) => g.hit_ray(ray, t_min, t_max),
        }
    }
}

#[derive(Clone)]
pub struct Sphere {
    pub origin: Vec3,
    pub radius: f64,
    pub material: Rc<Material>,
}

impl Sphere {
    pub fn new(origin: Vec3, radius: f64, material: Rc<Material>) -> Sphere {
        Sphere {
            origin,
            radius,
            material,
        }
    }

    pub fn hit_ray(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitInfo> {
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

        if d > 0.0 {
            let root = d.sqrt();

            let temp = [
                (-half_b - root) / a,
                (-half_b + root) / a,
            ];
            for temp in temp.iter() {
                if *temp < t_max && *temp > t_min {
                    let point = ray.at(*temp);
                    let normal = (point - self.origin) / self.radius;
                    let is_front_face = ray.direction.dot(normal) < 0.0;
                    
                    return Some(HitInfo {
                        point,
                        normal: if is_front_face { normal } else { -normal },
                        t: *temp,
                        is_front_face,
                        material: self.material.clone(),
                    })
                }
            }
        }

        None
    }
}