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

use crate::matrix::TMat4x4;
use crate::vector::{TVec2, TVec3, TVec4};

use crate::quaternion::TQuat;

///
/// A 2 component f32 vector
///
pub type Vec2 = TVec2<f32>;

///
/// A 3 component f32 vector
///
pub type Vec3 = TVec3<f32>;

///
/// A 4 component f32 vector
///
pub type Vec4 = TVec4<f32>;

///
/// A 2 component f64 vector
///
pub type DVec2 = TVec2<f64>;

///
/// A 3 component f64 vector
///
pub type DVec3 = TVec3<f64>;

///
/// A 4 component f64 vector
///
pub type DVec4 = TVec4<f64>;

///
/// A f32 quaternion
///
pub type Quat = TQuat<f32>;

///
/// A f64 quaternion
///
pub type DQuat = TQuat<f64>;

///
/// A 4x4 f32 matrix
///
pub type Mat4x4 = TMat4x4<f32>;

///
/// A 4x4 f64 matrix
///
pub type DMat4x4 = TMat4x4<f64>;
