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

use crate::dxgi;
use std::mem::transmute;
use windows::Win32::Graphics::Direct3D12::{
    D3D12_DEPTH_STENCIL_VIEW_DESC, D3D12_DEPTH_STENCIL_VIEW_DESC_0, D3D12_DSV_DIMENSION_TEXTURE1D,
    D3D12_DSV_DIMENSION_TEXTURE1DARRAY, D3D12_DSV_DIMENSION_TEXTURE2D,
    D3D12_DSV_DIMENSION_TEXTURE2DARRAY, D3D12_DSV_DIMENSION_TEXTURE2DMS,
    D3D12_DSV_DIMENSION_TEXTURE2DMSARRAY, D3D12_DSV_FLAG_NONE,
};

#[derive(Clone, Debug)]
pub enum DepthStencilViewDesc {
    Texture1D {
        format: dxgi::Format,
        texture_1d: Tex1DDsv,
    },
    Texture1DArray {
        format: dxgi::Format,
        texture_1d_array: Tex1DArrayDsv,
    },
    Texture2D {
        format: dxgi::Format,
        texture_2d: Tex2DDsv,
    },
    Texture2DArray {
        format: dxgi::Format,
        texture_2d_array: Tex2DArrayDsv,
    },
    Texture2DMS {
        format: dxgi::Format,
        texture_2dms: Tex2DMSDsv,
    },
    Texture2DMSArray {
        format: dxgi::Format,
        texture_2dms_array: Tex2DMSArrayDsv,
    },
}

impl From<DepthStencilViewDesc> for D3D12_DEPTH_STENCIL_VIEW_DESC {
    #[inline]
    fn from(v: DepthStencilViewDesc) -> Self {
        match v {
            DepthStencilViewDesc::Texture1D { format, texture_1d } => {
                D3D12_DEPTH_STENCIL_VIEW_DESC {
                    Format: format.into(),
                    ViewDimension: D3D12_DSV_DIMENSION_TEXTURE1D,
                    Flags: D3D12_DSV_FLAG_NONE,
                    Anonymous: D3D12_DEPTH_STENCIL_VIEW_DESC_0 {
                        Texture1D: unsafe { transmute(texture_1d) },
                    },
                }
            }
            DepthStencilViewDesc::Texture1DArray {
                format,
                texture_1d_array,
            } => D3D12_DEPTH_STENCIL_VIEW_DESC {
                Format: format.into(),
                ViewDimension: D3D12_DSV_DIMENSION_TEXTURE1DARRAY,
                Flags: D3D12_DSV_FLAG_NONE,
                Anonymous: D3D12_DEPTH_STENCIL_VIEW_DESC_0 {
                    Texture1DArray: unsafe { transmute(texture_1d_array) },
                },
            },
            DepthStencilViewDesc::Texture2D { format, texture_2d } => {
                D3D12_DEPTH_STENCIL_VIEW_DESC {
                    Format: format.into(),
                    ViewDimension: D3D12_DSV_DIMENSION_TEXTURE2D,
                    Flags: D3D12_DSV_FLAG_NONE,
                    Anonymous: D3D12_DEPTH_STENCIL_VIEW_DESC_0 {
                        Texture2D: unsafe { transmute(texture_2d) },
                    },
                }
            }
            DepthStencilViewDesc::Texture2DArray {
                format,
                texture_2d_array,
            } => D3D12_DEPTH_STENCIL_VIEW_DESC {
                Format: format.into(),
                ViewDimension: D3D12_DSV_DIMENSION_TEXTURE2DARRAY,
                Flags: D3D12_DSV_FLAG_NONE,
                Anonymous: D3D12_DEPTH_STENCIL_VIEW_DESC_0 {
                    Texture2DArray: unsafe { transmute(texture_2d_array) },
                },
            },
            DepthStencilViewDesc::Texture2DMS {
                format,
                texture_2dms,
            } => D3D12_DEPTH_STENCIL_VIEW_DESC {
                Format: format.into(),
                ViewDimension: D3D12_DSV_DIMENSION_TEXTURE2DMS,
                Flags: D3D12_DSV_FLAG_NONE,
                Anonymous: D3D12_DEPTH_STENCIL_VIEW_DESC_0 {
                    Texture2DMS: unsafe { transmute(texture_2dms) },
                },
            },
            DepthStencilViewDesc::Texture2DMSArray {
                format,
                texture_2dms_array,
            } => D3D12_DEPTH_STENCIL_VIEW_DESC {
                Format: format.into(),
                ViewDimension: D3D12_DSV_DIMENSION_TEXTURE2DMSARRAY,
                Flags: D3D12_DSV_FLAG_NONE,
                Anonymous: D3D12_DEPTH_STENCIL_VIEW_DESC_0 {
                    Texture2DMSArray: unsafe { transmute(texture_2dms_array) },
                },
            },
        }
    }
}

#[repr(C)]
#[derive(Clone, Default, Eq, PartialEq, Debug)]
pub struct Tex1DDsv {
    pub mip_slice: u32,
}

#[repr(C)]
#[derive(Clone, Default, Eq, PartialEq, Debug)]
pub struct Tex1DArrayDsv {
    pub mip_slice: u32,
    pub first_array_slice: u32,
    pub array_size: u32,
}

#[repr(C)]
#[derive(Clone, Default, Eq, PartialEq, Debug)]
pub struct Tex2DDsv {
    pub mip_slice: u32,
}

#[repr(C)]
#[derive(Clone, Default, Eq, PartialEq, Debug)]
pub struct Tex2DArrayDsv {
    pub mip_slice: u32,
    pub first_array_slice: u32,
    pub array_size: u32,
}

#[repr(C)]
#[derive(Clone, Default, Eq, PartialEq, Debug)]
pub struct Tex2DMSDsv {
    pub _unused: u32,
}

#[repr(C)]
#[derive(Clone, Default, Eq, PartialEq, Debug)]
pub struct Tex2DMSArrayDsv {
    pub first_array_slice: u32,
    pub array_size: u32,
}
