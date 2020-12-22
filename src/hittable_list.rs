use crate::hittable;
use crate::ray;
use std::rc;
use std::vec;

pub struct HittableList {
    objects: vec::Vec<rc::Rc<dyn hittable::Hittable>>,
}

impl HittableList {
    //- hittable_list() {}
    pub fn new() -> HittableList {
        HittableList { objects: vec::Vec::new() }
    }

    //- hittable_list(shared_ptr<hittable> object) { add(object); }
    //- NOTE: Call new() then add().

    //- void clear() { objects.clear(); }
    // pub fn clear(&mut self) {
    //     self.objects.clear();
    // }

    //- void add(shared_ptr<hittable> object) { objects.push_back(object); }
    pub fn add(&mut self, object: rc::Rc<dyn hittable::Hittable>) {
        self.objects.push(object);
    }
}

//- virtual bool hit(const ray& r, double tmin, double tmax, hit_record& rec) const override;
impl hittable::Hittable for HittableList {
    fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64, rec: &mut hittable::HitRecord) -> bool {
        let mut temp_rec = hittable::HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects[..] {
            //- A note about this block of code: The original C++ implementation is allocating
            //  space for the temp_rec variable so it can modify memory and, if the function
            //  was successful, set rec's data to that of temp_rec. This takes advantage of the
            //  implicit copy assignment operator so we have to explicitly implement the Copy
            //  trait on the HitRecord type.
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }

        hit_anything
    }
}
