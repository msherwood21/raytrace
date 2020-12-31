//! Helper functions related to color operations.

use crate::rtweekend;
use crate::vec3;

/// Writes the specified color to the supplied output stream.
///
/// Each value in `vec3::Color` is expected to have been added to `samples_per_pixel` times. If
/// you've just created a `vec3::Color` struct `samples_per_pixel` should be `1`.
///
/// The color output is 256 bit RGB and uses a gamma value of 1/2.
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
