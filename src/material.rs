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
    Dielectric(DielectricMtl),
}

impl Material {
    pub fn scatter(&self, ray_in: &Ray, hit_info: &HitInfo) -> Option<ScatterInfo> {
        match self {
            Material::Lambertian(mtl) => mtl.scatter(ray_in, hit_info),
            Material::Metal(mtl) => mtl.scatter(ray_in, hit_info),
            Material::Dielectric(mtl) => mtl.scatter(ray_in, hit_info),
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
    fuzziness: f64,
}

impl MetalMtl {
    pub fn new(albedo: Vec3, fuzziness: f64) -> MetalMtl {
        MetalMtl {
            albedo,
            fuzziness: fuzziness.min(1.0).max(0.0),
        }
    }

    pub fn scatter(&self, ray_in: &Ray, hit_info: &HitInfo) -> Option<ScatterInfo> {
        let reflected = ray_in.direction.reflected(hit_info.normal().clone());
        let scattered_ray = Ray::new(hit_info.point().clone(), reflected + Vec3::random_unit_vector()*self.fuzziness);
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

#[derive(Clone)]
pub struct DielectricMtl {
    ref_idx: f64
}

impl DielectricMtl {
    pub fn new(ref_idx: f64) -> DielectricMtl {
        DielectricMtl {
            ref_idx
        }
    }

    pub fn scatter(&self, ray_in: &Ray, hit_info: &HitInfo) -> Option<ScatterInfo> {
        Some(ScatterInfo {
            attenuation: Vec3(1.0, 1.0, 1.0),
            scattered_ray: {
                let etai_over_etat = if hit_info.is_front_face() { 1.0 / self.ref_idx } else { self.ref_idx };
                let unit_dir = ray_in.direction.normalized();
                let cos_theta = unit_dir.dot(-hit_info.normal().clone()).min(1.0);
                let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();
                if etai_over_etat*sin_theta > 1.0 {
                    Ray::new(hit_info.point().clone(), unit_dir.reflected(hit_info.normal().clone()))
                }
                else {
                    Ray::new(hit_info.point().clone(), unit_dir.refracted(hit_info.normal().clone(), etai_over_etat))
                }
            }
        })
    }
}