//
//
// This file is a part of Aleph
//
// https://github.com/nathanvoglsam/aleph
//
// MIT License
//
// Copyright (c) 2020 Aleph Engine
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

pub use to_double::ToDouble;
pub use to_single::ToSingle;
pub use ultraviolet::*;

pub mod unprojection {
    use ultraviolet::{Mat4, Vec3};

    ///
    /// Unprojects the given point, assuming that the point was projected with a reversed Z matrix
    /// with an infinite far plane.
    ///
    /// # Parameters
    ///
    /// let t = tan(vertical_fov / 2.0);
    /// let a = 1.0 / t;
    /// let b = a / aspect_ratio;
    ///
    /// These could be pulled directly from the projection matrix as m._m00 = a and m._m11 = b and
    /// m._32 = near. Again, assuming a reverse Z infinite projeciton matrix for a right-hand-y-up
    /// source coordinate system.
    ///
    #[inline]
    pub fn unproject_point(a: f32, b: f32, near: f32, point: &Vec3) -> Vec3 {
        let u_z = -near / point.z;
        let u_x = (-u_z * point.x) / a;
        let u_y = (-u_z * point.y) / b;
        ultraviolet::Vec3::new(u_x, u_y, u_z)
    }

    ///
    /// Unprojects the given point, assuming that the point was projected with the given pure
    /// projection matrix. This makes no assumptions about the input and derives 'a', 'b' and 'near'
    /// directly from the projection parameters. Obviously this requires having them available at
    /// the call site.
    ///
    /// # Info
    ///
    /// a, b, and near can be trivially extracted from a compatible matrix. If you have a pure
    /// projection matrix available use [unproject_point_with_matrix] instead as it avoids a 'tan'
    /// call and two fp divisions.
    ///
    #[inline]
    pub fn unproject_point_with_parameters(
        vertical_fov: f32,
        aspect_ratio: f32,
        near: f32,
        point: &Vec3,
    ) -> Vec3 {
        let t = (vertical_fov / 2.).tan();
        let a = 1. / t;
        let b = a / aspect_ratio;
        unproject_point(a, b, near, point)
    }

    ///
    /// Unprojects the given point, assuming that the point was projected with the given pure
    /// projection matrix. This follows the same assumption as [unproject_point]. That is: We assume
    /// the given matrix contains a reverse Z infinite projection matrix for a right-hand-y-up
    /// source coordinate system.
    ///
    /// # Warning
    ///
    /// 'proj' MUST be a pure projection matrix for it to get the correct parameters to unproject
    /// the point. If you don't have a pure matrix on hand you can use [unproject_point] and derive
    /// the inputs from the provided equations.
    ///
    #[inline]
    pub fn unproject_point_with_matrix(proj: &Mat4, point: &Vec3) -> Vec3 {
        let a = proj.cols[0][0];
        let b = proj.cols[1][1];
        let near = dbg!(proj.cols[3][2]);
        unproject_point(a, b, near, point)
    }
}

mod to_double {
    use ultraviolet::*;

    pub trait ToDouble {
        type Target;
        fn to_double(self) -> Self::Target;
    }

    impl ToDouble for Vec2 {
        type Target = DVec2;

        #[inline(always)]
        fn to_double(self) -> Self::Target {
            DVec2::new(self.x as f64, self.y as f64)
        }
    }

    impl ToDouble for Vec3 {
        type Target = DVec3;

        #[inline(always)]
        fn to_double(self) -> Self::Target {
            DVec3::new(self.x as f64, self.y as f64, self.z as f64)
        }
    }

    impl ToDouble for Vec4 {
        type Target = DVec4;

        #[inline(always)]
        fn to_double(self) -> Self::Target {
            DVec4::new(self.x as f64, self.y as f64, self.z as f64, self.w as f64)
        }
    }

    impl ToDouble for Bivec2 {
        type Target = DBivec2;

        #[inline(always)]
        fn to_double(self) -> Self::Target {
            DBivec2::new(self.xy as f64)
        }
    }

    impl ToDouble for Bivec3 {
        type Target = DBivec3;

        #[inline(always)]
        fn to_double(self) -> Self::Target {
            DBivec3::new(self.xy as f64, self.xz as f64, self.yz as f64)
        }
    }

    impl ToDouble for Rotor2 {
        type Target = DRotor2;

        #[inline(always)]
        fn to_double(self) -> Self::Target {
            DRotor2::new(self.s as f64, self.bv.to_double())
        }
    }

    impl ToDouble for Rotor3 {
        type Target = DRotor3;

        #[inline(always)]
        fn to_double(self) -> Self::Target {
            DRotor3::new(self.s as f64, self.bv.to_double())
        }
    }
}

mod to_single {
    use ultraviolet::*;

    pub trait ToSingle {
        type Target;
        fn to_single(self) -> Self::Target;
    }

    impl ToSingle for DVec2 {
        type Target = Vec2;

        #[inline(always)]
        fn to_single(self) -> Self::Target {
            Vec2::new(self.x as f32, self.y as f32)
        }
    }

    impl ToSingle for DVec3 {
        type Target = Vec3;

        #[inline(always)]
        fn to_single(self) -> Self::Target {
            Vec3::new(self.x as f32, self.y as f32, self.z as f32)
        }
    }

    impl ToSingle for DVec4 {
        type Target = Vec4;

        #[inline(always)]
        fn to_single(self) -> Self::Target {
            Vec4::new(self.x as f32, self.y as f32, self.z as f32, self.w as f32)
        }
    }

    impl ToSingle for DBivec2 {
        type Target = Bivec2;

        #[inline(always)]
        fn to_single(self) -> Self::Target {
            Bivec2::new(self.xy as f32)
        }
    }

    impl ToSingle for DBivec3 {
        type Target = Bivec3;

        #[inline(always)]
        fn to_single(self) -> Self::Target {
            Bivec3::new(self.xy as f32, self.xz as f32, self.yz as f32)
        }
    }

    impl ToSingle for DRotor2 {
        type Target = Rotor2;

        #[inline(always)]
        fn to_single(self) -> Self::Target {
            Rotor2::new(self.s as f32, self.bv.to_single())
        }
    }

    impl ToSingle for DRotor3 {
        type Target = Rotor3;

        #[inline(always)]
        fn to_single(self) -> Self::Target {
            Rotor3::new(self.s as f32, self.bv.to_single())
        }
    }
}
