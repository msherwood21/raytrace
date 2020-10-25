use std::io;
mod raytrace;

fn main() {
    //- Image
    let image_width: i32 = 256;
    let image_height: i32 = 256;

    //- Render
    //-   Header
    println!("P3\n{} {}\n255", image_width, image_height);

    //-   Body
    for j in (0..image_height).rev() {
        //- Progress bar
        eprint!("\rScanlines remaining: {}", j);

        for i in 0..image_width {
            let pixel_color = raytrace::vec3::Color {
                one: f64::from(i) / f64::from(image_width - 1),
                two: f64::from(j) / f64::from(image_height - 1),
                three: 0.25,
            };
            raytrace::vec3::write_color(&mut io::stdout(), pixel_color);
        }
    }

    eprintln!("\nDone.");
}
