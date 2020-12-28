use crate::hittable;
use crate::ray;
use crate::vec3;

pub trait Material {
    fn scatter(
        &self,
        r_in: &ray::Ray,
        rec: &hittable::HitRecord,
        attenuation: &mut vec3::Color,
        scattered: &mut ray::Ray,
    ) -> bool;
}

pub struct Lambertian {
    pub albedo: vec3::Color,
}

impl Lambertian {
    pub fn new(a: &vec3::Color) -> Lambertian {
        Lambertian { albedo: *a }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &ray::Ray,
        rec: &hittable::HitRecord,
        attenuation: &mut vec3::Color,
        scattered: &mut ray::Ray,
    ) -> bool {
        let scatter_direction = rec.normal + vec3::random_unit_vector();
        *scattered = ray::Ray {
            orig: rec.p,
            dir: scatter_direction,
        };
        *attenuation = self.albedo;

        true
    }
}

pub struct Metal {
    pub albedo: vec3::Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(a: &vec3::Color, f: f64) -> Metal {
        Metal { albedo: *a, fuzz: if f < 1.0 { f } else { 1.0 } }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &ray::Ray,
        rec: &hittable::HitRecord,
        attenuation: &mut vec3::Color,
        scattered: &mut ray::Ray,
    ) -> bool {
        let reflected = vec3::reflect(&vec3::unit_vector(r_in.direction()), &rec.normal);
        *scattered = ray::Ray {
            orig: rec.p,
            dir: reflected + self.fuzz * vec3::random_in_unit_sphere(),
        };
        *attenuation = self.albedo;

        vec3::dot(&scattered.direction(), &rec.normal) > 0.0
    }
}
