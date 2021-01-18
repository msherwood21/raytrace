use crate::camera;
use crate::hittable_list;
use crate::vec3;

/// World data needed by the renderer.
///
/// Not intended to be modified by the caller.
pub struct SceneData {
    pub image_width: u32,
    pub aspect_ratio: f64,
    pub samples_per_pixel: i32,
    pub max_ray_hits: i32,
    pub world: hittable_list::HittableList,
    pub camera: camera::Camera,
}

impl SceneData {
    pub fn new() -> SceneData {
        SceneData {
            image_width: 250,
            aspect_ratio: 16.0 / 9.0,
            samples_per_pixel: 1,
            max_ray_hits: 50,
            world: hittable_list::HittableList::new(),
            camera: camera::Camera::new(
                vec3::Point3 { e: [0.0, 1.0, -2.0] },
                vec3::Point3 { e: [0.0, 0.0, 0.0] },
                vec3::Point3 { e: [0.0, 0.0, 0.0] },
                20.0,
                0.0,
                0.0,
                0.0,
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Confirms SceneData is created with sensible default data.
    #[test]
    fn default_init_test() {
        let data = SceneData::new();

        assert_eq!(data.image_width, 250);
        assert_eq!(data.aspect_ratio, 16.0 / 9.0);
        assert_eq!(data.samples_per_pixel, 1);
        assert_eq!(data.max_ray_hits, 50);
    }
}
