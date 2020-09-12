use crate::{
    Geometry, HitInfo, Ray,
};

pub struct Scene {
    objects: Vec<Geometry>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            objects: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, geom: Geometry) {
        self.objects.push(geom);
    }

    pub fn hit_ray(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitInfo> {
        let mut hit_info = None;
        let mut closest_so_far = t_max;

        for obj in self.objects.iter() {
            if let Some(info) = obj.hit_ray(ray, t_min, closest_so_far) {
                hit_info = Some(info.clone());
                closest_so_far = info.t();
            }
        }

        hit_info
    }
}