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
use windows_raw::win32::direct3d12::{
    D3D12_BUFFER_RTV, D3D12_RTV_DIMENSION, D3D12_TEX1D_ARRAY_RTV, D3D12_TEX1D_RTV,
    D3D12_TEX2DMS_ARRAY_RTV, D3D12_TEX2DMS_RTV, D3D12_TEX2D_ARRAY_RTV, D3D12_TEX2D_RTV,
    D3D12_TEX3D_RTV,
};
use windows_raw::win32::dxgi::DXGI_FORMAT;
use std::mem::{transmute, ManuallyDrop};

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

impl Into<D3D12_RENDER_TARGET_VIEW_DESC> for RenderTargetViewDesc {
    fn into(self) -> D3D12_RENDER_TARGET_VIEW_DESC {
        match self {
            RenderTargetViewDesc::Buffer { format, buffer } => D3D12_RENDER_TARGET_VIEW_DESC {
                format: format.into(),
                view_dimension: D3D12_RTV_DIMENSION::D3D12_RTV_DIMENSION_BUFFER,
                variant: D3D12_RENDER_TARGET_VIEW_DESC_VARIANT {
                    buffer: ManuallyDrop::new(unsafe { transmute(buffer) }),
                },
            },
            RenderTargetViewDesc::Texture1D { format, texture_1d } => {
                D3D12_RENDER_TARGET_VIEW_DESC {
                    format: format.into(),
                    view_dimension: D3D12_RTV_DIMENSION::D3D12_RTV_DIMENSION_TEXTURE1D,
                    variant: D3D12_RENDER_TARGET_VIEW_DESC_VARIANT {
                        texture_1d: ManuallyDrop::new(texture_1d),
                    },
                }
            }
            RenderTargetViewDesc::Texture1DArray {
                format,
                texture_1d_array,
            } => D3D12_RENDER_TARGET_VIEW_DESC {
                format: format.into(),
                view_dimension: D3D12_RTV_DIMENSION::D3D12_RTV_DIMENSION_TEXTURE1DARRAY,
                variant: D3D12_RENDER_TARGET_VIEW_DESC_VARIANT {
                    texture_1d_array: ManuallyDrop::new(texture_1d_array),
                },
            },
            RenderTargetViewDesc::Texture2D { format, texture_2d } => {
                D3D12_RENDER_TARGET_VIEW_DESC {
                    format: format.into(),
                    view_dimension: D3D12_RTV_DIMENSION::D3D12_RTV_DIMENSION_TEXTURE2D,
                    variant: D3D12_RENDER_TARGET_VIEW_DESC_VARIANT {
                        texture_2d: ManuallyDrop::new(texture_2d),
                    },
                }
            }
            RenderTargetViewDesc::Texture2DArray {
                format,
                texture_2d_array,
            } => D3D12_RENDER_TARGET_VIEW_DESC {
                format: format.into(),
                view_dimension: D3D12_RTV_DIMENSION::D3D12_RTV_DIMENSION_TEXTURE2DARRAY,
                variant: D3D12_RENDER_TARGET_VIEW_DESC_VARIANT {
                    texture_2d_array: ManuallyDrop::new(texture_2d_array),
                },
            },
            RenderTargetViewDesc::Texture2DMS {
                format,
                texture_2dms,
            } => D3D12_RENDER_TARGET_VIEW_DESC {
                format: format.into(),
                view_dimension: D3D12_RTV_DIMENSION::D3D12_RTV_DIMENSION_TEXTURE2DMS,
                variant: D3D12_RENDER_TARGET_VIEW_DESC_VARIANT {
                    texture_2dms: ManuallyDrop::new(texture_2dms),
                },
            },
            RenderTargetViewDesc::Texture2DMSArray {
                format,
                texture_2dms_array,
            } => D3D12_RENDER_TARGET_VIEW_DESC {
                format: format.into(),
                view_dimension: D3D12_RTV_DIMENSION::D3D12_RTV_DIMENSION_TEXTURE2DMSARRAY,
                variant: D3D12_RENDER_TARGET_VIEW_DESC_VARIANT {
                    texture_2dms_array: ManuallyDrop::new(texture_2dms_array),
                },
            },
            RenderTargetViewDesc::Texture3D { format, texture_3d } => {
                D3D12_RENDER_TARGET_VIEW_DESC {
                    format: format.into(),
                    view_dimension: D3D12_RTV_DIMENSION::D3D12_RTV_DIMENSION_TEXTURE3D,
                    variant: D3D12_RENDER_TARGET_VIEW_DESC_VARIANT {
                        texture_3d: ManuallyDrop::new(texture_3d),
                    },
                }
            }
        }
    }
}

pub type BufferRtv = D3D12_BUFFER_RTV;
pub type Tex1DRtv = D3D12_TEX1D_RTV;
pub type Tex1DArrayRtv = D3D12_TEX1D_ARRAY_RTV;
pub type Tex2DRtv = D3D12_TEX2D_RTV;
pub type Tex2DArrayRtv = D3D12_TEX2D_ARRAY_RTV;
pub type Tex2DMSRtv = D3D12_TEX2DMS_RTV;
pub type Tex2DMSArrayRtv = D3D12_TEX2DMS_ARRAY_RTV;
pub type Tex3DRtv = D3D12_TEX3D_RTV;

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D12_RENDER_TARGET_VIEW_DESC {
    pub format: DXGI_FORMAT,
    pub view_dimension: D3D12_RTV_DIMENSION,
    pub variant: D3D12_RENDER_TARGET_VIEW_DESC_VARIANT,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub union D3D12_RENDER_TARGET_VIEW_DESC_VARIANT {
    pub buffer: ManuallyDrop<D3D12_BUFFER_RTV>,
    pub texture_1d: ManuallyDrop<D3D12_TEX1D_RTV>,
    pub texture_1d_array: ManuallyDrop<D3D12_TEX1D_ARRAY_RTV>,
    pub texture_2d: ManuallyDrop<D3D12_TEX2D_RTV>,
    pub texture_2d_array: ManuallyDrop<D3D12_TEX2D_ARRAY_RTV>,
    pub texture_2dms: ManuallyDrop<D3D12_TEX2DMS_RTV>,
    pub texture_2dms_array: ManuallyDrop<D3D12_TEX2DMS_ARRAY_RTV>,
    pub texture_3d: ManuallyDrop<D3D12_TEX3D_RTV>,
}
