mod image;
mod vec3;
mod ray;

pub use image::*;
pub use vec3::*;
pub use ray::*;

use std::io::Write;

const IMAGE_W: u32 = 256;
const IMAGE_H: u32 = 256;

fn calc_percents(y: u32) -> i32 {
    (100.0 - (y as f32 / IMAGE_H as f32)*100.0) as i32
}

fn main() {
    let mut image = Image::new(IMAGE_W, IMAGE_H);

    for y in (0..IMAGE_H).rev() {
        println!("Progress: {}%", calc_percents(y));
        std::io::stdout().flush().unwrap();
        for x in 0..IMAGE_W {
            let r = x as f64 / (IMAGE_W - 1) as f64;
            let g = y as f64 / (IMAGE_H - 1) as f64;
            let b = 0.25;

            let ir = (255.999*r) as u8;
            let ig = (255.999*g) as u8;
            let ib = (255.999*b) as u8;

            image.set_pixel(x, y, [ir, ig, ib]);
        }
    }
    image.save("result.ppm");

    println!("Done!");
    std::io::stdin().read_line(&mut String::new()).unwrap();
}
