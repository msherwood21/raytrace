//! A viewport through which the raytracer can create an image.

use crate::ray;
use crate::rtweekend;
use crate::vec3;

pub struct Camera {
    origin: vec3::Point3,
    lower_left_corner: vec3::Point3,
    horizontal: vec3::Point3,
    vertical: vec3::Point3,
    u: vec3::Vec3,
    v: vec3::Vec3,
    w: vec3::Vec3,
    lens_radius: f64,
}

impl Camera {
    /// Creates a new Camera object.
    ///
    /// `vfov` stands for vertical field-of-view in degrees.
    pub fn new(
        lookfrom: vec3::Point3,
        lookat: vec3::Point3,
        vup: vec3::Point3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let theta = rtweekend::degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        //- This section is a hack to make translating between the C++ nicer
        let origin_calc = lookfrom;
        let w_calc = vec3::unit_vector(origin_calc - lookat);
        let u_calc = vec3::unit_vector(vec3::cross(&vup, &w_calc));
        let v_calc = vec3::cross(&w_calc, &u_calc);
        let horizontal_calc = focus_dist * viewport_width * u_calc;
        let vertical_calc = focus_dist * viewport_height * v_calc;
        let lower_left_calc =
            origin_calc - horizontal_calc / 2.0 - vertical_calc / 2.0 - focus_dist * w_calc;

        Camera {
            origin: origin_calc,
            lower_left_corner: lower_left_calc,
            horizontal: horizontal_calc,
            vertical: vertical_calc,
            u: u_calc,
            v: v_calc,
            w: w_calc,
            lens_radius: aperture / 2.0,
        }
    }

    /// Returns a ray located from (s, t) on the camera plane emanating away from the viewer.
    ///
    /// The ray has been "defocused" or had depth-of-field processing applied to it.
    pub fn get_ray(&self, s: f64, t: f64) -> ray::Ray {
        let radian = self.lens_radius * vec3::random_in_unit_disk();
        let offset = self.u * radian.x() + self.v * radian.y();

        ray::Ray {
            orig: self.origin + offset,
            dir: self.lower_left_corner + s * self.horizontal + t * self.vertical
                - self.origin
                - offset,
        }
    }
}
