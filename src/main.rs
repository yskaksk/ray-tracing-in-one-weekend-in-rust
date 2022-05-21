#![allow(dead_code)]

mod camera;
mod hittable;
mod ray;
mod vec3;

use camera::Camera;
use hittable::{Hittable, HittableList, Sphere};
use ray::Ray;
use vec3::{Color, Point3};

use rand::Rng;

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    return x;
}

fn write_color(color: &Color, samples_per_pixel: u8) {
    let scale = 1.0 / samples_per_pixel as f64;
    let r = (clamp(color.x * scale, 0.0, 0.999) * 256.0).floor() as u8;
    let g = (clamp(color.y * scale, 0.0, 0.999) * 256.0).floor() as u8;
    let b = (clamp(color.z * scale, 0.0, 0.999) * 256.0).floor() as u8;
    println!("{} {} {}", r, g, b);
}

fn ray_color<T: Hittable>(ray: Ray, world: &T) -> Color {
    if let Some(rec) = world.hit(&ray, 0.0, f64::MAX) {
        return (Color::one() + rec.normal).mul(0.5);
    } else {
        let unit_direction = ray.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        return Color::one().mul(1.0 - t) + Color::new(0.5, 0.7, 1.0).mul(t);
    }
}

fn main() {
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width = 800;
    let image_height = (image_width as f64 / aspect_ratio).floor() as usize;
    let samples_per_pixel = 100 as u8;

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let camera = Camera::new();

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    let mut rng = rand::thread_rng();

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let mut color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen::<f64>()) / (image_width as f64 - 1.0);
                let v = (j as f64 + rng.gen::<f64>()) / (image_height as f64 - 1.0);
                let r = camera.get_ray(u, v);
                color = color + ray_color(r, &world);
            }
            write_color(&color, samples_per_pixel);
        }
    }
}
