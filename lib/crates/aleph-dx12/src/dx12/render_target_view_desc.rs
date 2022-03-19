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
    D3D12_RENDER_TARGET_VIEW_DESC, D3D12_RENDER_TARGET_VIEW_DESC_0, D3D12_RTV_DIMENSION_BUFFER,
    D3D12_RTV_DIMENSION_TEXTURE1D, D3D12_RTV_DIMENSION_TEXTURE1DARRAY,
    D3D12_RTV_DIMENSION_TEXTURE2D, D3D12_RTV_DIMENSION_TEXTURE2DARRAY,
    D3D12_RTV_DIMENSION_TEXTURE2DMS, D3D12_RTV_DIMENSION_TEXTURE2DMSARRAY,
    D3D12_RTV_DIMENSION_TEXTURE3D,
};

#[derive(Clone, Debug)]
pub enum RenderTargetViewDesc {
    Buffer {
        format: dxgi::Format,
        buffer: BufferRtv,
    },
    Texture1D {
        format: dxgi::Format,
        texture_1d: Tex1DRtv,
    },
    Texture1DArray {
        format: dxgi::Format,
        texture_1d_array: Tex1DArrayRtv,
    },
    Texture2D {
        format: dxgi::Format,
        texture_2d: Tex2DRtv,
    },
    Texture2DArray {
        format: dxgi::Format,
        texture_2d_array: Tex2DArrayRtv,
    },
    Texture2DMS {
        format: dxgi::Format,
        texture_2dms: Tex2DMSRtv,
    },
    Texture2DMSArray {
        format: dxgi::Format,
        texture_2dms_array: Tex2DMSArrayRtv,
    },
    Texture3D {
        format: dxgi::Format,
        texture_3d: Tex3DRtv,
    },
}

impl From<RenderTargetViewDesc> for D3D12_RENDER_TARGET_VIEW_DESC {
    #[inline]
    fn from(v: RenderTargetViewDesc) -> Self {
        match v {
            RenderTargetViewDesc::Buffer { format, buffer } => D3D12_RENDER_TARGET_VIEW_DESC {
                Format: format.into(),
                ViewDimension: D3D12_RTV_DIMENSION_BUFFER,
                Anonymous: D3D12_RENDER_TARGET_VIEW_DESC_0 {
                    Buffer: unsafe { transmute(buffer) },
                },
            },
            RenderTargetViewDesc::Texture1D { format, texture_1d } => {
                D3D12_RENDER_TARGET_VIEW_DESC {
                    Format: format.into(),
                    ViewDimension: D3D12_RTV_DIMENSION_TEXTURE1D,
                    Anonymous: D3D12_RENDER_TARGET_VIEW_DESC_0 {
                        Texture1D: unsafe { transmute(texture_1d) },
                    },
                }
            }
            RenderTargetViewDesc::Texture1DArray {
                format,
                texture_1d_array,
            } => D3D12_RENDER_TARGET_VIEW_DESC {
                Format: format.into(),
                ViewDimension: D3D12_RTV_DIMENSION_TEXTURE1DARRAY,
                Anonymous: D3D12_RENDER_TARGET_VIEW_DESC_0 {
                    Texture1DArray: unsafe { transmute(texture_1d_array) },
                },
            },
            RenderTargetViewDesc::Texture2D { format, texture_2d } => {
                D3D12_RENDER_TARGET_VIEW_DESC {
                    Format: format.into(),
                    ViewDimension: D3D12_RTV_DIMENSION_TEXTURE2D,
                    Anonymous: D3D12_RENDER_TARGET_VIEW_DESC_0 {
                        Texture2D: unsafe { transmute(texture_2d) },
                    },
                }
            }
            RenderTargetViewDesc::Texture2DArray {
                format,
                texture_2d_array,
            } => D3D12_RENDER_TARGET_VIEW_DESC {
                Format: format.into(),
                ViewDimension: D3D12_RTV_DIMENSION_TEXTURE2DARRAY,
                Anonymous: D3D12_RENDER_TARGET_VIEW_DESC_0 {
                    Texture2DArray: unsafe { transmute(texture_2d_array) },
                },
            },
            RenderTargetViewDesc::Texture2DMS {
                format,
                texture_2dms,
            } => D3D12_RENDER_TARGET_VIEW_DESC {
                Format: format.into(),
                ViewDimension: D3D12_RTV_DIMENSION_TEXTURE2DMS,
                Anonymous: D3D12_RENDER_TARGET_VIEW_DESC_0 {
                    Texture2DMS: unsafe { transmute(texture_2dms) },
                },
            },
            RenderTargetViewDesc::Texture2DMSArray {
                format,
                texture_2dms_array,
            } => D3D12_RENDER_TARGET_VIEW_DESC {
                Format: format.into(),
                ViewDimension: D3D12_RTV_DIMENSION_TEXTURE2DMSARRAY,
                Anonymous: D3D12_RENDER_TARGET_VIEW_DESC_0 {
                    Texture2DMSArray: unsafe { transmute(texture_2dms_array) },
                },
            },
            RenderTargetViewDesc::Texture3D { format, texture_3d } => {
                D3D12_RENDER_TARGET_VIEW_DESC {
                    Format: format.into(),
                    ViewDimension: D3D12_RTV_DIMENSION_TEXTURE3D,
                    Anonymous: D3D12_RENDER_TARGET_VIEW_DESC_0 {
                        Texture3D: unsafe { transmute(texture_3d) },
                    },
                }
            }
        }
    }
}

#[repr(C)]
#[derive(Clone, Default, Eq, PartialEq, Debug)]
pub struct BufferRtv {
    pub first_element: u64,
    pub num_elements: u32,
}

#[repr(C)]
#[derive(Clone, Default, Eq, PartialEq, Debug)]
pub struct Tex1DRtv {
    pub mip_slice: u32,
}

#[repr(C)]
#[derive(Clone, Default, Eq, PartialEq, Debug)]
pub struct Tex1DArrayRtv {
    pub mip_slice: u32,
    pub first_array_slice: u32,
    pub array_size: u32,
}

#[repr(C)]
#[derive(Clone, Default, Eq, PartialEq, Debug)]
pub struct Tex2DRtv {
    pub mip_slice: u32,
    pub plane_slice: u32,
}

#[repr(C)]
#[derive(Clone, Default, Eq, PartialEq, Debug)]
pub struct Tex2DArrayRtv {
    pub mip_slice: u32,
    pub first_array_slice: u32,
    pub array_size: u32,
    pub plane_slice: u32,
}

#[repr(C)]
#[derive(Clone, Default, Eq, PartialEq, Debug)]
pub struct Tex2DMSRtv {
    pub _unused: u32,
}

#[repr(C)]
#[derive(Clone, Default, Eq, PartialEq, Debug)]
pub struct Tex2DMSArrayRtv {
    pub first_array_slice: u32,
    pub array_size: u32,
}

#[repr(C)]
#[derive(Clone, Default, Eq, PartialEq, Debug)]
pub struct Tex3DRtv {
    pub mip_slice: u32,
    pub first_w_slice: u32,
    pub w_size: u32,
}
