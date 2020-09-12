extern crate rand;

use rand::{
    Rng,
    distributions::{
        Uniform,
    },
};

mod image;
mod vec3;
mod ray;
mod geometry;
mod scene;
mod camera;

pub use image::*;
pub use vec3::*;
pub use ray::*;
pub use geometry::*;
pub use scene::*;
pub use camera::*;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_W: u32 = 400;
const IMAGE_H: u32 = (IMAGE_W as f64 / ASPECT_RATIO) as u32;
const SAMPLES_PER_PIXEL: u32 = 100;
const MAX_DEPTH: u32 = 50;

fn ray_color(ray: &Ray, scene: &Scene, depth: u32) -> Vec3 {
    if depth <= 0 {
        return Vec3(0.0, 0.0, 0.0)
    }

    if let Some(info) = scene.hit_ray(&ray, 0.0, std::f64::INFINITY) {
        let target = info.point().clone() + info.normal().clone() + Vec3::random_unit_sphere();
        let next_ray = Ray::new(info.point().clone(), target - info.point().clone());
        return 0.5*ray_color(&next_ray, scene, depth - 1);
    }

    let unit_dir = ray.direction.normalized();
    let t = 0.5*(unit_dir.1 + 1.0);
    (1.0 - t)*Vec3(1.0, 1.0, 1.0) + t*Vec3(0.5, 0.7, 1.0)
}

fn main() {
    let mut scene = Scene::new();
    scene.add(Geometry::Sphere(Sphere::new(Vec3(0.0, 0.0, -1.0), 0.5)));
    scene.add(Geometry::Sphere(Sphere::new(Vec3(0.0, -100.5, -1.0), 100.0)));

    let mut image = Image::new(IMAGE_W, IMAGE_H);

    let camera = Camera::new(ASPECT_RATIO);

    let mut last_progress = -1;
    for y in 0..IMAGE_H {
        let progress = ((y as f32 / IMAGE_H as f32)*100.0).round() as i32;
        if progress != last_progress {
            last_progress = progress;
            println!("Progress: {}%", progress);
        }

        let mut rand_gen = rand::thread_rng();
        let rand_distrib = Uniform::new_inclusive(0.0, 1.0);
        for x in 0..IMAGE_W {
            let mut total_pixel_color = Vec3(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (x as f64 + rand_gen.sample::<f64, _>(rand_distrib)) / (IMAGE_W - 1) as f64;
                let v = (y as f64 + rand_gen.sample::<f64, _>(rand_distrib)) / (IMAGE_H - 1) as f64;
                let ray = camera.calc_ray(u, v);
                total_pixel_color += ray_color(&ray, &scene, MAX_DEPTH);
            }

            let pixel_color = total_pixel_color / SAMPLES_PER_PIXEL as f64;
            image.set_pixel(x, IMAGE_H - y - 1, &pixel_color);
        }
    }
    image.save("result.ppm");

    println!("Done!");
}
