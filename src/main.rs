extern crate rand;

use rand::{
    Rng,
    distributions::{
        Uniform,
    },
};

use std::rc::Rc;

mod image;
mod vec3;
mod ray;
mod geometry;
mod scene;
mod camera;
mod material;

pub use image::*;
pub use vec3::*;
pub use ray::*;
pub use geometry::*;
pub use scene::*;
pub use camera::*;
pub use material::*;

const ASPECT_RATIO: f64 = 3.0 / 2.0;
const IMAGE_W: u32 = 400;
const IMAGE_H: u32 = (IMAGE_W as f64 / ASPECT_RATIO) as u32;
const SAMPLES_PER_PIXEL: u32 = 100;
const MAX_DEPTH: u32 = 50;

fn ray_color(ray: &Ray, scene: &Scene, depth: u32) -> Vec3 {
    if depth <= 0 {
        return Vec3(0.0, 0.0, 0.0)
    }

    if let Some(hit_info) = scene.hit_ray(ray, 0.001, std::f64::INFINITY) {
        if let Some(scatter_info) = hit_info.material().scatter(ray, &hit_info) {
            return scatter_info.attenuation().clone()*ray_color(scatter_info.scattered_ray(), scene, depth - 1);
        }

        return Vec3(0.0, 0.0, 0.0)
    }

    let unit_dir = ray.direction.normalized();
    let t = 0.5*(unit_dir.1 + 1.0);
    (1.0 - t)*Vec3(1.0, 1.0, 1.0) + t*Vec3(0.5, 0.7, 1.0)
}

fn random_scene() -> Scene {
    let mut scene = Scene::new();

    let ground_mtl = Rc::new(Material::Lambertian(LambertianMtl::new(Vec3(0.5, 0.5, 0.5))));
    scene.add(Geometry::Sphere(Sphere::new(Vec3(0.0, -1000.0, 0.0), 1000.0, ground_mtl.clone())));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rand::random();
            let center = Vec3(a as f64 + 0.9*rand::random::<f64>(), 0.2, b as f64 + 0.9*rand::random::<f64>());

            if (center - Vec3(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_mtl = if choose_mat < 0.8 {
                    let albedo = Vec3::random()*Vec3::random();
                    Rc::new(Material::Lambertian(LambertianMtl::new(albedo)))
                }
                else if choose_mat < 0.95 {
                    let albedo = Vec3::random_range(0.5, 1.0);
                    let fuzz = rand::thread_rng().gen_range(0.0, 0.5);
                    Rc::new(Material::Metal(MetalMtl::new(albedo, fuzz)))
                }
                else {
                    Rc::new(Material::Dielectric(DielectricMtl::new(1.5)))
                };
                scene.add(Geometry::Sphere(Sphere::new(center, 0.2, sphere_mtl)));
            }
        }
    }

    let mtl1 = Rc::new(Material::Dielectric(DielectricMtl::new(1.5)));
    scene.add(Geometry::Sphere(Sphere::new(Vec3(0.0, 1.0, 0.0), 1.0, mtl1)));

    let mtl2 = Rc::new(Material::Lambertian(LambertianMtl::new(Vec3(0.4, 0.2, 0.1))));
    scene.add(Geometry::Sphere(Sphere::new(Vec3(-4.0, 1.0, 0.0), 1.0, mtl2)));
    
    let mtl3 = Rc::new(Material::Metal(MetalMtl::new(Vec3(0.7, 0.6, 0.5), 0.0)));
    scene.add(Geometry::Sphere(Sphere::new(Vec3(4.0, 1.0, 0.0), 1.0, mtl3)));

    scene
}

fn main() {
    let scene = random_scene();

    /*let ground_mtl = Rc::new(Material::Lambertian(LambertianMtl::new(Vec3(0.8, 0.8, 0.0))));
    let center_mtl = Rc::new(Material::Lambertian(LambertianMtl::new(Vec3(0.1, 0.2, 0.5))));
    let left_mtl = Rc::new(Material::Dielectric(DielectricMtl::new(1.5)));
    let right_mtl = Rc::new(Material::Metal(MetalMtl::new(Vec3(0.8, 0.6, 0.2), 0.0)));

    scene.add(Geometry::Sphere(Sphere::new(Vec3( 0.0, -100.5, -1.0),  100.0, ground_mtl)));
    scene.add(Geometry::Sphere(Sphere::new(Vec3( 0.0,  0.0,   -1.0),  0.5,   center_mtl)));
    scene.add(Geometry::Sphere(Sphere::new(Vec3(-1.0,  0.0,   -1.0),  0.5,   left_mtl.clone())));
    scene.add(Geometry::Sphere(Sphere::new(Vec3(-1.0,  0.0,   -1.0), -0.45,  left_mtl.clone())));
    scene.add(Geometry::Sphere(Sphere::new(Vec3( 1.0,  0.0,   -1.0),  0.5,   right_mtl)));*/

    let eye = Vec3(13.0, 2.0, 3.0);
    let target = Vec3(0.0, 0.0, 0.0);
    let camera = Camera::new(
        eye,
        target,
        Vec3(0.0, 1.0, 0.0),
        20.0_f64.to_radians(),
        ASPECT_RATIO,
        0.1,
        10.0
    );

    let mut image = Image::new(IMAGE_W, IMAGE_H);

    let mut last_progress = -1;
    for y in 0..IMAGE_H {
        let progress = ((y as f32 / IMAGE_H as f32)*100.0).round() as i32;
        if progress != last_progress {
            last_progress = progress;
            println!("Progress: {}%", progress);
        }

        let mut rand_gen = rand::thread_rng();
        let rand_distrib = Uniform::new(0.0, 1.0);
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
