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

// The MIT License (MIT)
//
// Copyright (c) <2014> <Michal Drobot>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

//! Dervied from: https://github.com/michaldrobot/ShaderFastLibs
//!
//! # References:
//! - [0] Handbook of Mathematical Functions (chapter : Elementary Transcendental Functions), M. Abramowitz and I.A. Stegun, Ed.
//! - [1] Efficient approximations for the arctangent function, Rajan, S. Sichun Wang Inkol, R. Joyal, A., May 2006
//!
//! # Info
//!
//! This is a CPU implementation of the fast math ops implemented in 'drobot_math.hlsl'. These are
//! typically _not_ faster than using the real hardware intrinsics on the CPU, so they're unlikely
//! to give you performance wins there. However having these functions available on the CPU is
//! useful for inspecting the output.

use std::f32::consts::{FRAC_PI_2, PI};

// Derived from batch testing
pub const IEEE_INT_RCP_CONST_NR0: i32 = 0x7EF311C2;
pub const IEEE_INT_RCP_CONST_NR1: i32 = 0x7EF311C3;
pub const IEEE_INT_RCP_CONST_NR2: i32 = 0x7EF312AC;

// Derived from batch testing
pub const IEEE_INT_SQRT_CONST_NR0: i32 = 0x1FBD1DF5;

// Biases for global ranges
// 0-1 or 1-2 specific ranges might improve from different bias
// Derived from batch testing
pub const IEEE_INT_RCP_SQRT_CONST_NR0: i32 = 0x5f3759df;
pub const IEEE_INT_RCP_SQRT_CONST_NR1: i32 = 0x5F375A86;
pub const IEEE_INT_RCP_SQRT_CONST_NR2: i32 = 0x5F375A86;

// //
// // Normalized range [0,1] Constants
// //
// pub const IEEE_INT_RCP_CONST_NR0_SNORM: i32 = 0x7EEF370B;
// pub const IEEE_INT_SQRT_CONST_NR0_SNORM: i32 = 0x1FBD1DF5;
// pub const IEEE_INT_RCP_SQRT_CONST_NR0_SNORM: i32 = 0x5F341A43;

//
// Distance [0,1000] based constants
//
pub const IEEE_INT_RCP_CONST_NR0_SNORM: i32 = 0x7EF3210C;
pub const IEEE_INT_SQRT_CONST_NR0_SNORM: i32 = 0x1FBD22DF;
pub const IEEE_INT_RCP_SQRT_CONST_NR0_SNORM: i32 = 0x5F33E79F;

/// Approximate guess using integer float arithmetics based on IEEE floating point standard.
#[inline]
pub fn fast_inv_sqrt(in_x: f32, in_rcp_sqrt_const: i32) -> f32 {
    let x: i32 = bytemuck::cast(in_x);
    let x = in_rcp_sqrt_const - (x >> 1);
    bytemuck::cast(x)
}

#[inline]
pub fn rcp_sqrt_newton_raphson(in_xhalf: f32, in_rcp_x: f32) -> f32 {
    in_rcp_x * (-in_xhalf * (in_rcp_x * in_rcp_x) + 1.5)
}

/// Using 0 Newton Raphson iterations
/// Relative error : ~3.4% over full
/// Precise format : ~small float
/// 2 ALU
#[inline]
pub fn fast_rcp_sqrt_nr0(in_x: f32) -> f32 {
    fast_inv_sqrt(in_x, IEEE_INT_RCP_SQRT_CONST_NR0)
}

/// Using 1 Newton Raphson iterations
/// Relative error : ~0.2% over full
/// Precise format : ~half float
/// 6 ALU
#[inline]
pub fn fast_rcp_sqrt_nr1(in_x: f32) -> f32 {
    let xhalf = 0.5 * in_x;
    let x_rcp_sqrt = fast_inv_sqrt(in_x, IEEE_INT_RCP_SQRT_CONST_NR1);
    rcp_sqrt_newton_raphson(xhalf, x_rcp_sqrt)
}

/// Using 2 Newton Raphson iterations
/// Relative error : ~4.6e-004%  over full
/// Precise format : ~full float
/// 9 ALU
#[inline]
pub fn fast_rcp_sqrt_nr2(in_x: f32) -> f32 {
    let xhalf = 0.5 * in_x;
    let x_rcp_sqrt = fast_inv_sqrt(in_x, IEEE_INT_RCP_SQRT_CONST_NR2);
    let x_rcp_sqrt = rcp_sqrt_newton_raphson(xhalf, x_rcp_sqrt);
    rcp_sqrt_newton_raphson(xhalf, x_rcp_sqrt)
}

