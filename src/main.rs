mod color;
mod hittable;
mod hittable_list;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;
use std::io;
use std::rc;

fn ray_color(r: &ray::Ray, world: &dyn hittable::Hittable) -> vec3::Color {
    let mut rec = hittable::HitRecord::new();
    if world.hit(r, 0.0, rtweekend::INFINITY, &mut rec) {
        return 0.5 * (rec.normal + vec3::Color { e: [1.0, 1.0, 1.0] });
    }
    let unit_direction = vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);

    (1.0 - t) * vec3::Color { e: [1.0, 1.0, 1.0] } + t * vec3::Color { e: [0.5, 0.7, 1.0] }
}

fn main() {
    //- Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: i32 = 400;
    let image_height: i32 = (f64::from(image_width) / aspect_ratio) as i32;

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
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = vec3::Point3 { e: [0.0, 0.0, 0.0] };
    let horizontal = vec3::Vec3 {
        e: [viewport_width, 0.0, 0.0],
    };
    let vertical = vec3::Vec3 {
        e: [0.0, viewport_height, 0.0],
    };
    let lower_left_corner = origin
        - horizontal / 2.0
        - vertical / 2.0
        - vec3::Vec3 {
            e: [0.0, 0.0, focal_length],
        };

    //- Render
    //    Header
    println!("P3\n{} {}\n255", image_width, image_height);

    //    Body
    for j in (0..image_height).rev() {
        //- Progress bar
        eprint!("\rScanlines remaining: {}", j);

        for i in 0..image_width {
            let u = f64::from(i) / f64::from(image_width - 1);
            let v = f64::from(j) / f64::from(image_height - 1);
            let r = ray::Ray {
                orig: origin,
                dir: lower_left_corner + u * horizontal + v * vertical - origin,
            };
            let pixel_color = ray_color(&r, &world);
            color::write_color(&mut io::stdout(), pixel_color);
        }
    }

    eprintln!("\nDone.");
}
