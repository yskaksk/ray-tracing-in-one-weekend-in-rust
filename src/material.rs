use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};

use rand::rngs::ThreadRng;

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Metal { albedo: Color },
    Lambertian { albedo: Color },
}

impl Material {
    pub fn scatter(
        &self,
        r_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        rng: &mut ThreadRng,
    ) -> bool {
        match self {
            Material::Lambertian { albedo } => {
                let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector(rng);
                if scatter_direction.near_zero() {
                    scatter_direction = hit_record.normal;
                }
                *scattered = Ray::new(hit_record.p, scatter_direction);
                *attenuation = albedo.clone();
                return true;
            }
            Material::Metal { albedo } => {
                let reflected = Vec3::reflect(&r_in.direction.unit_vector(), &hit_record.normal);
                *scattered = Ray::new(hit_record.p, reflected);
                *attenuation = albedo.clone();
                return scattered.direction.dot(&hit_record.normal) > 0.0;
            }
        }
    }
}
