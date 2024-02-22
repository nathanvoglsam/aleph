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

//==================================================================================================
//
//  Baking Lab
//  by MJP and David Neubelt
//  http://mynameismjp.wordpress.com/
//
//  All code licensed under the MIT license
//
//==================================================================================================

// The code in this file was originally written by Stephen Hill (@self_shadow), who deserves all
// credit for coming up with this fit and implementing it. Buy him a beer next time you see him. :)

inline func ACESInputMat_F<T: __BuiltinFloatingPointType>() -> matrix<T, 3, 3> {
    let v : matrix<T, 3, 3> = {
        { T(0.59719), T(0.35458), T(0.04823) },
        { T(0.07600), T(0.90834), T(0.01566) },
        { T(0.02840), T(0.13383), T(0.83777) }
    };
    return v;
}

inline func ACESOutputMat_F<T: __BuiltinFloatingPointType>() -> matrix<T, 3, 3> {
    let v : matrix<T, 3, 3> = {
        { T( 1.60475), T(-0.53108), T(-0.07367) },
        { T(-0.10208), T( 1.10813), T(-0.00605) },
        { T(-0.00327), T(-0.07276), T( 1.07602) }
    };
    return v;
}

func RRTAndODTFit<T: __BuiltinFloatingPointType>(vector<T, 3> v) -> vector<T, 3> {
    let a = v * (v + T(0.0245786)) - T(0.000090537);
    let b = v * (T(0.983729) * v + T(0.4329510)) + T(0.238081);
    return a / b;
}

func ACESFitted<T: __BuiltinFloatingPointType>(vector<T, 3> color) -> vector<T, 3> {
    let input_mat = ACESInputMat_F<T>();
    let output_mat = ACESOutputMat_F<T>();
    color = mul(input_mat, color);
    color = RRTAndODTFit(color);
    color = mul(output_mat, color);
    color = saturate(color);
    return color;
}
