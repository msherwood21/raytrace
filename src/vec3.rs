use crate::rtweekend;
use std::clone;
use std::marker;
use std::ops;

pub struct Vec3 {
    pub e: [f64; 3],
}

impl Vec3 {
    //- vec3() : e{0, 0, 0} {}
    pub fn new() -> Vec3 {
        Vec3 { e: [0.0, 0.0, 0.0] }
    }
    //- NOTE: The following is implemented through positional arguments
    //- vec3(double e0, double e1, double e2)

    //- double x() const { return e[0]; }
    pub fn x(&self) -> f64 {
        self.e[0]
    }
    //- double y() const { return e[1]; }
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    //- double z() const { return e[2]; }
    pub fn z(&self) -> f64 {
        self.e[2]
    }

    //- double operator[](int i) const { return e[i]; }
    //- double& operator[](int i) { return e[i]; }

    //- vec3& operator*=(const double t);
    //- vec3& operator/=(const double t);

    //- double length() const
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    //- double length_squared() const
    pub fn length_squared(&self) -> f64 {
        (self.e[0] * self.e[0]) + (self.e[1] * self.e[1]) + (self.e[2] * self.e[2])
    }

    //- inline static vec3 random()
    #[inline]
    pub fn random() -> Vec3 {
        Vec3 {
            e: [
                rtweekend::random_double(),
                rtweekend::random_double(),
                rtweekend::random_double(),
            ],
        }
    }

    //- inline static vec3 random(double min, double max)
    #[inline]
    pub fn random_range(min: f64, max: f64) -> Vec3 {
        Vec3 {
            e: [
                rtweekend::random_double_in_range(min, max),
                rtweekend::random_double_in_range(min, max),
                rtweekend::random_double_in_range(min, max),
            ],
        }
    }
}

//- Aliased types
pub type Point3 = Vec3;
pub type Color = Vec3;

//- Traits and utility functions
impl marker::Copy for Vec3 {}

impl clone::Clone for Vec3 {
    fn clone(&self) -> Vec3 {
        *self
    }
}

//- vec3 operator-() const { return vec3(-e[0], -e[1], -e[2]); }
impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

//- vec3& operator+=(const vec3 &v);
impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

//- inline vec3 operator+(const vec3 &u, const vec3 &v)
impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn add(self, rhs: Self::Output) -> Self::Output {
        Self::Output {
            e: [
                self.e[0] + rhs.e[0],
                self.e[1] + rhs.e[1],
                self.e[2] + rhs.e[2],
            ],
        }
    }
}

//- inline vec3 operator-(const vec3 &u, const vec3 &v)
impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn sub(self, rhs: Self::Output) -> Self::Output {
        Vec3 {
            e: [
                self.e[0] - rhs.e[0],
                self.e[1] - rhs.e[1],
                self.e[2] - rhs.e[2],
            ],
        }
    }
}

//- inline vec3 operator*(const vec3 &u, const vec3 &v)
impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            e: [
                self.e[0] * rhs.e[0],
                self.e[1] * rhs.e[1],
                self.e[2] * rhs.e[2],
            ],
        }
    }
}

//- inline vec3 operator*(double t, const vec3 &v)
impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            e: [self * rhs.e[0], self * rhs.e[1], self * rhs.e[2]],
        }
    }
}

//- inline vec3 operator*(const vec3 &v, double t)
impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        rhs * self
    }
}

//- inline vec3 operator/(vec3 v, double t)
impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn div(self, rhs: f64) -> Self::Output {
        (f64::from(1) / rhs) * self
    }
}

//- inline double dot(const vec3 &u, const vec3 &v)
#[inline]
pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    (u.e[0] * v.e[0]) + (u.e[1] * v.e[1]) + (u.e[2] * v.e[2])
}

//- inline vec3 cross(const vec3 &u, const vec3 &v)

//- inline vec3 unit_vector(vec3 v)
#[inline]
pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

//- vec3 random_in_unit_sphere()
pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random_range(-1.0, 1.0);
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

//- NOTE: The following function was commented out because it was replaced by
//      random_in_hemisphere() and we want to avoid compiler warnings.
//- vec3 random_unit_vector()
// pub fn random_unit_vector() -> Vec3 {
//     let a = rtweekend::random_double_in_range(0.0, 2.0 * rtweekend::PI);
//     let z = rtweekend::random_double_in_range(-1.0, 1.0);
//     let r = (1.0 - z * z).sqrt();
//     Vec3 {
//         e: [r * a.cos(), r * a.sin(), z],
//     }
// }

//- vec3 random_in_hemisphere(const vec3& normal)
pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if dot(&in_unit_sphere, normal) > 0.0 {
        return in_unit_sphere;
    } else {
        return -in_unit_sphere;
    }
}
