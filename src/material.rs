use crate::{
    Vec3, Ray, HitInfo,
};

#[derive(Clone)]
pub struct ScatterInfo {
    attenuation: Vec3,
    scattered_ray: Ray,
}

impl ScatterInfo {
    pub fn attenuation(&self) -> &Vec3 {
        &self.attenuation
    }

    pub fn scattered_ray(&self) -> &Ray {
        &self.scattered_ray
    }
}

#[derive(Clone)]
pub enum Material {
    Lambertian(LambertianMtl),
    Metal(MetalMtl),
}

impl Material {
    pub fn scatter(&self, ray_in: &Ray, hit_info: &HitInfo) -> Option<ScatterInfo> {
        match self {
            Material::Lambertian(mtl) => mtl.scatter(ray_in, hit_info),
            Material::Metal(mtl) => mtl.scatter(ray_in, hit_info),
        }
    }
}

#[derive(Clone)]
pub struct LambertianMtl {
    albedo: Vec3,
}

impl LambertianMtl {
    pub fn new(albedo: Vec3) -> LambertianMtl {
        LambertianMtl {
            albedo
        }
    }

    pub fn scatter(&self, _ray_in: &Ray, hit_info: &HitInfo) -> Option<ScatterInfo> {
        let scatter_dir = hit_info.normal().clone() + Vec3::random_unit_vector();
        Some(ScatterInfo {
            attenuation: self.albedo,
            scattered_ray: Ray::new(hit_info.point().clone(), scatter_dir),
        })
    }
}

#[derive(Clone)]
pub struct MetalMtl {
    albedo: Vec3,
}

impl MetalMtl {
    pub fn new(albedo: Vec3) -> MetalMtl {
        MetalMtl {
            albedo
        }
    }

    pub fn scatter(&self, ray_in: &Ray, hit_info: &HitInfo) -> Option<ScatterInfo> {
        let scattered_ray = Ray::new(hit_info.point().clone(), ray_in.direction.reflected(hit_info.normal().clone()));
        if scattered_ray.direction.dot(hit_info.normal().clone()) > 0.0 {
            Some(ScatterInfo {
                attenuation: self.albedo,
                scattered_ray,
            })
        }
        else {
            None
        }
    }
}