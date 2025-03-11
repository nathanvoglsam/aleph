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

use aleph_math::{Vec2, Vec4};

use crate::{IPixelAccess, IPixelStorage, ImageBuffer, PixelFormat};

/// An extended interface built atop [`IPixelAccess`] that enables bilinear sampling of the
/// underlying image using normalized image coordinates.
///
/// This is an extension over, and distinct from, direct pixel access. Sampling can be performed
/// without knowledge of the underlying image's dimensions _or_ format. The input coordinate is in
/// a normalized uv space using floating point coordinates, and all accesses yield a 4-component
/// floating point vector with the resulting value mapped from the in-memory format.
///
/// # Implementation
///
/// We follow the implemented as specified in the Direct3D11 texture sampling specification, using
/// their specified behavior for our addressing modes and bilinear filtering, with some things
/// omitted. Specifically we _do not_ implement any of the fixed point conversions. All operations
/// are done in floating point.
///
/// # Shaders?
///
/// This interface is a CPU implementation of what contemporary shading languages provide for image
/// sampling on the GPU. This interface provides similar capabilities.
pub trait IPixelSample: IPixelAccess {
    /// Performs a single sample of the image at the given uv coordinate.
    ///
    /// The input coordinates are in a normalized space where 0..1 ranges across the width/height of
    /// the underlying image. Images can be sampled without knowledge of the underlying size of the
    /// image.
    ///
    /// The resulting value is always a Vec4, mapped from the underlying pixel representation. The
    /// mapping rules are simple:
    /// - The Vec4's xyzw fields map to rgba in the source image.
    /// - Channels missing in the source image are mapped to zero. Loading an RG format image will
    ///   yield a Vec4 containing rg00.
    /// - Unorm images are mapped into the 0..1 range.
    /// - Float images have each channel loaded directly, widening to fp32 if needed.
    ///
    /// The addressing mode for the U and V coordinate is configurable for each axis separately. The
    /// [`IAddressMode`] interface provides the functionality needed. The 'U' and 'V' generic
    /// parameters configure the address mode for the U and V axis respectively.
    ///
    /// The sampling operation is unfiltered and will follow the point sampling rules specified in
    /// the d3d11 spec. Only a single tap of the texture will be taken, with the point snapped to
    /// a pixel by taking the floor of the uv as mapped into the size range of the texture.
    ///
    /// # Precision
    ///
    /// The sampling behavior is sensitive to floating point precision in the input coordinates.
    /// Using coordinates with a large magnitude will reduce the precision of the sampling. Sampling
    /// is also sensitive to precision via the underlying dimensions of the image. It is recommended
    /// to avoid passing in large UV coordinates, especially for large textures.
    fn point_sample<U: IAddressMode, V: IAddressMode>(&self, uv: Vec2) -> Vec4;

    /// Performs a bilinear sample of the image at the given uv coordinate.
    ///
    /// The input coordinates are in a normalized space where 0..1 ranges across the width/height of
    /// the underlying image. Images can be sampled without knowledge of the underlying size of the
    /// image.
    ///
    /// The resulting value is always a Vec4, mapped from the underlying pixel representation. The
    /// mapping rules are simple:
    /// - The Vec4's xyzw fields map to rgba in the source image.
    /// - Channels missing in the source image are mapped to zero. Loading an RG format image will
    ///   yield a Vec4 containing rg00.
    /// - Unorm images are mapped into the 0..1 range.
    /// - Float images have each channel loaded directly, widening to fp32 if needed.
    ///
    /// The addressing mode for the U and V coordinate is configurable for each axis separately. The
    /// [`IAddressMode`] interface provides the functionality needed. The 'U' and 'V' generic
    /// parameters configure the address mode for the U and V axis respectively.
    ///
    /// Samples are filterd using a bilinear kernel. Each pixel in the source image defines an
    /// explicit sample at a texel center in our normalized coordinate space. This means that to
    /// sample exactly the value at [0,0] as fetched from [`IPixelAccess::load`] you would sample
    /// the texture at [0.5, 0.5]. The famous half-pixel offset. The sample filter takes 4 taps of
    /// the stored image and performs linear interpolation across each axis to interpolate a
    /// continuous function from the discrete samples stored in the image.
    ///
    /// # Precision
    ///
    /// The sampling behavior is sensitive to floating point precision in the input coordinates.
    /// Using coordinates with a large magnitude will reduce the precision of the sampling. Sampling
    /// is also sensitive to precision via the underlying dimensions of the image. It is recommended
    /// to avoid passing in large UV coordinates, especially for large textures.
    fn sample<U: IAddressMode, V: IAddressMode>(&self, uv: Vec2) -> Vec4;
}

