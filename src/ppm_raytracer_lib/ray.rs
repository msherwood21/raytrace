//! Data and functions related to the "ray" part of ray tracing.

use crate::vec3;

/// A ray in 3D space determined by a point of `orig`in and a `dir`ection.
pub struct Ray {
    pub orig: vec3::Point3,
    pub dir: vec3::Vec3,
}

impl Ray {
    pub fn new() -> Ray {
        Ray {
            orig: vec3::Vec3::new(),
            dir: vec3::Vec3::new(),
        }
    }

    /// Returns the origin of the ray.
    pub fn origin(&self) -> vec3::Point3 {
        self.orig
    }

    /// Returns the direction of the ray.
    pub fn direction(&self) -> vec3::Vec3 {
        self.dir
    }

    /// Returns a point along the ray given by the time `t`.
    pub fn at(&self, t: f64) -> vec3::Point3 {
        self.orig + (t * self.dir)
    }
}
