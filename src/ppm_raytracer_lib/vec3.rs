//! Defines the `Vec3` type and its associated functions for use with 3D geometry and color.

use crate::rtweekend;
use std::clone;
use std::marker;
use std::ops;

pub struct Vec3 {
    pub e: [f64; 3],
}

impl Vec3 {
    pub fn new() -> Vec3 {
        Vec3 { e: [0.0, 0.0, 0.0] }
    }

    /// Returns the x in an (x,y,z) coordinate.
    pub fn x(&self) -> f64 {
        self.e[0]
    }

    /// Returns the y in an (x,y,z) coordinate.
    pub fn y(&self) -> f64 {
        self.e[1]
    }

    /// Returns the z in an (x,y,z) coordinate.
    pub fn z(&self) -> f64 {
        self.e[2]
    }

    /// Returns the length of the vector from the point of origin.
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    /// Returns the length squared of the vector from the point of origin.
    pub fn length_squared(&self) -> f64 {
        (self.e[0] * self.e[0]) + (self.e[1] * self.e[1]) + (self.e[2] * self.e[2])
    }

    /// Returns a `Vec3` with randomly assigned values ranging from 0.0 to 1.0.
    #[inline]
    pub fn random() -> Vec3 {
        Vec3 {
            e: [
                rtweekend::random_double(),
                rtweekend::random_double(),
                rtweekend::random_double(),
            ],
        }
    }

    /// Returns a `Vec3` with randomly assigned values ranging from `min` to `max`.
    #[inline]
    pub fn random_range(min: f64, max: f64) -> Vec3 {
        Vec3 {
            e: [
                rtweekend::random_double_in_range(min, max),
                rtweekend::random_double_in_range(min, max),
                rtweekend::random_double_in_range(min, max),
            ],
        }
    }
}

/// Represents a point in 3D space.
pub type Point3 = Vec3;
/// Represents an RGB color.
pub type Color = Vec3;


impl marker::Copy for Vec3 {}

impl clone::Clone for Vec3 {
    fn clone(&self) -> Vec3 {
        *self
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn add(self, rhs: Self::Output) -> Self::Output {
        Self::Output {
            e: [
                self.e[0] + rhs.e[0],
                self.e[1] + rhs.e[1],
                self.e[2] + rhs.e[2],
            ],
        }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn sub(self, rhs: Self::Output) -> Self::Output {
        Vec3 {
            e: [
                self.e[0] - rhs.e[0],
                self.e[1] - rhs.e[1],
                self.e[2] - rhs.e[2],
            ],
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            e: [
                self.e[0] * rhs.e[0],
                self.e[1] * rhs.e[1],
                self.e[2] * rhs.e[2],
            ],
        }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            e: [self * rhs.e[0], self * rhs.e[1], self * rhs.e[2]],
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        rhs * self
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn div(self, rhs: f64) -> Self::Output {
        (f64::from(1) / rhs) * self
    }
}

/// Returns the dot product of two passed in vectors.
#[inline]
pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    (u.e[0] * v.e[0]) + (u.e[1] * v.e[1]) + (u.e[2] * v.e[2])
}

/// Returns the cross product of two passed in vectors.
#[inline]
pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3 {
        e: [
            u.e[1] * v.e[2] - u.e[2] * v.e[1],
            u.e[2] * v.e[0] - u.e[0] * v.e[2],
            u.e[0] * v.e[1] - u.e[1] * v.e[0],
        ],
    }
}

/// Returns the unit vector of the passed in vector.
#[inline]
pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

/// Returns a `Vec3` whose squared length is less than 1.0.
pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random_range(-1.0, 1.0);
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

/// Returns a random `Vec3` following a Lambertian distribution.
pub fn random_unit_vector() -> Vec3 {
    let a = rtweekend::random_double_in_range(0.0, 2.0 * rtweekend::PI);
    let z = rtweekend::random_double_in_range(-1.0, 1.0);
    let r = (1.0 - z * z).sqrt();
    Vec3 {
        e: [r * a.cos(), r * a.sin(), z],
    }
}

/// Returns a reflected `Vec3` satisfying the equation v + 2b.
/// 
/// `n` is a unit vector at which `v` is directed. b can be restated as the dot product of `v` and
/// `n`.
pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2.0 * dot(v, n) * *n
}

/// Returns a refracted direction from the original direction represented by `uv`.
/// 
/// `uv` stands for unit direction, `n` stands for normal, and etai_over_etat represents the
/// refractive property of the surface.
pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    //- &-* is the strangest symbol combo I've seen...Pass by reference
    //  the negated Vec3 created from the reference uv.
    let cos_theta = dot(&-*uv, n);
    let r_out_perp = etai_over_etat * (*uv + cos_theta * *n);
    let r_out_parallel = -((1.0 - r_out_perp.length_squared()).abs().sqrt()) * *n;

    r_out_perp + r_out_parallel
}

/// Returns a `Vec3` whose squared length is less than 1.0 and z coordinate is 0.0.
pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3 {
            e: [
                rtweekend::random_double_in_range(-1.0, 1.0),
                rtweekend::random_double_in_range(-1.0, 1.0),
                0.0,
            ],
        };
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}
