//! Helper functions and definitions for use throughout the code base.

use rand::Rng;

/// Ported from the book. Use f64::INFINITY for future code.
pub const INFINITY: f64 = f64::INFINITY;
/// Ported from the book. Use f64::consts::PI for future code.
pub const PI: f64 = 3.1415926535897932385;

/// Returns the radian value from the passed in degree.
#[inline]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

/// Returns a randomly selected `f64` from the range 0.0 - 1.0.
#[inline]
pub fn random_double() -> f64 {
    rand::thread_rng().gen_range(0.0, 1.0)
}

/// Returns a randomly selected `f64` from the range `min` - `max`.
#[inline]
pub fn random_double_in_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}

/// Returns `x`, `min` or `max` depending on whether `x` is within the range of `min` and `max`.
#[inline]
pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    return x;
}
