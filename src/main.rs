mod image;
mod vec3;
mod ray;
mod sphere;

pub use image::*;
pub use vec3::*;
pub use ray::*;
pub use sphere::*;

const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_W: u32 = 400;
const IMAGE_H: u32 = (IMAGE_W as f32 / ASPECT_RATIO) as u32;

fn calc_percents(y: u32) -> i32 {
    ((y as f32 / IMAGE_H as f32)*100.0).round() as i32
}

fn ray_color(ray: &Ray) -> Vec3 {
    let sphere = Sphere::new(Vec3(0.0, 0.0, -1.0), 0.5);
    let t = ray.hit_sphere(&sphere);
    if t > 0.0 {
        let sphere_normal = ray.at(t) - sphere.origin;
        return 0.5*Vec3(sphere_normal.0 + 1.0, sphere_normal.1 + 1.0, sphere_normal.2 + 1.0);
    }

    let unit_dir = ray.direction.normalized();
    let t = 0.5*(unit_dir.1 + 1.0);
    (1.0 - t)*Vec3(1.0, 1.0, 1.0) + t*Vec3(0.5, 0.7, 1.0)
}

fn main() {
    let mut image = Image::new(IMAGE_W, IMAGE_H);

    let vp = {
        let vp_h = 2.0;
        let vp_w = ASPECT_RATIO as f64*vp_h;
        (vp_w, vp_h)
    };
    let focal_length = 1.0;

    let origin = Vec3(0.0, 0.0, 0.0);
    let h_axis = Vec3(vp.0, 0.0, 0.0);
    let v_axis = Vec3(0.0, vp.1, 0.0);
    let lower_left_corner = origin - h_axis / 2.0 - v_axis / 2.0 - Vec3(0.0, 0.0, focal_length);

    let mut last_progress = -1;
    for y in 0..IMAGE_H {
        let progress = calc_percents(y);
        if progress != last_progress {
            last_progress = progress;
            println!("Progress: {}%", progress);
        }

        for x in 0..IMAGE_W {
            let u = x as f64 / (IMAGE_W - 1) as f64;
            let v = y as f64 / (IMAGE_H - 1) as f64;
            let ray = Ray::new(origin, lower_left_corner + u*h_axis + v*v_axis - origin);

            image.set_pixel(x, IMAGE_H - y - 1, &ray_color(&ray));
        }
    }
    image.save("result.ppm");

    println!("Done!");
    std::io::stdin().read_line(&mut String::new()).unwrap();
}
