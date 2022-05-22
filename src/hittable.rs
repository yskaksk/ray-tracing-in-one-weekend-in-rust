use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Color, Point3, Vec3};

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Material,
}

impl HitRecord {
    fn new(p: Point3, outward_normal: Vec3, t: f64, r: &Ray, material: Material) -> Self {
        let front_face = r.direction.dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        HitRecord {
            p,
            normal,
            t,
            front_face,
            material,
        }
    }

    fn empty() -> Self {
        let p = Point3::zero();
        let normal = Vec3::zero();
        let t = 0.0;
        let front_face = true;
        HitRecord {
            p,
            normal,
            t,
            front_face,
            material: Material::Lambertian {
                albedo: Color::new(0.0, 0.0, 0.0),
            },
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Material) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = oc.dot(&r.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrd = discriminant.sqrt();
        let r_n = (-half_b - sqrd) / a;
        let r_p = (-half_b + sqrd) / a;
        return if r_n >= t_min && r_n <= t_max {
            let t = r_n;
            let p = r.at(t);
            let outward_normal = (p - self.center).unit_vector();
            Some(HitRecord::new(p, outward_normal, t, r, self.material))
        } else if r_p >= t_min && r_p <= t_max {
            let t = r_p;
            let p = r.at(t);
            let outward_normal = (p - self.center).unit_vector();
            Some(HitRecord::new(p, outward_normal, t, r, self.material))
        } else {
            None
        };
    }
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        let objects: _ = vec![];
        HittableList { objects }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object)
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut tmp_rec = HitRecord::empty();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for obj in self.objects.iter() {
            if let Some(rec) = obj.hit(r, t_min, closest_so_far) {
                tmp_rec = rec.clone();
                closest_so_far = rec.t;
                hit_anything = true;
            } else {
                continue;
            }
        }
        return if hit_anything { Some(tmp_rec) } else { None };
    }
}
