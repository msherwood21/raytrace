use crate::hittable;
use crate::ray;
use crate::rtweekend;
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
        Metal {
            albedo: *a,
            fuzz: if f < 1.0 { f } else { 1.0 },
        }
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

pub struct Dielectric {
    pub ref_idx: f64,
}

impl Dielectric {
    pub fn new(ri: f64) -> Dielectric {
        Dielectric { ref_idx: ri }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &ray::Ray,
        rec: &hittable::HitRecord,
        attenuation: &mut vec3::Color,
        scattered: &mut ray::Ray,
    ) -> bool {
        *attenuation = vec3::Color { e: [1.0, 1.0, 1.0] };
        let etai_over_etat = if rec.front_face {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };

        let unit_direction = vec3::unit_vector(r_in.direction());
        let cos_theta = vec3::dot(&-unit_direction, &rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        if etai_over_etat * sin_theta > 1.0 {
            let reflected = vec3::reflect(&unit_direction, &rec.normal);
            *scattered = ray::Ray {
                orig: rec.p,
                dir: reflected,
            };

            return true;
        }
        let reflect_prob = schlick(cos_theta, etai_over_etat);
        if rtweekend::random_double() < reflect_prob {
            let reflected = vec3::reflect(&unit_direction, &rec.normal);
            *scattered = ray::Ray{ orig: rec.p, dir: reflected };

            return true;
        }

        let refracted = vec3::refract(&unit_direction, &rec.normal, etai_over_etat);
        *scattered = ray::Ray {
            orig: rec.p,
            dir: refracted,
        };

        return true;
    }
}

//- double schlick(double cosine, double ref_idx) 
pub fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;

    return r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0);
}