use crate::{
    Vec3,
    Ray,
};

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    h_axis: Vec3,
    v_axis: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64) -> Camera {
        let vp = {
            let vp_h = 2.0;
            let vp_w = aspect_ratio*vp_h;
            (vp_w, vp_h)
        };
        let focal_length = 1.0;

        let origin = Vec3(0.0, 0.0, 0.0);
        let h_axis = Vec3(vp.0, 0.0, 0.0);
        let v_axis = Vec3(0.0, vp.1, 0.0);

        Camera {
            origin,
            lower_left_corner: origin - h_axis / 2.0 - v_axis / 2.0 - Vec3(0.0, 0.0, focal_length),
            h_axis,
            v_axis,
        }
    }

    pub fn calc_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + u*self.h_axis + v*self.v_axis - self.origin)
    }
}