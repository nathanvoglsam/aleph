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

/*
 * Approximate SRGB to linear (linear from srgb) based on the gamma 2.2 curve
 *
 * Faster, and simpler, but less correct
 */
inline float3 ApproxLinearFromSRGB(float3 srgb) {
    return pow(srgb, 2.2);
}

/*
 * Approximate linear to SRGB (srgb from linear) based on the gamma 2.2 curve
 *
 * Faster, and simpler, but less correct
 */
inline float3 ApproxLinearToSRGB(float3 colour) {
    return pow(colour, 1 / 2.2);
}

/*
 * Approximate SRGB to linear (linear from srgb) based on the gamma 2.2 curve
 *
 * Faster, and simpler, but less correct
 *
 * The 4th alpha channel is unchanged (presumed linear in most cases)
 */
inline float4 ApproxLinearFromSRGBA(float4 srgba) {
    return float4(ApproxLinearFromSRGB(srgba.xyz), srgba.w);
}

/*
 * Approximate linear to SRGB (srgb from linear) based on the gamma 2.2 curve
 *
 * Faster, and simpler, but less correct
 *
 * The 4th alpha channel is unchanged (presumed linear in most cases)
 */
inline float4 ApproxLinearToSRGBA(float4 colour) {
    return float4(ApproxLinearToSRGB(colour.xyz), colour.w);
}

/*
 * Single component implementation for correct linear to SRGB
 */
inline float LinearComponentToSRGB(float val)
{
    if( val < 0.0031308 ) {
        val *= 12.92;
    } else {
        val = 1.055 * pow(val,1.0/2.4) - 0.055;
    }
    return val;
}

/*
 * Single component implementation for correct SRGB to linear
 */
inline float LinearComponentFromSRGB(float val)
{
    if( val < 0.04045f ) {
        val /= 12.92f;
    } else {
        val = pow((val + 0.055f)/1.055f,2.4f);
    }
    return val;
}

/*
 * SRGB to linear (linear from srgb)
 */
inline float3 LinearFromSRGB(float3 srgb) {
    const float x = LinearComponentFromSRGB(srgb.x);
    const float y = LinearComponentFromSRGB(srgb.y);
    const float z = LinearComponentFromSRGB(srgb.z);
    return float3(x,y,z);
}

/*
 * Linear to SRGB (srgb from linear)
 */
inline float3 LinearToSRGB(float3 colour) {
    const float x = LinearComponentToSRGB(colour.x);
    const float y = LinearComponentToSRGB(colour.y);
    const float z = LinearComponentToSRGB(colour.z);
    return float3(x,y,z);
}

/*
 * SRGB to linear (linear from srgb)
 *
 * The 4th alpha channel is unchanged (presumed linear in most cases)
 */
inline float4 LinearFromSRGBA(float4 srgba) {
    return float4(LinearFromSRGB(srgba.xyz), srgba.w);
}

/*
 * Linear to SRGB (srgb from linear)
 *
 * The 4th alpha channel is unchanged (presumed linear in most cases)
 */
inline float4 LinearToSRGBA(float4 colour) {
    return float4(LinearToSRGB(colour.xyz), colour.w);
}
