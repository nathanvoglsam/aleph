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

use windows_raw::Win32::Direct3D12::D3D12_FILTER;

#[repr(i32)]
#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug, Hash)]
pub enum Filter {
    MinMagMipPoint = 0,
    MinMagPointMipLinear = 1,
    MinPointMagLinearMipPoint = 4,
    MinPointMagMipLinear = 5,
    MinLinearMagMipPoint = 16,
    MinLinearMagPointMipLinear = 17,
    MinMagLinearMipPoint = 20,
    MinMagMipLinear = 21,
    Anisotropic = 85,
    ComparisonMinMagMipPoint = 128,
    ComparisonMinMagPointMipLinear = 129,
    ComparisonMinPointMagLinearMipPoint = 132,
    ComparisonMinPointMagMipLinear = 133,
    ComparisonMinLinearMagMipPoint = 144,
    ComparisonMinLinearMagPointMipLinear = 145,
    ComparisonMinMagLinearMipPoint = 148,
    ComparisonMinMagMipLinear = 149,
    ComparisonAnisotropic = 213,
    MinimumMinMagMipPoint = 256,
    MinimumMinMagPointMipLinear = 257,
    MinimumMinPointMagLinearMipPoint = 260,
    MinimumMinPointMagMipLinear = 261,
    MinimumMinLinearMagMipPoint = 272,
    MinimumMinLinearMagPointMipLinear = 273,
    MinimumMinMagLinearMipPoint = 276,
    MinimumMinMagMipLinear = 277,
    MinimumAnisotropic = 341,
    MaximumMinMagMipPoint = 384,
    MaximumMinMagPointMipLinear = 385,
    MaximumMinPointMagLinearMipPoint = 388,
    MaximumMinPointMagMipLinear = 389,
    MaximumMinLinearMagMipPoint = 400,
    MaximumMinLinearMagPointMipLinear = 401,
    MaximumMinMagLinearMipPoint = 404,
    MaximumMinMagMipLinear = 405,
    MaximumAnisotropic = 469,
}

impl Into<D3D12_FILTER> for Filter {
    #[inline]
    fn into(self) -> D3D12_FILTER {
        D3D12_FILTER(self as i32)
    }
}
