use crate::camera;

/// World data needed by the renderer.
///
/// Not intended to be modified by the caller.
#[derive(Clone, Copy, Debug)]
pub struct SceneData {
    pub image_width: u32,
    pub aspect_ratio: f64,
    pub samples_per_pixel: i32,
    pub max_ray_hits: i32,
    pub camera: camera::Camera,
}

impl SceneData {
    pub const fn new() -> SceneData {
        SceneData {
            image_width: 250,
            aspect_ratio: 1.7777777777777777, // 16 x 9 aspect ratio hack
            samples_per_pixel: 1,
            max_ray_hits: 50,
            camera: camera::Camera::busted_new(),
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