/// Generic interface for address mode implementations. Used by [`IPixelSample`] to allow
/// configuring the addressing mode for UV coordinates using a monomorphized interface.
pub trait IAddressMode {
    fn reduce_range(u: f32) -> f32;
    fn apply(dim: f32, scaled_u: f32) -> f32;
}

/// Implements [`IAddressMode`] following the 'D3D11_TEXTURE_ADDRESS_CLAMP' addressming mode.
pub struct AddressModeClamp;

impl IAddressMode for AddressModeClamp {
    #[inline(always)]
    fn reduce_range(u: f32) -> f32 {
        if u <= -10.0 {
            -10.0
        } else if u >= 10.0 {
            10.0
        } else {
            u
        }
    }

    #[inline(always)]
    fn apply(dim: f32, scaled_u: f32) -> f32 {
        f32::max(0.0, f32::min(scaled_u, dim - 1.0))
    }
}

/// Implements [`IAddressMode`] following the 'D3D11_TEXTURE_ADDRESS_WRAP' addressming mode.
pub struct AddressModeWrap;

impl IAddressMode for AddressModeWrap {
    #[inline(always)]
    fn reduce_range(x: f32) -> f32 {
        x.fract()
    }

    #[inline(always)]
    fn apply(dim: f32, scaled_u: f32) -> f32 {
        let x = scaled_u % dim;
        if x < 0.0 {
            x + dim
        } else {
            x
        }
    }
}

impl<T: PixelFormat> IPixelSample for ImageBuffer<T> {
    fn point_sample<U: IAddressMode, V: IAddressMode>(&self, uv: Vec2) -> Vec4 {
        let dims_f32 = self.dimensions_f32();

        let u = point_texel_coord_to_sample_pos::<U>(dims_f32.x, uv.x);
        let v = point_texel_coord_to_sample_pos::<V>(dims_f32.y, uv.y);

        self.load(u as u32, v as u32).as_vec4()
    }

    fn sample<U: IAddressMode, V: IAddressMode>(&self, uv: Vec2) -> Vec4 {
        let dims_f32 = self.dimensions_f32();

        let (t_floor_u, t_ceil_u, w_floor_u, w_ceil_u) =
            texel_coord_to_sample_pos_and_weights::<U>(dims_f32.x, uv.x);
        let (t_floor_v, t_ceil_v, w_floor_v, w_ceil_v) =
            texel_coord_to_sample_pos_and_weights::<V>(dims_f32.y, uv.y);

        let sample_0 = self.load(t_floor_u as u32, t_floor_v as u32).as_vec4();
        let sample_0 = sample_0 * w_floor_u * w_floor_v;

        let sample_1 = self.load(t_floor_u as u32, t_ceil_v as u32).as_vec4();
        let sample_1 = sample_1 * w_floor_u * w_ceil_v;

        let sample_2 = self.load(t_ceil_u as u32, t_floor_v as u32).as_vec4();
        let sample_2 = sample_2 * w_ceil_u * w_floor_v;

        let sample_3 = self.load(t_ceil_u as u32, t_ceil_v as u32).as_vec4();
        let sample_3 = sample_3 * w_ceil_u * w_ceil_v;

        sample_0 + sample_1 + sample_2 + sample_3
    }
}

fn texel_coord_to_sample_pos_and_weights<M: IAddressMode>(
    dim: f32,
    x: f32,
) -> (f32, f32, f32, f32) {
    let x = M::reduce_range(x);
    let scaled = x * dim - 0.5;

    let floor = scaled.floor();
    let floor = M::apply(dim, floor);

    let ceil = floor + 1.0;
    let ceil = M::apply(dim, ceil);

    let ceil_w = scaled.fract();
    let floor_w = 1.0 - ceil_w;

    (floor, ceil, floor_w, ceil_w)
}

fn point_texel_coord_to_sample_pos<M: IAddressMode>(dim: f32, x: f32) -> f32 {
    let x = M::reduce_range(x);
    let scaled = x * dim;

    let floor = scaled.floor();
    let floor = M::apply(dim, floor);

    floor
}
