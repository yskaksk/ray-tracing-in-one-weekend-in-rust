#![allow(dead_code)]

mod vec3;

use vec3::{Color, Point3, Vec3};

fn write_color(color: &Color) {
    let r = (255.999 * color.x).floor() as u8;
    let g = (255.999 * color.y).floor() as u8;
    let b = (255.999 * color.z).floor() as u8;
    println!("{} {} {}", r, g, b);
}

struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    fn at(self, t: f64) -> Point3 {
        self.origin + self.direction.mul(t)
    }
}

fn ray_color(ray: Ray) -> Color {
    if hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, &ray) {
        return Color::new(1.0, 0.0, 0.0);
    }
    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    Color::one().mul(1.0 - t) + Color::new(0.5, 0.7, 1.0).mul(t)
}

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> bool {
    let oc = r.origin - center.clone();
    let a = r.direction.dot(r.direction);
    let b = 2.0 * oc.dot(r.direction);
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}

fn main() {
    let aspect_ratio: f64 = 16.0 / 9.0;

    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio).floor() as usize;

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
            let color = ray_color(r);
            write_color(&color);
        }
    }
}
