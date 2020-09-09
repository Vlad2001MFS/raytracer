use crate::Vec3;
use std::{
    fs::File,
    io::Write,
};

pub struct Image {
    data: Vec<Vec3>,
    size: (u32, u32),
}

impl Image {
    pub fn new(w: u32, h: u32) -> Image {
        Image {
            data: (0..w*h).map(|_| Vec3(0.0, 0.0, 0.0)).collect(),
            size: (w, h),
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: &Vec3) {
        self.data[(x + y*self.size.0) as usize] = *color;
    }

    pub fn save(&self, path: &str) {
        let mut file = File::create(path).unwrap();
        file.write_fmt(format_args!("P3\n{} {}\n255\n", self.size.0, self.size.1)).unwrap();

        for texel in self.data.iter() {
            let ir = (255.999*texel.0) as i32;
            let ig = (255.999*texel.1) as i32;
            let ib = (255.999*texel.2) as i32;

            file.write_fmt(format_args!("{} {} {}\n", ir, ig, ib)).unwrap();
        }
    }
}