mod color;
mod hittable;
mod ray;
mod sphere;
mod vec3;
use std::io;

fn hit_sphere(center: &vec3::Point3, radius: f64, r: &ray::Ray) -> f64 {
    let oc: vec3::Vec3 = r.origin() - *center;
    let a = r.direction().length_squared();
    let half_b = vec3::dot(&oc, &r.direction());
    let c = oc.length_squared() - (radius * radius);
    let discriminant = (half_b * half_b) - (a * c);

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-half_b - discriminant.sqrt()) / a;
    }
}

fn ray_color(r: &ray::Ray) -> vec3::Color {
    let mut t = hit_sphere(
        &vec3::Point3 {
            e: [0.0, 0.0, -1.0],
        },
        0.5,
        &r,
    );
    if t > 0.0 {
        let n: vec3::Vec3 = vec3::unit_vector(
            r.at(t)
                - vec3::Vec3 {
                    e: [0.0, 0.0, -1.0],
                },
        );
        return 0.5
            * vec3::Color {
                e: [n.x() + 1.0, n.y() + 1.0, n.z() + 1.0],
            };
    }

    let unit_direction = vec3::unit_vector(r.direction());
    t = 0.5 * (unit_direction.y() + 1.0);

    (1.0 - t) * vec3::Color { e: [1.0, 1.0, 1.0] } + t * vec3::Color { e: [0.5, 0.7, 1.0] }
}

fn main() {
    //- Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: i32 = 400;
    let image_height: i32 = (f64::from(image_width) / aspect_ratio) as i32;

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
            let pixel_color = ray_color(&r);
            color::write_color(&mut io::stdout(), pixel_color);
        }
    }

    eprintln!("\nDone.");
}
