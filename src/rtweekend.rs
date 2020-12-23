use rand::Rng;

//- Using declarations from C++ are not applicable here

//- std::numeric_limits<double>::infinity();
pub const INFINITY: f64 = f64::INFINITY;
//- const double pi = 3.1415926535897932385;
// pub const PI: f64 = 3.1415926535897932385;

//- inline double degrees_to_radians(double degrees) 
// #[inline]
// pub fn degrees_to_radians(degrees: f64) -> f64 {
//     degrees * PI / 180.0
// }

//- inline double random_double()
#[inline]
pub fn random_double() -> f64 {
    rand::thread_rng().gen_range(0.0, 1.0)
}

//- inline double random_double(double min, double max)
// #[inline]
// pub fn random_double_in_range(min: f64, max: f64) -> f64 {
//     min + (max - min) * random_double()
// }

//- inline double clamp(double x, double min, double max)
#[inline]
pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min { return min; }
    if x > max { return max; }
    return x;
}

//- Pulling in common headers from C++ are not applicable here (I think?)