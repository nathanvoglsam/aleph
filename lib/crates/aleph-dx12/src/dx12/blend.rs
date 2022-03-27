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

use windows::Win32::Graphics::Direct3D12::D3D12_BLEND;

#[repr(i32)]
#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug, Hash)]
pub enum Blend {
    Zero = 1,
    One = 2,
    SrcColor = 3,
    SrcColorInv = 4,
    SrcAlpha = 5,
    SrcAlphaInv = 6,
    DestAlpha = 7,
    DestAlphaInv = 8,
    DestColor = 9,
    DestColorInv = 10,
    SrcAlphaSaturated = 11,
    BlendFactor = 14,
    BlendFactorInv = 15,
    Src1Color = 16,
    Src1ColorInv = 17,
    Src1Alpha = 18,
    Src1AlphaInv = 19,
}

impl From<Blend> for D3D12_BLEND {
    #[inline]
    fn from(v: Blend) -> Self {
        D3D12_BLEND(v as _)
    }
}
