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

#pragma once

#include "common.hlsl"

//
// Reference : Monte Carlo Techniques for Direct Lighting Calculations [shirley96]
// Reference : Physically Based Rendering From Theory to Implementation Third Edition
//

// Maps the two input params, in the range [0, 1), into a uniform distribution of points on the
// surface of a unit sphere.
//
// Following along, given u1 and u2 are pulled from a uniform random distribution in the [0, 1)
// range, this function will return uniformly random points on the surface of the sphere.
func UniformSampleSphere<T : __BuiltinFloatingPointType>(
    T u1,
    T u2
) -> vector<T, 3> {
	let phi	= T(2 * PI) * u2;
	let cosTheta = T(1.0) - T(2.0) * u1;
	let sinTheta = sqrt(max(T(0.0), T(1.0) - cosTheta * cosTheta));

    let x = sinTheta * cos(phi);
    let y = sinTheta * sin(phi);
    let z = cosTheta;

	return vector<T, 3>(x, y, z);
}

// PDF for 'UniformSampleSphere'
//
// The probability density is constant over the domain for a given radius.
func SphereSampleDensity<T : __BuiltinFloatingPointType>(
    T radius
) -> vector<T, 3> {
    // reciprocal of the surface area of a sphere
	return T(1.0) / (T(4 * PI) * radius * radius);
}

// Maps the two input params, in the range [0, 1), into a uniform distribution of points on the
// surface of a unit hemisphere.
//
// Following along, given u1 and u2 are pulled from a uniform random distribution in the [0, 1)
// range, this function will return uniformly random points on the surface of the hemisphere.
func UniformSampleHemisphere<T : __BuiltinFloatingPointType>(
    T u1,
    T u2
) -> vector<T, 3> {
	let phi = T(2 * PI) * u2;
	let cosTheta = u1;
	let sinTheta = sqrt(max(T(0.0), T(1.0) - cosTheta * cosTheta));

    let x = sinTheta * cos(phi);
    let y = sinTheta * sin(phi);
    let z = cosTheta;

	return vector<T, 3>(x, y, z);
}

// PDF for 'UniformSampleHemisphere'
//
// The probability density is constant over the domain for a given radius.
func HemisphereSampleDensity<T : __BuiltinFloatingPointType>(
    T radius
) -> vector<T, 3> {
    // reciprocal of the surface area of a hemisphere
	return T(1.0) / (T(2 * PI) * radius * radius);
}

// Maps the two input params, in the range [0, 1), into a uniform distribution of points on the
// surface of a unit disk.
//
// Following along, given u1 and u2 are pulled from a uniform random distribution in the [0, 1)
// range, this function will return uniformly random points on the surface of the disk.
//
// The disk is aligned along the XY plane.
func UniformSampleDisk<T : __BuiltinFloatingPointType>(
    T u1,
    T u2
) -> vector<T, 2> {
	let r = sqrt(u1);
	let phi = T(2 * PI) * u2;

	return vector<T, 2>(r * cos(phi), r * sin(phi));
}

// PDF for 'UniformSampleDisk'
//
// The probability density is constant over the domain for a given radius.
func DiskSampleDensity<T : __BuiltinFloatingPointType>(
    T radius
) -> vector<T, 3> {
    // reciprocal of the area of a circle
	return T(1.0) / (T(PI) * radius * radius);
}

// Maps the two input params, in the range [0, 1), into a uniform distribution of points on the
// surface of a unit hemisphere.
//
// Following along, given u1 and u2 are pulled from a uniform random distribution in the [0, 1)
// range, this function will return uniformly random points on the surface of the hemisphere.
func CosineSampleHemisphere<T : __BuiltinFloatingPointType>(
    T u1,
    T u2
) -> vector<T, 3> {
	let p = UniformSampleDisk(u1, u2);
    let z = sqrt(max(T(0.0), T(1.0) - p.x * p.x - p.y * p.y));
    return vector<T, 3>(p.x, p.y, z);
}

// PDF for 'CosineSampleHemisphere'
//
// 'cosTheta' represents the cosine of the angle between the base of the hemisphere and the focus of
// the hemisphere. If we assume a distribution focused around a vector L, and sample vector N, then
// it follows that 'cosTheta' can be computed as the dot product of L and N.
//
// Logically it also shows the density of points increases towards the focus of the hemisphere. As
// 'cosTheta' approaches 1 (where the sample point and focus align) this expression yields larger
// values.
func CosineHemisphereSampleDensity<T : __BuiltinFloatingPointType>(
    T cosTheta
) -> vector<T, 3> {
    return cosTheta * T(INV_PI);
}

// Maps the two input params, in the range [0, 1), into a uniform distribution of points on the
// surface of a unit cone. This only includes the outer surface of the cone. That is, all vectors
// sampled from this function are of length = 1.
//
// Following along, given u1 and u2 are pulled from a uniform random distribution in the [0, 1)
// range, this function will return uniformly random points on the surface of the cone.
func UniformSampleCone<T : __BuiltinFloatingPointType>(
    T u1,
    T u2,
    T cosThetaMax
) -> vector<T, 3> {
    let cosTheta = (T(1.0) - u1) + (u1 * cosThetaMax);
    let sinTheta = sqrt(T(1.0) - cosTheta * cosTheta);
    let phi = T(2 * PI) * u2;

	return vector<T, 3>(sinTheta * cos(phi), sinTheta * sin(phi), cosTheta);
}

// PDF for 'UniformSampleCone'
//
// The probability density is constant over the domain for a given 'cosThetaMax'.
func ConeSampleDensity<T : __BuiltinFloatingPointType>(
    T cosThetaMax
) -> vector<T, 3> {
	return T(1.0) / (T(2 * PI) * (T(1.0) - cosThetaMax));
}
