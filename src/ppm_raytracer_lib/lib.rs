//! An implementation of the raytracer from
//! [Ray Tracing In One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html)
//! in Rust.

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod render;
mod rtweekend;
mod scene_data;
mod sphere;
mod vec3;

pub mod command {
    use crate::camera;
    use crate::render;
    use crate::scene_data;
    use crate::vec3;
    use std::env;
    use std::sync::Once;

    static mut SCENE: scene_data::SceneData = scene_data::SceneData::new();
    static INIT: Once = Once::new();

    /// Initializes scene data and private data sctructures.
    fn init() -> scene_data::SceneData {
        let mut data = scene_data::SceneData::new();

        //- Image
        data.image_width = 1200;
        let mut arg_iter = env::args().peekable();
        while arg_iter.peek() != None {
            let opt = arg_iter
                .next()
                .expect("Invalid iterator value after initial peek");

            if opt == "--width" || opt == "-w" {
                data.image_width = arg_iter
                    .next()
                    .expect("You must pass an argument to the width argument")
                    .parse::<u32>()
                    .expect("Invalid value with width option. Use --width <u32>.");
            }
        }

        data.aspect_ratio = 3.0 / 2.0;
        data.samples_per_pixel = 500;
        data.max_ray_hits = 50;

        //- Camera
        let lookfrom = vec3::Point3 {
            e: [13.0, 2.0, 3.0],
        };
        let lookat = vec3::Point3 { e: [0.0, 0.0, 0.0] };
        let vup = vec3::Vec3 { e: [0.0, 1.0, 0.0] };
        let dist_to_focus = 10.0;
        let aperture = 0.1;

        data.camera = camera::Camera::new(
            lookfrom,
            lookat,
            vup,
            20.0,
            data.aspect_ratio,
            aperture,
            dist_to_focus,
        );

        data
    }

    /// Initiates an iteration of the rendering loop.
    pub fn start() {
        unsafe {
            INIT.call_once(|| {
                SCENE = init();
            });
            render::render_loop(SCENE);
        }
    }
}
