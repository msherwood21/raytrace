//! A collection of `Hittable` objects and associated helper functions.

use crate::hittable;
use crate::ray;
use std::rc;
use std::vec;

/// A collection of `Hittable` objects.
pub struct HittableList {
    objects: vec::Vec<rc::Rc<dyn hittable::Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: vec::Vec::new(),
        }
    }

    //- void clear() { objects.clear(); }
    // pub fn clear(&mut self) {
    //     self.objects.clear();
    // }

    /// Add a reference counted pointer to the list of `Hittable` objects.
    ///
    /// If the calling function already has a reference to the object pointed to by `objects` then
    /// the parent object must use `Rc::clone()`.
    pub fn add(&mut self, object: rc::Rc<dyn hittable::Hittable>) {
        self.objects.push(object);
    }
}

impl hittable::Hittable for HittableList {
    /// Determines if any object in its collection is hit by ray `r`.
    ///
    /// If the ray can hit multiple objects in the collection the object closest to `t_min` is
    /// returned in `rec`.
    fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64, rec: &mut hittable::HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects[..] {
            let mut temp_rec = hittable::HitRecord::new();

            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }

        hit_anything
    }
}
