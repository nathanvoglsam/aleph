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

//
// Approximate SRGB to linear (linear from srgb) based on the gamma 2.2 curve
//
// Faster, and simpler, but less correct
//
inline func ApproxLinearFromSRGB<T : __BuiltinFloatingPointType>(
    vector<T, 3> srgb
) -> vector<T, 3> {
    return pow(srgb, T(2.2));
}

//
// Approximate linear to SRGB (srgb from linear) based on the gamma 2.2 curve
//
// Faster, and simpler, but less correct
//
inline func ApproxLinearToSRGB<T : __BuiltinFloatingPointType>(
    vector<T, 3> colour
) -> vector<T, 3> {
    return pow(colour, T(1 / 2.2));
}

//
// Approximate SRGB to linear (linear from srgb) based on the gamma 2.2 curve
//
// Faster, and simpler, but less correct
//
// The 4th alpha channel is unchanged (presumed linear in most cases)
//
inline func ApproxLinearFromSRGBA<T : __BuiltinFloatingPointType>(
    vector<T, 4> srgba
) -> vector<T, 4> {
    return vector<T, 4>(ApproxLinearFromSRGB(srgba.xyz), srgba.w);
}

//
// Approximate linear to SRGB (srgb from linear) based on the gamma 2.2 curve
//
// Faster, and simpler, but less correct
//
// The 4th alpha channel is unchanged (presumed linear in most cases)
//
inline func ApproxLinearToSRGBA<T : __BuiltinFloatingPointType>(
    vector<T, 4> colour
) -> vector<T, 4> {
    return vector<T, 4>(ApproxLinearToSRGB(colour.xyz), colour.w);
}

//
// Single component implementation for correct linear to SRGB
//
inline func LinearComponentToSRGB<T: __BuiltinFloatingPointType>(T val) -> T {
    if( val < T(0.0031308) ) {
        val *= T(12.92);
    } else {
        val = T(1.055) * pow(val,T(1.0/2.4)) - T(0.055);
    }
    return val;
}

//
// Single component implementation for correct SRGB to linear
//
inline func LinearComponentFromSRGB<T: __BuiltinFloatingPointType>(T val) -> T {
    if( val < T(0.04045) ) {
        val /= T(12.92);
    } else {
        val = pow((val + T(0.055)) / T(1.055), T(2.4));
    }
    return val;
}

//
// SRGB to linear (linear from srgb)
//
inline func LinearFromSRGB<T : __BuiltinFloatingPointType>(vector<T, 3> srgb) -> vector<T, 3> {
    let x = LinearComponentFromSRGB(srgb.x);
    let y = LinearComponentFromSRGB(srgb.y);
    let z = LinearComponentFromSRGB(srgb.z);
    return vector<T, 3>(x,y,z);
}

//
// Linear to SRGB (srgb from linear)
//
inline func LinearToSRGB<T : __BuiltinFloatingPointType>(vector<T, 3> colour) -> vector<T, 3> {
    let x = LinearComponentToSRGB(colour.x);
    let y = LinearComponentToSRGB(colour.y);
    let z = LinearComponentToSRGB(colour.z);
    return vector<T, 3>(x,y,z);
}

//
// SRGB to linear (linear from srgb)
//
// The 4th alpha channel is unchanged (presumed linear in most cases)
//
inline func LinearFromSRGBA<T : __BuiltinFloatingPointType>(vector<T, 4> srgba) -> vector<T, 4> {
    return vector<T, 4>(LinearFromSRGB(srgba.xyz), srgba.w);
}

//
// Linear to SRGB (srgb from linear)
//
// The 4th alpha channel is unchanged (presumed linear in most cases)
//
inline func LinearToSRGBA<T : __BuiltinFloatingPointType>(vector<T, 4> colour) -> vector<T, 4> {
    return vector<T, 4>(LinearToSRGB(colour.xyz), colour.w);
}
