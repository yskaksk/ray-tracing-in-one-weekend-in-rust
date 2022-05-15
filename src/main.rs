#![allow(dead_code)]

mod hittable;
mod ray;
mod vec3;

use hittable::{Hittable, HittableList, Sphere};
use ray::Ray;
use vec3::{Color, Point3, Vec3};

fn write_color(color: &Color) {
    let r = (255.999 * color.x).floor() as u8;
    let g = (255.999 * color.y).floor() as u8;
    let b = (255.999 * color.z).floor() as u8;
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

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> Option<f64> {
    let oc = r.origin - center.clone();
    let a = r.direction.length_squared();
    let half_b = oc.dot(r.direction);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant > 0.0 {
        Some((-half_b - discriminant.sqrt()) / a)
    } else {
        None
    }
}

fn main() {
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio).floor() as usize;

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let viewport_height: f64 = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::zero();
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin
        - horizontal.div(2.0).unwrap()
        - vertical.div(2.0).unwrap()
        - Vec3::new(0.0, 0.0, focal_length);

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let u = i as f64 / (image_width as f64 - 1.0);
            let v = j as f64 / (image_height as f64 - 1.0);
            let r = Ray {
                origin,
                direction: lower_left_corner + horizontal.mul(u) + vertical.mul(v) - origin,
            };
            let color = ray_color(r, &world);
            write_color(&color);
        }
    }
}
