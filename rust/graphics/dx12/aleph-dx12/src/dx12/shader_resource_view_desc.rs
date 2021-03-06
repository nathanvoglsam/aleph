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
    D3D12_BUFFER_SRV, D3D12_RAYTRACING_ACCELERATION_STRUCTURE_SRV, D3D12_SRV_DIMENSION,
    D3D12_TEX1D_ARRAY_SRV, D3D12_TEX1D_SRV, D3D12_TEX2DMS_ARRAY_SRV, D3D12_TEX2DMS_SRV,
    D3D12_TEX2D_ARRAY_SRV, D3D12_TEX2D_SRV, D3D12_TEX3D_SRV, D3D12_TEXCUBE_ARRAY_SRV,
    D3D12_TEXCUBE_SRV,
};
use windows_raw::win32::dxgi::DXGI_FORMAT;
use std::mem::{transmute, ManuallyDrop};

#[derive(Clone, Debug)]
pub enum ShaderResourceViewDesc {
    Buffer {
        format: dxgi::Format,
        component_mapping: u32,
        buffer: BufferSrv,
    },
    Texture1D {
        format: dxgi::Format,
        component_mapping: u32,
        texture_1d: Tex1DSrv,
    },
    Texture1DArray {
        format: dxgi::Format,
        component_mapping: u32,
        texture_1d_array: Tex1DArraySrv,
    },
    Texture2D {
        format: dxgi::Format,
        component_mapping: u32,
        texture_2d: Tex2DSrv,
    },
    Texture2DArray {
        format: dxgi::Format,
        component_mapping: u32,
        texture_2d_array: Tex2DArraySrv,
    },
    Texture2DMS {
        format: dxgi::Format,
        component_mapping: u32,
        texture_2dms: Tex2DMSSrv,
    },
    Texture2DMSArray {
        format: dxgi::Format,
        component_mapping: u32,
        texture_2dms_array: Tex2DMSArraySrv,
    },
    Texture3D {
        format: dxgi::Format,
        component_mapping: u32,
        texture_3d: Tex3DSrv,
    },
    TextureCube {
        format: dxgi::Format,
        component_mapping: u32,
        texture_cube: TexCubeSrv,
    },
    TextureCubeArray {
        format: dxgi::Format,
        component_mapping: u32,
        texture_cube_array: TexCubeArraySrv,
    },
    RaytracingAccelerationStructure {
        format: dxgi::Format,
        component_mapping: u32,
        raytracing_acceleration_structure: RaytracingAccelerationStructureSrv,
    },
}

