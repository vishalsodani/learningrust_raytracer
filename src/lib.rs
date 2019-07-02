pub mod raytracer {
    use std::ops;

    #[derive(Copy, Clone)]
    pub struct Vec3 {
        pub e1: f32,
        pub e2: f32,
        pub e3: f32
    }

    impl Vec3 {
        fn length(self) -> f32 {
            (self.e1*self.e1 + self.e2*self.e2 + self.e3*self.e3).sqrt()
        }

        fn y(self) -> f32 {
            self.e2
        }
    }

    impl ops::Add<Vec3> for Vec3 {
        type Output = Vec3;

        fn add(self, _rhs: Vec3) -> Vec3 {
            let e1: f32 = self.e1 + _rhs.e2;
            let e2: f32 = self.e2 + _rhs.e2;
            let e3: f32 = self.e3 + _rhs.e3;

            Vec3 { e1, e2, e3}
        }
    }

    impl ops::Div<f32> for Vec3 {
        type Output = Vec3;

        fn div(self, t: f32) -> Vec3 {
            Vec3 { e1: self.e1/t, e2: self.e2/t, e3: self.e3/t}
        }
    }
    
    impl ops::Mul<f32> for Vec3 {
        type Output = Vec3;

        fn mul(self, t: f32) -> Vec3 {
            Vec3 {e1: t* self.e1, e2: t*self.e2, e3: t*self.e3}
        }
    }

    pub struct Ray {
        pub A: Vec3,
        pub B: Vec3
    }

    impl Ray {
        pub fn origin(self) -> Vec3 {
            self.A
        }

        pub fn direction(self) -> Vec3 {
            self.B
        }

        pub fn point_at_parameter(self, t :f32) -> Vec3 {
            self.A + (self.B*t)
        }

        pub fn color(self) -> Vec3 {
            let unit_direction: Vec3 = self.direction();
            let length = unit_direction.clone().length();

            let unit_direction :Vec3 = unit_direction / length;
            let t: f32 = (unit_direction.y()  + 1.0)*0.5;
            Vec3{e1:1.0, e2:1.0, e3: 1.0}*(1.0-t) + Vec3{e1:0.5, e2:0.7, e3:1.0}*t
        }
    }
}
