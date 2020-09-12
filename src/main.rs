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

fn ray_color(ray: &Ray, scene: &Scene) -> Vec3 {
    if let Some(info) = scene.hit_ray(&ray, 0.0, std::f64::INFINITY) {
        return 0.5*(info.normal().clone() + Vec3(1.0, 1.0, 1.0));
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

        for x in 0..IMAGE_W {
            let u = x as f64 / (IMAGE_W - 1) as f64;
            let v = y as f64 / (IMAGE_H - 1) as f64;
            let ray = camera.calc_ray(u, v);

            image.set_pixel(x, IMAGE_H - y - 1, &ray_color(&ray, &scene));
        }
    }
    image.save("result.ppm");

    println!("Done!");
    std::io::stdin().read_line(&mut String::new()).unwrap();
}
