use crate::ray;
use crate::vec3;
use std::clone;
use std::marker;

pub struct HitRecord {
    pub p: vec3::Point3,
    pub normal: vec3::Vec3,
    pub t: f64,
    pub front_face: bool,
}

pub trait Hittable {
    fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            p: vec3::Point3::new(),
            normal: vec3::Vec3::new(),
            t: 0.0,
            front_face: false,
        }
    }

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

//- Implementing the Copy trait for code differing from the C++ implementation. See
//  the trait implementation in hittable_list.rs.
impl marker::Copy for HitRecord {}

impl clone::Clone for HitRecord {
    fn clone(&self) -> HitRecord {
        *self
    }
}
