use crate::vec3;

pub struct Ray {
    pub orig: vec3::Point3,
    pub dir: vec3::Vec3,
}

impl Ray {
    //- Implemented through positional arguments?
    //- NOTE: Yes, but only because we implemented the Copy trait for Vec3.
    //      Otherwise the positional argument constructor would actually be
    //      a move constructor.
    //  ray(const point3& origin, const vec3& direction)
    //      : orig(origin), dir(direction)

    pub fn new() -> Ray {
        Ray {
            orig: vec3::Vec3::new(),
            dir: vec3::Vec3::new(),
        }
    }

    pub fn origin(&self) -> vec3::Point3 {
        self.orig
    }

    pub fn direction(&self) -> vec3::Vec3 {
        self.dir
    }

    pub fn at(&self, t: f64) -> vec3::Point3 {
        self.orig + (t * self.dir)
    }
}
