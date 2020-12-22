use crate::hittable;
use crate::ray;
use crate::vec3;

pub struct Sphere {
    pub center: vec3::Point3,
    pub radius: f64,
}

impl Sphere {
    //- sphere() {}
    // pub fn new() -> Sphere {
    //     Sphere {
    //         center: vec3::Point3::new(),
    //         radius: 0.0,
    //     }
    // }

    //- sphere(point3 cen, double r) : center(cen), radius(r) {}
    //  Implemented through positional arguments
}

impl hittable::Hittable for Sphere {
    fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64, rec: &mut hittable::HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = vec3::dot(&oc, &r.direction());
        let c = oc.length_squared() - (self.radius * self.radius);
        let discriminant = (half_b * half_b) - (a * c);

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let mut temp = (-half_b - root) / a;

            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.at(rec.t);
                let outward_normal = (rec.p - self.center) / self.radius;
                rec.set_face_normal(r, &outward_normal);
                return true;
            }

            temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.at(rec.t);
                let outward_normal = (rec.p - self.center) / self.radius;
                rec.set_face_normal(r, &outward_normal);
                return true;
            }
        }

        return false;
    }
}
