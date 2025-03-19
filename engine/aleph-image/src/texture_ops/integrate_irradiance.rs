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

use aleph_math::sampling::{
    center_sample_around_normal, cosine_sample_hemisphere, equirectangular_uv_to_direction,
    octahedral_decode,
};
use aleph_math::{UVec2, Vec2, Vec3, Vec4};
use rand::Rng;

use crate::{
    FaceNegX, FaceNegY, FaceNegZ, FacePosX, FacePosY, FacePosZ, IDirectionalSampler, IFaceSelector,
    IPixelAccess, IPixelStorage, ImageBuffer, PixelFormat,
};

/// Performs a Monte Carlo integration of the input environment map.
///
/// This performs an importance sample of the irradiance reaching some point with normal 'n' from
/// the provided environment map.
///
/// # Warning
///
/// This leaves out a necessary multiplication by pi to produce the true irradiance value. This is
/// intentional as that multiplication is often factored into different terms where the irradiance
/// map is sampled.
///
/// You _can_ correct this by performing the multiplication yourself by taking
/// `integrate_irradiance_for_direction(...) * PI`.
pub fn integrate_irradiance_for_direction(
    src: &impl IDirectionalSampler,
    random: &mut impl Rng,
    n: Vec3,
    samples: usize,
) -> Vec4 {
    // This function will importance sample the hemisphere centered on 'n' using a cosine
    // distribution.
    //
    // The full integrand we are calculating is:
    // ∫ L(l) <n.l> dl
    //
    // Which is integrated over the hemisphere centered on 'n'. This is the irradiance that reaches
    // a point with normal 'n' from the input environment map.
    //
    // Our integrator draws points from a cosine distribution. This has a pdf of pi/<n.l>. You may
    // note that when we produce our monte carlo estimator for our integrand, the <n.l> terms cancel
    // and the pi is constant. This follows from the monte carlo estimator given as.
    //
    // ∫ f(x) dx -> (1/N) Σ f(x)/p(x)
    //
    // The <n.l> divides out, and the PI is constant so it can be hoisted out of the integrand and
    // multiplied after the convolution. It becomes
    //
    // (pi/N) Σ L(l)
    //
    // We choose to not perform the post-multiply by PI, so we're not calculating the full integral.
    // That term can be factored in at a later
    let mut acc = Vec4::zero();

    for _ in 0..samples {
        let u1 = random.random::<f32>();
        let u2 = random.random::<f32>();
        let k = cosine_sample_hemisphere(u1, u2);

        let l = center_sample_around_normal(k, n);

        let nol = n.dot(l);

        // The PDF is not used as the full term that we're integrating cancels out with the PDF
        if nol > 0.0 {
            acc += src.sample(l);
        }
    }

    acc * (1.0 / samples as f32)
}

pub fn integrate_irradiance_to_cube<F: IFaceSelector, O: PixelFormat>(
    src: &impl IDirectionalSampler,
    face_dimension: UVec2,
    samples: usize,
) -> ImageBuffer<O> {
    use rand::SeedableRng;

    let mut dst = ImageBuffer::<O>::new(face_dimension.x, face_dimension.y);
    let dim_f32 = dst.dimensions_f32();

    let mut rng = rand_xoshiro::Xoshiro256PlusPlus::from_os_rng();

    for y in 0..dst.height() {
        let v = (y as f32 + 0.5) / dim_f32.y;
        for x in 0..dst.width() {
            let u = (x as f32 + 0.5) / dim_f32.x;

            // Use our face selector interface to map the uv space onto the requested cube direction
            // that we want to sample.
            let dir = F::get_mapped(u, v);
            let p = integrate_irradiance_for_direction(src, &mut rng, dir, samples);
            let p = O::from_vec4(p);
            dst.store(x, y, p);
        }
    }

    dst
}

pub(crate) fn integrate_irradiance_to_whole_cube<O: PixelFormat>(
    dst: &mut Vec<ImageBuffer<O>>,
    src: &impl IDirectionalSampler,
    face_dimensions: UVec2,
    samples: usize,
) {
    dst.push(integrate_irradiance_to_cube::<FacePosX, _>(
        src,
        face_dimensions,
        samples,
    ));
    dst.push(integrate_irradiance_to_cube::<FaceNegX, _>(
        src,
        face_dimensions,
        samples,
    ));
    dst.push(integrate_irradiance_to_cube::<FacePosY, _>(
        src,
        face_dimensions,
        samples,
    ));
    dst.push(integrate_irradiance_to_cube::<FaceNegY, _>(
        src,
        face_dimensions,
        samples,
    ));
    dst.push(integrate_irradiance_to_cube::<FacePosZ, _>(
        src,
        face_dimensions,
        samples,
    ));
    dst.push(integrate_irradiance_to_cube::<FaceNegZ, _>(
        src,
        face_dimensions,
        samples,
    ));
}

pub fn integrate_irradiance_to_equi<O: PixelFormat>(
    src: &impl IDirectionalSampler,
    face_dimension: UVec2,
    samples: usize,
) -> ImageBuffer<O> {
    use rand::SeedableRng;

    let mut dst = ImageBuffer::<O>::new(face_dimension.x, face_dimension.y);
    let dim_f32 = dst.dimensions_f32();

    let mut rng = rand_xoshiro::Xoshiro256PlusPlus::from_os_rng();

    for y in 0..dst.height() {
        let v = (y as f32 + 0.5) / dim_f32.y;
        for x in 0..dst.width() {
            let u = (x as f32 + 0.5) / dim_f32.x;

            // Use our face selector interface to map the uv space onto the requested cube direction
            // that we want to sample.
            let dir = equirectangular_uv_to_direction(Vec2::new(u, v));
            let p = integrate_irradiance_for_direction(src, &mut rng, dir, samples);
            let p = O::from_vec4(p);
            dst.store(x, y, p);
        }
    }

    dst
}

pub fn integrate_irradiance_to_octahedral<O: PixelFormat>(
    src: &impl IDirectionalSampler,
    face_dimension: UVec2,
    samples: usize,
) -> ImageBuffer<O> {
    use rand::SeedableRng;

    let mut dst = ImageBuffer::<O>::new(face_dimension.x, face_dimension.y);
    let dim_f32 = dst.dimensions_f32();

    let mut rng = rand_xoshiro::Xoshiro256PlusPlus::from_os_rng();

    for y in 0..dst.height() {
        let v = (y as f32 + 0.5) / dim_f32.y;
        for x in 0..dst.width() {
            let u = (x as f32 + 0.5) / dim_f32.x;

            // Use our face selector interface to map the uv space onto the requested cube direction
            // that we want to sample.
            let dir = octahedral_decode(Vec2::new(u, v));
            let p = integrate_irradiance_for_direction(src, &mut rng, dir, samples);
            let p = O::from_vec4(p);
            dst.store(x, y, p);
        }
    }

    dst
}
