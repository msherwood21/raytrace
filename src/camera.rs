use crate::ray;
use crate::vec3;

pub struct Camera {
    origin: vec3::Point3,
    lower_left_corner: vec3::Point3,
    horizontal: vec3::Point3,
    vertical: vec3::Point3,
}

impl Camera {
    //- camera()
    pub fn new() -> Camera {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin_calc = vec3::Point3 { e: [0.0, 0.0, 0.0] };
        let horizontal_calc = vec3::Point3 {
            e: [viewport_width, 0.0, 0.0],
        };
        let vertical_calc = vec3::Point3 {
            e: [0.0, viewport_height, 0.0],
        };
        let lower_left_calc = origin_calc
            - horizontal_calc / 2.0
            - vertical_calc / 2.0
            - vec3::Vec3 {
                e: [0.0, 0.0, focal_length],
            };
        Camera {
            origin: origin_calc,
            lower_left_corner: lower_left_calc,
            horizontal: horizontal_calc,
            vertical: vertical_calc,
        }
    }

    //- ray get_ray(double u, double v) const
    pub fn get_ray(&self, u: f64, v: f64) -> ray::Ray {
        ray::Ray {
            orig: self.origin,
            dir: self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        }
    }
}
