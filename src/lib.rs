pub mod raytracer {
    use std::ops;

    pub struct Vec3 {
        pub e1: f32,
        pub e2: f32,
        pub e3: f32
    }

    impl ops::Add<Vec3> for Vec3 {
        type Output = Vec3;

        fn add(self, _rhs: Vec3) -> Vec3 {
            let e1: f32 = self.e1 + _rhs.e2;
            let e2: f32 = self.e2 + _rhs.e2;
            let e3: f32 = self.e3 + _rhs.e3;

            Vec3 { e1: e1, e2: e2, e3: e3}
        }
}

    pub struct Ray {
        A: Vec3,
        B: Vec3
    }

    impl Ray {
        pub fn origin(self) -> Vec3 {
            self.A
        }

        pub fn direction(self) -> Vec3 {
            self.B
        }

        pub fn point_at_parameter(self, t :f32) -> Vec3 {
            self.A + self.B
        }
    }
}
