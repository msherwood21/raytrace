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