/// Approximate guess using integer float arithmetics based on IEEE floating point standard.
#[inline]
pub fn fast_sqrt(in_x: f32, in_sqrt_const: i32) -> f32 {
    let x: i32 = bytemuck::cast(in_x);
    let x = in_sqrt_const + (x >> 1);
    bytemuck::cast(x)
}

/// Using 0 Newton Raphson iterations
/// Relative error : < 0.7% over full
/// Precise format : ~small float
/// 1 ALU
#[inline]
pub fn fast_sqrt_nr0(in_x: f32) -> f32 {
    fast_sqrt(in_x, IEEE_INT_SQRT_CONST_NR0)
}

/// Use inverse Rcp Sqrt
/// Using 1 Newton Raphson iterations
/// Relative error : ~0.2% over full
/// Precise format : ~half float
/// 6 ALU
#[inline]
pub fn fast_sqrt_nr1(in_x: f32) -> f32 {
    // Inverse Rcp Sqrt
    in_x * fast_rcp_sqrt_nr1(in_x)
}

/// Use inverse Rcp Sqrt
/// Using 2 Newton Raphson iterations
/// Relative error : ~4.6e-004%  over full
/// Precise format : ~full float
/// 9 ALU
#[inline]
pub fn fast_sqrt_nr2(in_x: f32) -> f32 {
    // Inverse Rcp Sqrt
    in_x * fast_rcp_sqrt_nr2(in_x)
}

#[inline]
pub fn rcp_ieee_int_approximation(in_x: f32, in_rcp_const: i32) -> f32 {
    let x: i32 = bytemuck::cast(in_x);
    let x = in_rcp_const - x;
    bytemuck::cast(x)
}

#[inline]
pub fn rcp_newton_raphson(in_x: f32, in_rcp_x: f32) -> f32 {
    in_rcp_x * (-in_rcp_x * in_x + 2.0)
}

/// Using 0 Newton Raphson iterations
/// Relative error : < 0.4% over full
/// Precise format : ~small float
/// 1 ALU
#[inline]
pub fn fast_rcp_nr0(in_x: f32) -> f32 {
    rcp_ieee_int_approximation(in_x, IEEE_INT_RCP_CONST_NR0)
}

/// Using 1 Newton Raphson iterations
/// Relative error : < 0.02% over full
/// Precise format : ~half float
/// 3 ALU
#[inline]
pub fn fast_rcp_nr1(in_x: f32) -> f32 {
    let x_rcp = rcp_ieee_int_approximation(in_x, IEEE_INT_RCP_CONST_NR1);
    rcp_newton_raphson(in_x, x_rcp)
}

/// Using 2 Newton Raphson iterations
/// Relative error : < 5.0e-005%  over full
/// Precise format : ~full float
/// 5 ALU
#[inline]
pub fn fast_rcp_nr2(in_x: f32) -> f32 {
    let x_rcp = rcp_ieee_int_approximation(in_x, IEEE_INT_RCP_CONST_NR2);
    let x_rcp = rcp_newton_raphson(in_x, x_rcp);
    rcp_newton_raphson(in_x, x_rcp)
}

/// 4th order polynomial approximation
/// 4 VGRP, 16 ALU Full Rate
/// 7 * 10^-5 radians precision
/// Reference : [0]
#[inline]
pub fn fast_acos4(in_x: f32) -> f32 {
    let x1 = f32::abs(in_x);
    let x2 = x1 * x1;
    let x3 = x2 * x1;

    let s = -0.2121144 * x1 + 1.5707288;
    let s = 0.0742610 * x2 + s;
    let s = -0.0187293 * x3 + s;
    let s = f32::sqrt(1.0 - x1) * s;

    // acos function mirroring
    // check per platform if compiles to a selector - no branch neeeded
    if in_x >= 0.0 { s } else { PI - s }
}

/// 4th order polynomial approximation
/// 4 VGRP, 16 ALU Full Rate
/// 7 * 10^-5 radians precision
#[inline]
pub fn fast_asin4(in_x: f32) -> f32 {
    let x = in_x;

    // asin is offset of acos
    FRAC_PI_2 - fast_acos4(x)
}

/// 4th order hyperbolical approximation
/// 4 VGRP, 12 ALU Full Rate
/// 7 * 10^-5 radians precision
/// Reference : [1]
#[inline]
pub fn fast_atan4(in_x: f32) -> f32 {
    let x = in_x;
    x * (-0.1784 * f32::abs(x) - 0.0663 * x * x + 1.0301)
}
