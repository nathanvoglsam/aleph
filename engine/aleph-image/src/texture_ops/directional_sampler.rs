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

use aleph_math::sampling::{octahedral_encode, sample_spherical_map};
use aleph_math::{Vec3, Vec4};

use crate::{AddressModeClamp, AddressModeWrap, CubeSampler, IPixelSample, PixelFormat};

/// A trait that abstracts over the concept of a texture that stores data for points on the surface
/// of a unit sphere, rather than points on a 2D grid.
///
/// This is logically similar to a cube map, but is abstracted away from the actual layout of the
/// backing image(s). Noteworthy implementations of this trait include:
/// - [`CubeSampler`]
/// - [`OctahderalDirectionalSampler`]
/// - [`EquirectangularDirectionalSampler`]
pub trait IDirectionalSampler {
    /// Performs a point-sample operation, returning the result for the given direction vector.
    ///
    /// - 'dir' must be a unit vector for valid output.
    fn point_sample(&self, dir: Vec3) -> Vec4;

    /// Performs a bilinear filtered sampple operation, returning the result for the given direction
    /// vector.
    ///
    /// - 'dir' must be a unit vector for valid output.
    fn sample(&self, dir: Vec3) -> Vec4;
}

impl<'a, T: PixelFormat> IDirectionalSampler for CubeSampler<'a, T> {
    #[inline]
    fn point_sample(&self, dir: Vec3) -> Vec4 {
        CubeSampler::<T>::point_sample::<AddressModeClamp, AddressModeClamp>(self, dir)
    }

    #[inline]
    fn sample(&self, _dir: Vec3) -> Vec4 {
        todo!()
    }
}

/// Wrapper over some sample-able image that implements [`IDirectionalSampler`] assuming that the
/// underlying image contains an environment map using an octahedral parametrization. That is, if
/// the image is an octahedral map.
pub struct OctahderalDirectionalSampler<'a, T>(pub &'a T);

impl<'a, T: IPixelSample> IDirectionalSampler for OctahderalDirectionalSampler<'a, T> {
    #[inline]
    fn point_sample(&self, dir: Vec3) -> Vec4 {
        let uv = octahedral_encode(dir);
        self.0.point_sample::<AddressModeWrap, AddressModeWrap>(uv)
    }

    #[inline]
    fn sample(&self, dir: Vec3) -> Vec4 {
        let uv = octahedral_encode(dir);
        self.0.sample::<AddressModeWrap, AddressModeWrap>(uv)
    }
}

/// Wrapper over some sample-able image that implements [`IDirectionalSampler`] assuming that the
/// underlying image contains an environment map using an equirectangular parametrization. That is,
/// if the image is an equirectangular map.
pub struct EquirectangularDirectionalSampler<'a, T>(pub &'a T);

impl<'a, T: IPixelSample> IDirectionalSampler for EquirectangularDirectionalSampler<'a, T> {
    #[inline]
    fn point_sample(&self, dir: Vec3) -> Vec4 {
        let uv = sample_spherical_map(dir);
        self.0.point_sample::<AddressModeWrap, AddressModeClamp>(uv)
    }

    #[inline]
    fn sample(&self, dir: Vec3) -> Vec4 {
        let uv = sample_spherical_map(dir);
        self.0.sample::<AddressModeWrap, AddressModeClamp>(uv)
    }
}
