//! An object a ray can encounter in a scene.

use crate::material;
use crate::ray;
use crate::vec3;
use std::rc;

/// The location and associated metadata of a `ray::Ray` intersecting a `Hittable` object.
pub struct HitRecord {
    /// The point in 3D space where a ray hit an object.
    pub p: vec3::Point3,
    /// The normal vector from the point on the object where a ray hit.
    pub normal: vec3::Vec3,
    /// A shared pointer to a material description object
    pub mat_ptr: Option<rc::Rc<dyn material::Material>>,
    /// The value to use to complete the equation P(t) = A + tb.
    ///
    /// P(t) is the point in 3D space, A is the ray origin and b is the ray direction.
    pub t: f64,
    /// `true` if the ray hit the outside of the object. `false` if the ray hit the inside.
    pub front_face: bool,
}

/// Defines an object a ray can encounter in a scene.
pub trait Hittable {
    /// Returns true if the object has been hit.
    ///
    /// `t_min` and `t_max` specify the upper and lower bounds of the function P(t) = A + tb where
    /// P(t) is a point in 3D space, A is the ray origin and b is the ray direction.
    ///
    /// `rec` is filled with the appropriate details if the object has been hit.
    fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            p: vec3::Point3::new(),
            normal: vec3::Vec3::new(),
            mat_ptr: None,
            t: 0.0,
            front_face: false,
        }
    }

    /// Updates the `front_face` and `normal` members based on `r` and `outward_normal`.
    #[inline]
    pub fn set_face_normal(&mut self, r: &ray::Ray, outward_normal: &vec3::Vec3) {
        self.front_face = vec3::dot(&r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}
