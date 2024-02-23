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

// 
// Struct that describes a ray for use with ray tracing functions.
// 
// Members:
// 
// - origin: The origin of the ray in some coordinate space
// - direction: A unit vector that represents the direction to trace in
// 
struct Ray {
    float3 origin;
    float3 direction;
};

//
// Returns the value of the largest component in the vector passed into the function
//
func MaxComponent<T: __BuiltinFloatingPointType>(vector<T, 2> v) -> T {
    return max(v.x, v.y);
}

//
// Returns the value of the largest component in the vector passed into the function
//
func MaxComponent<T: __BuiltinFloatingPointType>(vector<T, 3> v) -> T {
    let max_xy = max(v.x, v.y);
    return max(max_xy, v.z);
}

//
// Returns the value of the largest component in the vector passed into the function
//
func MaxComponent<T: __BuiltinFloatingPointType>(vector<T, 4> v) -> T {
    let max_xy = max(v.x, v.y);
    let max_xyz = max(max_xy, v.z);
    return max(max_xyz, v.w);
}

//
// Saturate a FP16 (half precision) float to be in the 0-1 range but never actually 0
//
inline func SaturateFP16<T: __BuiltinFloatingPointType>(T val) -> T{
    return clamp(val, T(0.089), T(1.0));
}

//
// Saturate a FP32 (full precision) float to be in the 0-1 range but never actually 0
//
inline func SaturateFP32<T : __BuiltinFloatingPointType>(T val) -> T {
    return clamp(val, T(0.045), T(1.0));
}

inline func Saturate<T : __BuiltinFloatingPointType>(T val) -> T {
    if (val is float) {
        return clamp(val, T(0.045), T(1.0));
    } else if (val is half) {
        return clamp(val, T(0.089), T(1.0));
    } else {
        return clamp(val, T(0.045), T(1.0)); // TODO: other types
    }
}
