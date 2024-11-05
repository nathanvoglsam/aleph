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

//!
//! Reference : Monte Carlo Techniques for Direct Lighting Calculations [shirley96]
//! Reference : Physically Based Rendering From Theory to Implementation Third Edition
//!

use std::f32::consts::{FRAC_1_PI, PI};

use ultraviolet::{Vec2, Vec3};

/// Maps the two input params, in the range [0, 1), into a uniform distribution of points on the
/// surface of a unit sphere.
///
/// Following along, given u1 and u2 are pulled from a uniform random distribution in the [0, 1)
/// range, this function will return uniformly random points on the surface of the sphere.
pub fn uniform_sample_sphere(u1: f32, u2: f32) -> Vec3 {
    let phi = 2.0 * PI * u2;
    let cos_theta = 1.0 - 2.0 * u1;
    let sin_theta = f32::sqrt(f32::max(0.0, 1.0 - cos_theta * cos_theta));

    let x = sin_theta * f32::cos(phi);
    let y = sin_theta * f32::sin(phi);
    let z = cos_theta;

    return Vec3::new(x, y, z);
}

/// PDF for 'UniformSampleSphere'
///
/// The probability density is constant over the domain for a given radius.
pub fn sphere_sample_density(radius: f32) -> f32 {
    // reciprocal of the surface area of a sphere
    return 1.0 / (4.0 * PI * radius * radius);
}

/// Maps the two input params, in the range [0, 1), into a uniform distribution of points on the
/// surface of a unit hemisphere.
///
/// Following along, given u1 and u2 are pulled from a uniform random distribution in the [0, 1)
/// range, this function will return uniformly random points on the surface of the hemisphere.
pub fn uniform_sample_hemisphere(u1: f32, u2: f32) -> Vec3 {
    let phi = 2.0 * PI * u2;
    let cos_theta = u1;
    let sin_theta = f32::sqrt(f32::max(0.0, 1.0 - cos_theta * cos_theta));

    let x = sin_theta * f32::cos(phi);
    let y = sin_theta * f32::sin(phi);
    let z = cos_theta;

    return Vec3::new(x, y, z);
}

/// PDF for 'UniformSampleHemisphere'
///
/// The probability density is constant over the domain for a given radius.
pub fn hemisphere_sample_density(radius: f32) -> f32 {
    // reciprocal of the surface area of a hemisphere
    return 1.0 / (2.0 * PI * radius * radius);
}

/// Maps the two input params, in the range [0, 1), into a uniform distribution of points on the
/// surface of a unit disk.
///
/// Following along, given u1 and u2 are pulled from a uniform random distribution in the [0, 1)
/// range, this function will return uniformly random points on the surface of the disk.
///
/// The disk is aligned along the XY plane.
pub fn uniform_sample_disk(u1: f32, u2: f32) -> Vec2 {
    let r = f32::sqrt(u1);
    let phi = 2.0 * PI * u2;

    return Vec2::new(r * f32::cos(phi), r * f32::sin(phi));
}

/// PDF for 'UniformSampleDisk'
///
/// The probability density is constant over the domain for a given radius.
pub fn disk_sample_density(radius: f32) -> f32 {
    // reciprocal of the area of a circle
    return 1.0 / (PI * radius * radius);
}

/// Maps the two input params, in the range [0, 1), into a uniform distribution of points on the
/// surface of a unit hemisphere.
///
/// Following along, given u1 and u2 are pulled from a uniform random distribution in the [0, 1)
/// range, this function will return uniformly random points on the surface of the hemisphere.
pub fn cosine_sample_hemisphere(u1: f32, u2: f32) -> Vec3 {
    let p = uniform_sample_disk(u1, u2);

    // Alternate form from PBRT. Reconstructs r^2 from x^2+y^2 using pythagoras. Or in this case
    // we're doing (1 - (x^2 + y^2)) = (1 - x^2 - y^2).
    // let z = f32::sqrt(f32::max(0.0, 1.0 - p.x * p.x - p.y * p.y));

    // Faster form based on the observation that r = sqrt(u1) -> r^2 = u1. (see UniformSampleDisk).
    let z = f32::sqrt(f32::max(0.0, 1.0 - u1));
    return Vec3::new(p.x, p.y, z);
}

/// PDF for 'CosineSampleHemisphere'
///
/// 'cosTheta' represents the cosine of the angle between the base of the hemisphere and the focus
/// of the hemisphere. If we assume a distribution focused around a vector L, and sample vector N,
/// then it follows that 'cosTheta' can be computed as the dot product of L and N.
///
/// Logically it also shows the density of points increases towards the focus of the hemisphere. As
/// 'cosTheta' approaches 1 (where the sample point and focus align) this expression yields larger
/// values.
pub fn cosine_hemisphere_sample_density(cos_theta: f32) -> f32 {
    return cos_theta * FRAC_1_PI;
}

/// Maps the two input params, in the range [0, 1), into a uniform distribution of points on the
/// surface of a unit cone. This only includes the outer surface of the cone. That is, all vectors
/// sampled from this function are of length = 1.
///
/// Following along, given u1 and u2 are pulled from a uniform random distribution in the [0, 1)
/// range, this function will return uniformly random points on the surface of the cone.
pub fn uniform_sample_cone(u1: f32, u2: f32, cos_theta_max: f32) -> Vec3 {
    let cos_theta = (1.0 - u1) + (u1 * cos_theta_max);
    let sin_theta = f32::sqrt(1.0 - cos_theta * cos_theta);
    let phi = 2.0 * PI * u2;

    return Vec3::new(
        sin_theta * f32::cos(phi),
        sin_theta * f32::sin(phi),
        cos_theta,
    );
}

/// PDF for 'UniformSampleCone'
///
/// The probability density is constant over the domain for a given 'cosThetaMax'.
pub fn cone_sample_density(cos_theta_max: f32) -> f32 {
    return 1.0 / (2.0 * PI * (1.0 - cos_theta_max));
}
