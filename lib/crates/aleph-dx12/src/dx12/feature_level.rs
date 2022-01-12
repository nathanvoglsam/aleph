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

use windows::Win32::Graphics::Direct3D::D3D_FEATURE_LEVEL;

/// Maps to `D3D_FEATURE_LEVEL`
///
/// # Info
///
/// Using camel case for the names would make the names too difficult to read as there would be no
/// way to separate the version parts (i.e 9_1 to 91)
#[repr(i32)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug, Hash)]
pub enum FeatureLevel {
    Level_1_0_Core = 4096,
    Level_9_1 = 37120,
    Level_9_2 = 37376,
    Level_9_3 = 37632,
    Level_10_0 = 40960,
    Level_10_1 = 41216,
    Level_11_0 = 45056,
    Level_11_1 = 45312,
    Level_12_0 = 49152,
    Level_12_1 = 49408,
    Level_12_2 = 49664,
}

impl Into<D3D_FEATURE_LEVEL> for FeatureLevel {
    #[inline]
    fn into(self) -> D3D_FEATURE_LEVEL {
        self as i32
    }
}
