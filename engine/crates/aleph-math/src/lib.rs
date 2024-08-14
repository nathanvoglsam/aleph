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
