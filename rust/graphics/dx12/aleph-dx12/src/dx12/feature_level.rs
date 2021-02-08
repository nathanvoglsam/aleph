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

use crate::raw::windows::win32::direct3d11::D3D_FEATURE_LEVEL;

/// Maps to `D3D_FEATURE_LEVEL`
///
/// # Info
///
/// Using camel case for the names would make the names too difficult to read as there would be no
/// way to separate the version parts (i.e 9_1 to 91)
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub enum FeatureLevel {
    Level_1_0_Core,
    Level_9_1,
    Level_9_2,
    Level_9_3,
    Level_10_0,
    Level_10_1,
    Level_11_0,
    Level_11_1,
    Level_12_0,
    Level_12_1,
}

impl Into<D3D_FEATURE_LEVEL> for FeatureLevel {
    fn into(self) -> D3D_FEATURE_LEVEL {
        match self {
            FeatureLevel::Level_1_0_Core => D3D_FEATURE_LEVEL::D3D_FEATURE_LEVEL_1_0_CORE,
            FeatureLevel::Level_9_1 => D3D_FEATURE_LEVEL::D3D_FEATURE_LEVEL_9_1,
            FeatureLevel::Level_9_2 => D3D_FEATURE_LEVEL::D3D_FEATURE_LEVEL_9_2,
            FeatureLevel::Level_9_3 => D3D_FEATURE_LEVEL::D3D_FEATURE_LEVEL_9_3,
            FeatureLevel::Level_10_0 => D3D_FEATURE_LEVEL::D3D_FEATURE_LEVEL_10_0,
            FeatureLevel::Level_10_1 => D3D_FEATURE_LEVEL::D3D_FEATURE_LEVEL_10_1,
            FeatureLevel::Level_11_0 => D3D_FEATURE_LEVEL::D3D_FEATURE_LEVEL_11_0,
            FeatureLevel::Level_11_1 => D3D_FEATURE_LEVEL::D3D_FEATURE_LEVEL_11_1,
            FeatureLevel::Level_12_0 => D3D_FEATURE_LEVEL::D3D_FEATURE_LEVEL_12_0,
            FeatureLevel::Level_12_1 => D3D_FEATURE_LEVEL::D3D_FEATURE_LEVEL_12_1,
        }
    }
}
