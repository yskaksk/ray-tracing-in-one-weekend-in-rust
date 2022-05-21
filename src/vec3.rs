#![allow(dead_code)]

use std::f64;
use std::fmt;
use std::ops::{Add, Mul, Neg, Sub};

use rand::random;
use rand::{rngs::ThreadRng, Rng};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub fn mul(self, t: f64) -> Self {
        Vec3 {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
    }

    pub fn div(self, t: f64) -> Option<Self> {
        if t == 0.0 {
            None
        } else {
            Some(Vec3 {
                x: self.x / t,
                y: self.y / t,
                z: self.z / t,
            })
        }
    }

    pub fn length_squared(self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn cross(self, other: Vec3) -> Self {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn unit_vector(self) -> Self {
        self.div(self.length()).unwrap()
    }

    pub fn zero() -> Self {
        Vec3::new(0.0, 0.0, 0.0)
    }

    pub fn one() -> Self {
        Vec3::new(1.0, 1.0, 1.0)
    }

    pub fn random(rng: &mut ThreadRng) -> Self {
        Vec3::new(rng.gen(), rng.gen(), rng.gen())
    }

    pub fn random_in_unit_sphere(rng: &mut ThreadRng) -> Self {
        let u = rng.gen::<f64>();
        let v = rng.gen::<f64>();
        let o = rng.gen::<f64>();
        let o_c = o.powf(1.0 / 3.0);
        let z = o_c * (-2.0 * u + 1.0);
        let x = o_c * (1.0 - z * z).sqrt() * (2.0 * f64::consts::PI * v).cos();
        let y = o_c * (1.0 - z * z).sqrt() * (2.0 * f64::consts::PI * v).sin();

        Vec3::new(x, y, z)
    }

    pub fn random_unit_vector(rng: &mut ThreadRng) -> Self {
        Self::random_in_unit_sphere(rng).unit_vector()
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

pub type Point3 = Vec3;
pub type Color = Vec3;

#[cfg(test)]
mod tests {
    const EPSILON: f64 = 0.0000000001;
    use super::*;

    #[test]
    fn vec3_new() {
        let expected = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        assert_eq!(Vec3::new(1.0, 2.0, 3.0), expected);
    }

    #[test]
    fn vec3_mul() {
        let expected = Vec3::new(2.0, 4.0, 6.0);
        assert_eq!(Vec3::new(1.0, 2.0, 3.0).mul(2.0), expected);
    }

    #[test]
    fn vec3_div() {
        let expected = Vec3::new(0.5, 1.0, 1.5);
        assert_eq!(Vec3::new(1.0, 2.0, 3.0).div(2.0).unwrap(), expected);
    }

    #[test]
    fn vec3_length_squared() {
        let expected = 1.0 * 1.0 + 2.0 * 2.0 + 3.0 * 3.0;
        assert_eq!(Vec3::new(1.0, 2.0, 3.0).length_squared(), expected);
    }

    #[test]
    fn vec3_length() {
        let expected = (1.0 * 1.0 + 2.0 * 2.0 + 3.0 * 3.0 as f64).sqrt();
        assert_eq!(Vec3::new(1.0, 2.0, 3.0).length(), expected);
    }

    #[test]
    fn vec3_dot() {
        let expected = 1.0 * 10.0 + 2.0 * 20.0 + 3.0 * 30.0;
        assert_eq!(
            Vec3::new(1.0, 2.0, 3.0).dot(Vec3::new(10.0, 20.0, 30.0)),
            expected
        );
    }

    #[test]
    fn vec3_cross() {
        // [1.0, 2.0, 3.0] x [5.0, 1.0, 4.0]
        let expected = Vec3::new(
            2.0 * 4.0 - 3.0 * 1.0,
            3.0 * 5.0 - 1.0 * 4.0,
            1.0 * 1.0 - 2.0 * 5.0,
        );
        assert_eq!(
            Vec3::new(1.0, 2.0, 3.0).cross(Vec3::new(5.0, 1.0, 4.0)),
            expected
        );
    }

    #[test]
    fn vec3_unit_vector() {
        let length = 14.0_f64.sqrt();
        let expected = Vec3::new(1.0 / length, 2.0 / length, 3.0 / length);
        let test = Vec3::new(1.0, 2.0, 3.0).unit_vector();
        assert!((test - expected).x.abs() < EPSILON);
        assert!((test - expected).y.abs() < EPSILON);
        assert!((test - expected).z.abs() < EPSILON);
    }

    #[test]
    fn vec3_zero() {
        assert_eq!(Vec3::zero(), Vec3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn vec3_one() {
        assert_eq!(Vec3::one(), Vec3::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn vec3_add() {
        let expected = Vec3::new(11.0, 22.0, 33.0);
        assert_eq!(
            Vec3::new(1.0, 2.0, 3.0) + Vec3::new(10.0, 20.0, 30.0),
            expected
        );
    }

    #[test]
    fn vec3_sub() {
        let expected = Vec3::new(9.0, 18.0, 27.0);
        assert_eq!(
            Vec3::new(10.0, 20.0, 30.0) - Vec3::new(1.0, 2.0, 3.0),
            expected
        );
    }

    #[test]
    fn vec3_multiply() {
        let expected = Vec3::new(10.0, 40.0, 90.0);
        assert_eq!(
            Vec3::new(1.0, 2.0, 3.0) * Vec3::new(10.0, 20.0, 30.0),
            expected
        );
    }

    #[test]
    fn vec3_neg() {
        let expected = Vec3::new(-1.0, -2.0, -3.0);
        assert_eq!(-Vec3::new(1.0, 2.0, 3.0), expected);
    }
}
