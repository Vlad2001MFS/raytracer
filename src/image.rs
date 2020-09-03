use std::{
    fs::File,
    io::Write,
};

pub struct Image {
    data: Vec<[u8; 3]>,
    size: (u32, u32),
}

impl Image {
    pub fn new(w: u32, h: u32) -> Image {
        Image {
            data: (0..w*h).map(|_| [0, 0, 0]).collect(),
            size: (w, h),
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: [u8; 3]) {
        self.data[(x + y*self.size.0) as usize] = color;
    }

    pub fn save(&self, path: &str) {
        let mut file = File::create(path).unwrap();
        file.write_fmt(format_args!("P3\n{} {}\n255\n", self.size.0, self.size.1)).unwrap();
    
        for y in (0..self.size.1).rev() {
            for x in 0..self.size.0 {
                let r = x as f64 / (self.size.0 - 1) as f64;
                let g = y as f64 / (self.size.1 - 1) as f64;
                let b = 0.25;
    
                let ir = (255.999*r) as i32;
                let ig = (255.999*g) as i32;
                let ib = (255.999*b) as i32;
    
                file.write_fmt(format_args!("{} {} {}\n", ir, ig, ib)).unwrap();
            }
        }
    }
}