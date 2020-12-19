use crate::vec3;

pub fn write_color(out: &mut dyn std::io::Write, pixel_color: vec3::Color) {
    // Write the translated [0,255] value of each color component
    write!(
        out,
        "{} {} {}\n",
        (255.999 * pixel_color.x()) as i32,
        (255.999 * pixel_color.y()) as i32,
        (255.999 * pixel_color.z()) as i32
    )
    .expect("failed to output color line");
}