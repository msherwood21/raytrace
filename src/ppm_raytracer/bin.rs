//! ppm_raytracer is a binary shell around libraytracer. When executed the raytracer will run once
//! and output its image in the PPM image format to stdout.

use libraytracer;

/// Executes the raytracer once with default arguments.
/// 
/// As the library is refactored the implementation of this function will become the living docs
/// for using it.
fn main() {
    libraytracer::run();
}