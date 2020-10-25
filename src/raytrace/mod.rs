pub mod vec3 {
    pub struct Vec3 {
        pub one: f64,
        pub two: f64,
        pub three: f64,
    }
    //- The following alias will be uncommented when needed. Written now to
    //  follow the book.
    // pub type Point3 = Vec3;
    pub type Color = Vec3;

    impl Vec3 {
        //- Unless otherwise noted only functions currently needed are
        //  implemented

        //- vec3() : e{0, 0, 0} {}
        //- vec3(double e0, double e1, double e2) implemented through
        //  positional arguments

        pub fn x(&self) -> f64 {
            self.one
        }
        pub fn y(&self) -> f64 {
            self.two
        }
        pub fn z(&self) -> f64 {
            self.three
        }

        //- vec3 operator-() const;
        //- double operator[](int i) const;
        //- double& operator[](int i);
        //- vec3& operator+=(const vec3 &v);
        //- vec3& operator*=(const double t);
        //- vec3& operator/=(const double t);

        //- The following two methods are implemented as an exercise and will
        //  be uncommented when needed
        // pub fn length(&self) -> f64 {
        //     self.length_squared().sqrt()
        // }

        // pub fn length_squared(&self) -> f64 {
        //     (self.one * self.one) + (self.two * self.two) + (self.three * self.three)
        // }
    }

    pub fn write_color(out: &mut dyn std::io::Write, pixel_color: Color) {
        // Write the translated [0,255] value of each color component
        write!(
            out,
            "{} {} {}\n",
            (255.999 * pixel_color.x()) as i32,
            (255.999 * pixel_color.y()) as i32,
            (255.999 * pixel_color.z()) as i32
        ).expect("failed to output color line");
    }
}
