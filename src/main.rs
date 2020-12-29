mod camera;
mod color;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;
use std::env;
use std::io;
use std::rc::Rc;

fn ray_color(r: &ray::Ray, world: &dyn hittable::Hittable, depth: i32) -> vec3::Color {
    let mut rec = hittable::HitRecord::new();

    if depth <= 0 {
        return vec3::Color { e: [0.0, 0.0, 0.0] };
    }

    if world.hit(r, 0.001, rtweekend::INFINITY, &mut rec) {
        let mut scattered = ray::Ray::new();
        let mut attenuation = vec3::Color::new();

        match &rec.mat_ptr {
            Some(val) => {
                if val.scatter(r, &rec, &mut attenuation, &mut scattered) {
                    return attenuation * ray_color(&scattered, world, depth - 1);
                }
            }
            None => return vec3::Color::new(),
        }

        return vec3::Color::new();
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

    let material_ground =
        Rc::<material::Lambertian>::new(material::Lambertian::new(&vec3::Color {
            e: [0.8, 0.8, 0.0],
        }));
    let material_center =
        Rc::<material::Lambertian>::new(material::Lambertian::new(&vec3::Color {
            e: [0.1, 0.2, 0.5],
        }));
    let material_left = Rc::<material::Dielectric>::new(material::Dielectric::new(1.5));
    let material_left_two = Rc::<material::Dielectric>::new(material::Dielectric::new(1.5));
    let material_right = Rc::<material::Metal>::new(material::Metal::new(
        &vec3::Color { e: [0.8, 0.6, 0.2] },
        0.0,
    ));

    world.add(Rc::new(sphere::Sphere {
        center: vec3::Point3 {
            e: [0.0, -100.5, -1.0],
        },
        radius: 100.0,
        mat_ptr: material_ground,
    }));
    world.add(Rc::new(sphere::Sphere {
        center: vec3::Point3 {
            e: [0.0, 0.0, -1.0],
        },
        radius: 0.5,
        mat_ptr: material_center,
    }));
    world.add(Rc::new(sphere::Sphere {
        center: vec3::Point3 {
            e: [-1.0, 0.0, -1.0],
        },
        radius: 0.5,
        mat_ptr: material_left,
    }));
    world.add(Rc::new(sphere::Sphere {
        center: vec3::Point3 {
            e: [-1.0, 0.0, -1.0],
        },
        radius: -0.4,
        mat_ptr: material_left_two,
    }));
    world.add(Rc::new(sphere::Sphere {
        center: vec3::Point3 {
            e: [1.0, 0.0, -1.0],
        },
        radius: 0.5,
        mat_ptr: material_right,
    }));

    //- Camera
    let cam = camera::Camera::new();

    //- Render
    //    Header
    println!("P3\n{} {}\n255", image_width, image_height);

    //    Body
    for j in (0..image_height).rev() {
        //- Progress bar
        eprint!("\rScanlines remaining: {:#04}", j);

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
