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

static const float PI = 3.14159265f;

/*
 * Struct that describes a ray for use with ray tracing functions.
 *
 * Members:
 *
 * - origin: The origin of the ray in some coordinate space
 * - direction: A unit vector that represents the direction to trace in
 */
struct Ray {
    float3 origin;
    float3 direction;
};

/*
 * Returns the value of the largest component in the vector passed into the function
 */
float MaxComponent(float2 v) {
    return max(v.x, v.y);
}

/*
 * Returns the value of the largest component in the vector passed into the function
 */
float MaxComponent(float3 v) {
    const float max_xy = max(v.x, v.y);
    return max(max_xy, v.z);
}

/*
 * Returns the value of the largest component in the vector passed into the function
 */
float MaxComponent(float4 v) {
    const float max_xy = max(v.x, v.y);
    const float max_xyz = max(max_xy, v.z);
    return max(max_xyz, v.w);
}

/*
 * Saturate a FP16 (half precision) float to be in the 0-1 range but never actually 0
 */
inline float SaturateFP16(float val) {
    return clamp(val, 0.089, 1.0);
}

/*
 * Saturate a FP32 (full precision) float to be in the 0-1 range but never actually 0
 */
inline float SaturateFP32(float val) {
    return clamp(val, 0.045, 1.0);
}

inline float3 ApproxLinearFromSRGB(float3 srgb) {
    return pow(srgb, 2.2);
}

inline float3 ApproxLinearToSRGB(float3 colour) {
    return pow(colour, 1 / 2.2);
}

inline float4 ApproxLinearFromSRGBA(float4 srgba) {
    return float4(ApproxLinearFromSRGB(srgba.xyz), srgba.w);
}

inline float4 ApproxLinearToSRGBA(float4 colour) {
    return float4(ApproxLinearToSRGB(colour.xyz), colour.w);
}
