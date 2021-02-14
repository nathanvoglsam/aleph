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

use raw::windows::win32::direct3d12::D3D12_BLEND;

#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug, Hash)]
pub enum Blend {
    Zero,
    One,
    SrcColor,
    SrcColorInv,
    SrcAlpha,
    SrcAlphaInv,
    DestAlpha,
    DestAlphaInv,
    DestColor,
    DestColorInv,
    SrcAlphaSaturated,
    BlendFactor,
    BlendFactorInv,
    Src1Color,
    Src1ColorInv,
    Src1Alpha,
    Src1AlphaInv,
}

impl Into<D3D12_BLEND> for Blend {
    fn into(self) -> D3D12_BLEND {
        match self {
            Blend::Zero => D3D12_BLEND::D3D12_BLEND_ZERO,
            Blend::One => D3D12_BLEND::D3D12_BLEND_ONE,
            Blend::SrcColor => D3D12_BLEND::D3D12_BLEND_SRC_COLOR,
            Blend::SrcColorInv => D3D12_BLEND::D3D12_BLEND_INV_SRC_COLOR,
            Blend::SrcAlpha => D3D12_BLEND::D3D12_BLEND_SRC_ALPHA,
            Blend::SrcAlphaInv => D3D12_BLEND::D3D12_BLEND_INV_SRC_ALPHA,
            Blend::DestAlpha => D3D12_BLEND::D3D12_BLEND_DEST_ALPHA,
            Blend::DestAlphaInv => D3D12_BLEND::D3D12_BLEND_INV_DEST_ALPHA,
            Blend::DestColor => D3D12_BLEND::D3D12_BLEND_DEST_COLOR,
            Blend::DestColorInv => D3D12_BLEND::D3D12_BLEND_INV_DEST_COLOR,
            Blend::SrcAlphaSaturated => D3D12_BLEND::D3D12_BLEND_SRC_ALPHA_SAT,
            Blend::BlendFactor => D3D12_BLEND::D3D12_BLEND_BLEND_FACTOR,
            Blend::BlendFactorInv => D3D12_BLEND::D3D12_BLEND_INV_BLEND_FACTOR,
            Blend::Src1Color => D3D12_BLEND::D3D12_BLEND_SRC1_COLOR,
            Blend::Src1ColorInv => D3D12_BLEND::D3D12_BLEND_INV_SRC1_COLOR,
            Blend::Src1Alpha => D3D12_BLEND::D3D12_BLEND_SRC1_ALPHA,
            Blend::Src1AlphaInv => D3D12_BLEND::D3D12_BLEND_INV_SRC1_ALPHA,
        }
    }
}
