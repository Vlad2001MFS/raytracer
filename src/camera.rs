use crate::{
    Vec3,
    Ray,
};

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    h_axis: Vec3,
    v_axis: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(eye: Vec3, target: Vec3, up: Vec3, fov_y: f64, aspect_ratio: f64, aperture: f64, focus_dist: f64) -> Camera {
        let h = (fov_y / 2.0).tan();
        let vp = {
            let vp_h = 2.0*h;
            let vp_w = aspect_ratio*vp_h;
            (vp_w, vp_h)
        };
        
        let w = (eye - target).normalized();
        let u = up.cross(w).normalized();
        let v = w.cross(u);

        let origin = eye;
        let h_axis = focus_dist*vp.0*u;
        let v_axis = focus_dist*vp.1*v;

        Camera {
            origin,
            lower_left_corner: origin - h_axis / 2.0 - v_axis / 2.0 - focus_dist*w,
            h_axis,
            v_axis,
            u,
            v,
            w,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn calc_ray(&self, u: f64, v: f64) -> Ray {
        let rd = self.lens_radius*Vec3::random_in_unit_disk();
        let offset = self.u*rd.0 + self.v*rd.1;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + u*self.h_axis + v*self.v_axis - self.origin - offset
        )
    }
}