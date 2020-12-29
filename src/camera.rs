use crate::ray;
use crate::rtweekend;
use crate::vec3;

pub struct Camera {
    origin: vec3::Point3,
    lower_left_corner: vec3::Point3,
    horizontal: vec3::Point3,
    vertical: vec3::Point3,
}

impl Camera {
    //- camera(
    //      point3 lookfrom,
    //      point3 lookat,
    //      vec3   vup,
    //      double vfov, // vertical field-of-view in degrees
    //      double aspect_ratio
    //  )
    pub fn new(
        lookfrom: vec3::Point3,
        lookat: vec3::Point3,
        vup: vec3::Point3,
        vfov: f64,
        aspect_ratio: f64,
    ) -> Camera {
        let theta = rtweekend::degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = vec3::unit_vector(lookfrom - lookat);
        let u = vec3::unit_vector(vec3::cross(&vup, &w));
        let v = vec3::cross(&w, &u);

        let lower_left_calc =
            lookfrom - (viewport_width * u) / 2.0 - (viewport_height * v) / 2.0 - w;
        Camera {
            origin: lookfrom,
            lower_left_corner: lower_left_calc,
            horizontal: viewport_width * u,
            vertical: viewport_height * v,
        }
    }

    //- ray get_ray(double s, double t) const
    pub fn get_ray(&self, s: f64, t: f64) -> ray::Ray {
        ray::Ray {
            orig: self.origin,
            dir: self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin,
        }
    }
}
