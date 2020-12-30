use crate::rtweekend;
use crate::vec3;

pub fn write_color(out: &mut dyn std::io::Write, pixel_color: vec3::Color, samples_per_pixel: i32) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    // Divide the color by the number of samples and gamma-correct for gamma=2.0.
    let scale = 1.0 / f64::from(samples_per_pixel);
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    // Write the translated [0,255] value of each color component.
    write!(
        out,
        "{} {} {}\n",
        (256.0 * rtweekend::clamp(r, 0.0, 0.999)) as i32,
        (256.0 * rtweekend::clamp(g, 0.0, 0.999)) as i32,
        (256.0 * rtweekend::clamp(b, 0.0, 0.999)) as i32
    )
    .expect("failed to output color line");
}
