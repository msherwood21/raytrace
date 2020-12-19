use std::clone;
use std::marker;
use std::ops;

pub struct Vec3 {
    pub e: [f64; 3],
}

impl Vec3 {
    //- Unless otherwise noted only functions currently needed are
    //  implemented

    //- vec3() : e{0, 0, 0} {}
    //- Implemented through positional arguments
    //- vec3(double e0, double e1, double e2)

    pub fn x(&self) -> f64 {
        self.e[0]
    }
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    pub fn z(&self) -> f64 {
        self.e[2]
    }

    //- vec3 operator-() const;
    //- double operator[](int i) const;
    //- double& operator[](int i);
    //- vec3& operator+=(const vec3 &v);
    //- vec3& operator*=(const double t);
    //- vec3& operator/=(const double t);

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        (self.e[0] * self.e[0]) + (self.e[1] * self.e[1]) + (self.e[2] * self.e[2])
    }
}

//- Aliased types
pub type Point3 = Vec3;
pub type Color = Vec3;

//- Traits
impl marker::Copy for Vec3 {}

impl clone::Clone for Vec3 {
    fn clone(&self) -> Vec3 {
        *self
    }
}

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

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            e: [self * rhs.e[0], self * rhs.e[1], self * rhs.e[2]],
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        rhs * self
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn div(self, rhs: f64) -> Self::Output {
        (f64::from(1) / rhs) * self
    }
}

//- Utility functions
#[inline]
pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}
