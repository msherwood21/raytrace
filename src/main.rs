mod camera;
mod color;
mod hittable;
mod hittable_list;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;
use std::env;
use std::io;
use std::rc;

fn ray_color(r: &ray::Ray, world: &dyn hittable::Hittable, depth: i32) -> vec3::Color {
    let mut rec = hittable::HitRecord::new();

    if depth <= 0 { return vec3::Color{ e: [0.0, 0.0, 0.0] } }

    if world.hit(r, 0.001, rtweekend::INFINITY, &mut rec) {
        let target = rec.p + vec3::random_in_hemisphere(&rec.normal);
        return 0.5 * ray_color(&ray::Ray{ orig: rec.p, dir: target - rec.p }, world, depth - 1);
    }

    let unit_direction = vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * vec3::Color { e: [1.0, 1.0, 1.0] } + t * vec3::Color { e: [0.5, 0.7, 1.0] }
}

fn main() {
    //- Image
    let mut image_width: u32 = 400;
    let mut arg_iter = env::args().peekable();
    while arg_iter.peek() != None {
        let opt = arg_iter
            .next()
            .expect("Invalid iterator value after initial peek");

        if opt == "--width" || opt == "-w" {
            image_width = arg_iter
                .next()
                .expect("You must pass an argument to the width argument")
                .parse::<u32>()
                .expect("Invalid value with width option. Use --width <u32>.");
        }
    }

    let aspect_ratio = 16.0 / 9.0;
    let image_height: i32 = (f64::from(image_width) / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    eprintln!(
        "Creating image with a resolution of {}x{}",
        image_width, image_height
    );

    //- World
    let mut world = hittable_list::HittableList::new();
    world.add(rc::Rc::new(sphere::Sphere {
        center: vec3::Point3 {
            e: [0.0, 0.0, -1.0],
        },
        radius: 0.5,
    }));
    world.add(rc::Rc::new(sphere::Sphere {
        center: vec3::Point3 {
            e: [0.0, -100.5, -1.0],
        },
        radius: 100.0,
    }));

    //- Camera
    let cam = camera::Camera::new();

    //- Render
    //    Header
    println!("P3\n{} {}\n255", image_width, image_height);

    //    Body
    for j in (0..image_height).rev() {
        //- Progress bar
        eprint!("\rScanlines remaining: {}", j);

        for i in 0..image_width {
            let mut pixel_color = vec3::Color { e: [0.0, 0.0, 0.0] };
            for _s in 0..samples_per_pixel {
                let u = (f64::from(i) + rtweekend::random_double()) / f64::from(image_width - 1);
                let v = (f64::from(j) + rtweekend::random_double()) / f64::from(image_height - 1);
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, max_depth);
            }
            color::write_color(&mut io::stdout(), pixel_color, samples_per_pixel);
        }
    }

    eprintln!("\nDone.");
}
