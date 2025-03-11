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

use aleph_math::{Vec2, Vec3, Vec4};

use crate::{IAddressMode, IPixelAccess, IPixelSample, ImageBuffer, PixelFormat};

pub struct CubeSampler<'a, T: PixelFormat> {
    images: [&'a ImageBuffer<T>; 6],
}

impl<'a, T: PixelFormat> CubeSampler<'a, T> {
    pub fn new(
        px: &'a ImageBuffer<T>,
        nx: &'a ImageBuffer<T>,
        py: &'a ImageBuffer<T>,
        ny: &'a ImageBuffer<T>,
        pz: &'a ImageBuffer<T>,
        nz: &'a ImageBuffer<T>,
    ) -> Self {
        Self {
            images: [px, nx, py, ny, pz, nz],
        }
    }

    pub fn new_from_slice(images: &'a [ImageBuffer<T>]) -> Self {
        Self {
            images: [
                &images[0], &images[1], &images[2], &images[3], &images[4], &images[5],
            ],
        }
    }

    pub fn point_sample<U: IAddressMode, V: IAddressMode>(&self, dir: Vec3) -> Vec4 {
        let (face, uv) = cube_sample_to_face_and_uv(dir);
        self.images[face as usize].point_sample::<U, V>(uv)
    }
}

fn cube_sample_to_face_and_uv(d: Vec3) -> (u8, Vec2) {
    // This function implementation follows the rules as set out by the d3d11_3 spec.

    // Precalculate abs value for all axis, needed for determining the major axis
    let d_abs = d.abs();

    // Initialize the fields we will be building while selecting the major axis
    //
    // This will assume that x is the major axis and overwrite the values if we detect another
    // axis is the major axis.
    //
    // Of note is that we always select the _positive_ face index when we detect a new major axis
    // regardless of the sign of that axis. We perform the fixup to select the real face after the
    // final major axis has been found.
    //
    // Our uv selection follows the same rules as IFaceSelector (it's the inverse!) with the
    // additional constraint of negating one of the axis based on the sign of the major axis. This
    // is distinct from face selection, the true final uv value is deduced here without a fixup
    // step. Face selecting is easier with a conditional at the end so we do it that way.
    let mut axis_major = d.x; // Value of axis major
    let mut axis_major_abs = d_abs.x; // abs(axis_major)
    let mut u = -d.z * axis_major.signum(); // u coord mapped from direction vector
    let mut v = -d.y; // v coord mapped from direction vector
    let mut face = 0; // selected face we should sample from

    // Check if the y axis magnitude is larger. Comparing >= instead of > allows us to handle the
    // precedence rules d3d11 asks for. If X and Y tie then Y will be selected here. This will
    // overwrite all the fields we just set.
    if d_abs.y >= axis_major_abs {
        axis_major = d.y;
        axis_major_abs = d_abs.y;
        u = d.x;
        v = d.z * axis_major.signum();
        face = 2;
    }

    // Do the same as above for the Z axis. The same comparison property applies. By selecting the
    // axis in the X,Y,Z order and using >= the precedence of Z,Y,X for tie breaking naturally falls
    // out.
    if d_abs.z >= axis_major_abs {
        axis_major = d.z;
        axis_major_abs = d_abs.z;
        u = d.x * axis_major.signum();
        v = -d.y;
        face = 4;
    }

    // If axis_major is < 0 then we should select the appropiate negative face for that axis.
    // Thankfully we can exploit the property that we always select the positive face first. If we
    // detect a negative face we just add one to the face index as the negative face for a given
    // axis is always the face after the positive for that axis.
    if axis_major.is_sign_negative() {
        face += 1;
    }

    // Normalize uv coordinate and map it from [-1,1] into [0,1] range. This is now a valid UV
    // coordinate that can be used to sample a cube map.
    u /= axis_major_abs;
    v /= axis_major_abs;
    u = (u * 0.5) + 0.5;
    v = (v * 0.5) + 0.5;

    (face, Vec2::new(u, v))
}

#[cfg(test)]
mod tests {
    use aleph_math::Vec3;

    use crate::texture_ops::cube_sample::cube_sample_to_face_and_uv;

    #[test]
    fn test_cube_sample() {
        let center_pos_x = cube_sample_to_face_and_uv(Vec3::unit_x());
        assert_eq!(center_pos_x.0, 0);
        assert_eq!(center_pos_x.1.x, 0.5);
        assert_eq!(center_pos_x.1.y, 0.5);

        let center_neg_x = cube_sample_to_face_and_uv(-Vec3::unit_x());
        assert_eq!(center_neg_x.0, 1);
        assert_eq!(center_neg_x.1.x, 0.5);
        assert_eq!(center_neg_x.1.y, 0.5);

        let center_pos_y = cube_sample_to_face_and_uv(Vec3::unit_y());
        assert_eq!(center_pos_y.0, 2);
        assert_eq!(center_pos_y.1.x, 0.5);
        assert_eq!(center_pos_y.1.y, 0.5);

        let center_neg_y = cube_sample_to_face_and_uv(-Vec3::unit_y());
        assert_eq!(center_neg_y.0, 3);
        assert_eq!(center_neg_y.1.x, 0.5);
        assert_eq!(center_neg_y.1.y, 0.5);

        let center_pos_z = cube_sample_to_face_and_uv(Vec3::unit_z());
        assert_eq!(center_pos_z.0, 4);
        assert_eq!(center_pos_z.1.x, 0.5);
        assert_eq!(center_pos_z.1.y, 0.5);

        let center_neg_z = cube_sample_to_face_and_uv(-Vec3::unit_z());
        assert_eq!(center_neg_z.0, 5);
        assert_eq!(center_neg_z.1.x, 0.5);
        assert_eq!(center_neg_z.1.y, 0.5);

        let result = cube_sample_to_face_and_uv(Vec3::new(1.0, 1.0, 1.0));
        assert_eq!(result.0, 4);
        assert_eq!(result.1.x, 1.0);
        assert_eq!(result.1.y, 0.0);

        let result = cube_sample_to_face_and_uv(Vec3::new(2.0, 2.0, 2.0));
        assert_eq!(result.0, 4);
        assert_eq!(result.1.x, 1.0);
        assert_eq!(result.1.y, 0.0);
    }
}
