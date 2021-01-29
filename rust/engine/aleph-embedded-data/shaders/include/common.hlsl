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

// 0-1 linear  from  0-255 sRGB
inline float3 LinearFromSRGB(float3 srgb) {
    const float3 vec_10_31475 = float3(10.31475, 10.31475, 10.31475);
    const float3 vec_3294_6 = float3(3294.6, 3294.6, 3294.6);
    const float3 vec_14_025 = float3(14.025, 14.025, 14.025);
    const float3 vec_269_025 = float3(269.025, 269.025, 269.025);
    const float3 vec_2_4 = float3(2.4, 2.4, 2.4);

    const bool3 cutoff = srgb < vec_10_31475;
    const float3 lower = srgb / vec_3294_6;
    const float3 higher = pow((srgb + vec_14_025) / vec_269_025, vec_2_4);
    return lerp(higher, lower, cutoff);
}
inline float4 LinearFromSRGBA(float4 srgba) {
    const float alpha = srgba.a / 255.0;
    const float3 colour = LinearFromSRGB(srgba.rgb);
    return float4(colour, alpha);
}
