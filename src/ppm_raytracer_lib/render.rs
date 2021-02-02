use crate::{color, hittable, hittable_list, material, ray, rtweekend, scene_data, sphere, vec3};
use std::{io, rc::Rc};

/// Starts or stops rendering operations.
///
/// For the moment this function is a blocking call to be run on a single
/// thread. In the future this module will be its own thread and this function
/// will be the interface.
pub fn render_loop(data: scene_data::SceneData) {
    let image_height = (f64::from(data.image_width) / data.aspect_ratio) as i32;
    let world = random_scene(-11, -11);

    eprintln!(
        "Creating image with a resolution of {}x{}",
        data.image_width, image_height
    );

    // Header
    println!("P3\n{} {}\n255", data.image_width, image_height);

    // Body
    for j in (0..image_height).rev() {
        //- Progress bar
        eprint!("\rScanlines remaining: {:#04}", j);

        for i in 0..data.image_width {
            if i % 100 == 0 {
                eprint!(".");
            }

            render(&data, &world, image_height, j, i);
        }
    }

    eprintln!("\nDone.");
}

/// Executes one iteration of the raytracer.
fn render(
    data: &scene_data::SceneData,
    world: &hittable_list::HittableList,
    image_height: i32,
    scanline: i32,
    column: u32,
) {
    let mut pixel_color = vec3::Color { e: [0.0, 0.0, 0.0] };

    for _ in 0..data.samples_per_pixel {
        let u = (f64::from(column) + rtweekend::random_double()) / f64::from(data.image_width - 1);
        let v = (f64::from(scanline) + rtweekend::random_double()) / f64::from(image_height - 1);
        let r = data.camera.get_ray(u, v);
        pixel_color += ray_color(&r, world, data.max_ray_hits);
    }

    color::write_color(&mut io::stdout(), pixel_color, data.samples_per_pixel);
}

/// Returns a `vec3::Color` specifying the color of ray `r`.
///
/// `world` is a collection of all objects in the world. `depth` specifies how many times this
/// function can be called recursively before returning a default `vec3::Color`.
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

/// Creates a random world of objects with matte, metal and dielectric material surfaces.
///
/// `x` and `z` are used to determine the number of spheres and their location. The number of
/// spheres is x.abs() * 2 if the value is negative or x if the value is positive.
fn random_scene(x: i32, z: i32) -> hittable_list::HittableList {
    let mut world = hittable_list::HittableList::new();
    let x_min = if x.abs() == x { 0 } else { x };
    let x_max = if x < 0 { x.abs() } else { 0 };
    let z_min = if z.abs() == z { 0 } else { z };
    let z_max = if z < 0 { z.abs() } else { 0 };

    let ground_material =
        Rc::<material::Lambertian>::new(material::Lambertian::new(&vec3::Color {
            e: [0.5, 0.5, 0.5],
        }));
    world.add(Rc::new(sphere::Sphere {
        center: vec3::Point3 {
            e: [0.0, -1000.0, 0.0],
        },
        radius: 1000.0,
        mat_ptr: ground_material,
    }));

    for a in x_min..x_max {
        for b in z_min..z_max {
            let choose_mat = rtweekend::random_double();
            let center = vec3::Point3 {
                e: [
                    f64::from(a) + 0.9 * rtweekend::random_double(),
                    0.2,
                    f64::from(b) + 0.9 * rtweekend::random_double(),
                ],
            };

            if (center - vec3::Point3 { e: [4.0, 0.2, 0.0] }).length() > 0.9 {
                if choose_mat < 0.8 {
                    //- diffuse
                    let albedo = vec3::Color::random() * vec3::Color::random();
                    let sphere_material = Rc::new(material::Lambertian::new(&albedo));
                    world.add(Rc::new(sphere::Sphere {
                        center: center,
                        radius: 0.2,
                        mat_ptr: sphere_material,
                    }));
                } else if choose_mat < 0.95 {
                    //- metal
                    let albedo = vec3::Color::random_range(0.5, 1.0);
                    let fuzz = rtweekend::random_double_in_range(0.0, 0.5);
                    let sphere_material = Rc::new(material::Metal::new(&albedo, fuzz));
                    world.add(Rc::new(sphere::Sphere {
                        center: center,
                        radius: 0.2,
                        mat_ptr: sphere_material,
                    }));
                } else {
                    //- glass
                    let sphere_material = Rc::new(material::Dielectric::new(1.5));
                    world.add(Rc::new(sphere::Sphere {
                        center: center,
                        radius: 0.2,
                        mat_ptr: sphere_material,
                    }));
                }
            }
        }
    }

    let material1 = Rc::new(material::Dielectric::new(1.5));
    world.add(Rc::new(sphere::Sphere {
        center: vec3::Point3 { e: [0.0, 1.0, 0.0] },
        radius: 1.0,
        mat_ptr: material1,
    }));

    let material2 = Rc::new(material::Lambertian::new(&vec3::Color {
        e: [0.4, 0.2, 0.1],
    }));
    world.add(Rc::new(sphere::Sphere {
        center: vec3::Point3 {
            e: [-4.0, 1.0, 0.0],
        },
        radius: 1.0,
        mat_ptr: material2,
    }));

    let material3 = Rc::new(material::Metal::new(
        &vec3::Color { e: [0.7, 0.6, 0.5] },
        0.0,
    ));
    world.add(Rc::new(sphere::Sphere {
        center: vec3::Point3 { e: [4.0, 1.0, 0.0] },
        radius: 1.0,
        mat_ptr: material3,
    }));

    return world;
}
