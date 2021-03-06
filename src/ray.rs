use crate::vec3::{Point3, Vec3};

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction.mul(t)
    }
}