impl Into<D3D12_SHADER_RESOURCE_VIEW_DESC> for ShaderResourceViewDesc {
    fn into(self) -> D3D12_SHADER_RESOURCE_VIEW_DESC {
        match self {
            ShaderResourceViewDesc::Buffer {
                format,
                component_mapping,
                buffer,
            } => D3D12_SHADER_RESOURCE_VIEW_DESC {
                format: format.into(),
                view_dimension: D3D12_SRV_DIMENSION::D3D12_SRV_DIMENSION_BUFFER,
                shader_4_component_mapping: component_mapping,
                variant: D3D12_SHADER_RESOURCE_VIEW_DESC_VARIANT {
                    buffer: ManuallyDrop::new(unsafe { transmute(buffer) }),
                },
            },
            ShaderResourceViewDesc::Texture1D {
                format,
                component_mapping,
                texture_1d,
            } => D3D12_SHADER_RESOURCE_VIEW_DESC {
                format: format.into(),
                view_dimension: D3D12_SRV_DIMENSION::D3D12_SRV_DIMENSION_TEXTURE1D,
                shader_4_component_mapping: component_mapping,
                variant: D3D12_SHADER_RESOURCE_VIEW_DESC_VARIANT {
                    texture_1d: ManuallyDrop::new(texture_1d),
                },
            },
            ShaderResourceViewDesc::Texture1DArray {
                format,
                component_mapping,
                texture_1d_array,
            } => D3D12_SHADER_RESOURCE_VIEW_DESC {
                format: format.into(),
                view_dimension: D3D12_SRV_DIMENSION::D3D12_SRV_DIMENSION_TEXTURE1DARRAY,
                shader_4_component_mapping: component_mapping,
                variant: D3D12_SHADER_RESOURCE_VIEW_DESC_VARIANT {
                    texture_1d_array: ManuallyDrop::new(texture_1d_array),
                },
            },
            ShaderResourceViewDesc::Texture2D {
                format,
                component_mapping,
                texture_2d,
            } => D3D12_SHADER_RESOURCE_VIEW_DESC {
                format: format.into(),
                view_dimension: D3D12_SRV_DIMENSION::D3D12_SRV_DIMENSION_TEXTURE2D,
                shader_4_component_mapping: component_mapping,
                variant: D3D12_SHADER_RESOURCE_VIEW_DESC_VARIANT {
                    texture_2d: ManuallyDrop::new(texture_2d),
                },
            },
            ShaderResourceViewDesc::Texture2DArray {
                format,
                component_mapping,
                texture_2d_array,
            } => D3D12_SHADER_RESOURCE_VIEW_DESC {
                format: format.into(),
                view_dimension: D3D12_SRV_DIMENSION::D3D12_SRV_DIMENSION_TEXTURE2DARRAY,
                shader_4_component_mapping: component_mapping,
                variant: D3D12_SHADER_RESOURCE_VIEW_DESC_VARIANT {
                    texture_2d_array: ManuallyDrop::new(texture_2d_array),
                },
            },
            ShaderResourceViewDesc::Texture2DMS {
                format,
                component_mapping,
                texture_2dms,
            } => D3D12_SHADER_RESOURCE_VIEW_DESC {
                format: format.into(),
                view_dimension: D3D12_SRV_DIMENSION::D3D12_SRV_DIMENSION_TEXTURE2DMS,
                shader_4_component_mapping: component_mapping,
                variant: D3D12_SHADER_RESOURCE_VIEW_DESC_VARIANT {
                    texture_2dms: ManuallyDrop::new(texture_2dms),
                },
            },
            ShaderResourceViewDesc::Texture2DMSArray {
                format,
                component_mapping,
                texture_2dms_array,
            } => D3D12_SHADER_RESOURCE_VIEW_DESC {
                format: format.into(),
                view_dimension: D3D12_SRV_DIMENSION::D3D12_SRV_DIMENSION_TEXTURE2DMSARRAY,
                shader_4_component_mapping: component_mapping,
                variant: D3D12_SHADER_RESOURCE_VIEW_DESC_VARIANT {
                    texture_2dms_array: ManuallyDrop::new(texture_2dms_array),
                },
            },
            ShaderResourceViewDesc::Texture3D {
                format,
                component_mapping,
                texture_3d,
            } => D3D12_SHADER_RESOURCE_VIEW_DESC {
                format: format.into(),
                view_dimension: D3D12_SRV_DIMENSION::D3D12_SRV_DIMENSION_TEXTURE3D,
                shader_4_component_mapping: component_mapping,
                variant: D3D12_SHADER_RESOURCE_VIEW_DESC_VARIANT {
                    texture_3d: ManuallyDrop::new(texture_3d),
                },
            },
            ShaderResourceViewDesc::TextureCube {
                format,
                component_mapping,
                texture_cube,
            } => D3D12_SHADER_RESOURCE_VIEW_DESC {
                format: format.into(),
                view_dimension: D3D12_SRV_DIMENSION::D3D12_SRV_DIMENSION_TEXTURECUBE,
                shader_4_component_mapping: component_mapping,
                variant: D3D12_SHADER_RESOURCE_VIEW_DESC_VARIANT {
                    texture_cube: ManuallyDrop::new(texture_cube),
                },
            },
            ShaderResourceViewDesc::TextureCubeArray {
                format,
                component_mapping,
                texture_cube_array,
            } => D3D12_SHADER_RESOURCE_VIEW_DESC {
                format: format.into(),
                view_dimension: D3D12_SRV_DIMENSION::D3D12_SRV_DIMENSION_TEXTURECUBEARRAY,
                shader_4_component_mapping: component_mapping,
                variant: D3D12_SHADER_RESOURCE_VIEW_DESC_VARIANT {
                    texture_cube_array: ManuallyDrop::new(texture_cube_array),
                },
            },
            ShaderResourceViewDesc::RaytracingAccelerationStructure {
                format,
                component_mapping,
                raytracing_acceleration_structure,
            } => D3D12_SHADER_RESOURCE_VIEW_DESC {
                format: format.into(),
                view_dimension:
                    D3D12_SRV_DIMENSION::D3D12_SRV_DIMENSION_RAYTRACING_ACCELERATION_STRUCTURE,
                shader_4_component_mapping: component_mapping,
                variant: D3D12_SHADER_RESOURCE_VIEW_DESC_VARIANT {
                    raytracing_acceleration_structure: ManuallyDrop::new(
                        raytracing_acceleration_structure,
                    ),
                },
            },
        }
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct BufferSrv {
    pub first_element: u64,
    pub num_elements: u32,
    pub structure_byte_stride: u32,
    pub flags: BufferSrvFlags,
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug, Hash)]
pub struct BufferSrvFlags(pub i32);

impl BufferSrvFlags {
    pub const NONE: Self = Self(0i32);
    pub const RAW: Self = Self(1i32);
}

impl Default for BufferSrvFlags {
    fn default() -> Self {
        Self::NONE
    }
}

crate::flags_bitwise_impl!(BufferSrvFlags);

pub type Tex1DSrv = D3D12_TEX1D_SRV;
pub type Tex1DArraySrv = D3D12_TEX1D_ARRAY_SRV;
pub type Tex2DSrv = D3D12_TEX2D_SRV;
pub type Tex2DArraySrv = D3D12_TEX2D_ARRAY_SRV;
pub type Tex2DMSSrv = D3D12_TEX2DMS_SRV;
pub type Tex2DMSArraySrv = D3D12_TEX2DMS_ARRAY_SRV;
pub type Tex3DSrv = D3D12_TEX3D_SRV;
pub type TexCubeSrv = D3D12_TEXCUBE_SRV;
pub type TexCubeArraySrv = D3D12_TEXCUBE_ARRAY_SRV;
pub type RaytracingAccelerationStructureSrv = D3D12_RAYTRACING_ACCELERATION_STRUCTURE_SRV;

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D12_SHADER_RESOURCE_VIEW_DESC {
    pub format: DXGI_FORMAT,
    pub view_dimension: D3D12_SRV_DIMENSION,
    pub shader_4_component_mapping: u32,
    pub variant: D3D12_SHADER_RESOURCE_VIEW_DESC_VARIANT,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub union D3D12_SHADER_RESOURCE_VIEW_DESC_VARIANT {
    pub buffer: ManuallyDrop<D3D12_BUFFER_SRV>,
    pub texture_1d: ManuallyDrop<D3D12_TEX1D_SRV>,
    pub texture_1d_array: ManuallyDrop<D3D12_TEX1D_ARRAY_SRV>,
    pub texture_2d: ManuallyDrop<D3D12_TEX2D_SRV>,
    pub texture_2d_array: ManuallyDrop<D3D12_TEX2D_ARRAY_SRV>,
    pub texture_2dms: ManuallyDrop<D3D12_TEX2DMS_SRV>,
    pub texture_2dms_array: ManuallyDrop<D3D12_TEX2DMS_ARRAY_SRV>,
    pub texture_3d: ManuallyDrop<D3D12_TEX3D_SRV>,
    pub texture_cube: ManuallyDrop<D3D12_TEXCUBE_SRV>,
    pub texture_cube_array: ManuallyDrop<D3D12_TEXCUBE_ARRAY_SRV>,
    pub raytracing_acceleration_structure:
        ManuallyDrop<D3D12_RAYTRACING_ACCELERATION_STRUCTURE_SRV>,
}
