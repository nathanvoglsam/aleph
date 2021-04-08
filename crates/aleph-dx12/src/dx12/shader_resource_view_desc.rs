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

use crate::{dxgi, ComponentMapping};
use std::mem::transmute;
use windows_raw::Win32::Direct3D12::{
    D3D12_SHADER_RESOURCE_VIEW_DESC, D3D12_SHADER_RESOURCE_VIEW_DESC_0, D3D12_SRV_DIMENSION,
};

#[derive(Clone, Debug)]
pub enum ShaderResourceViewDesc {
    Buffer {
        format: dxgi::Format,
        component_mapping: ComponentMapping,
        buffer: BufferSrv,
    },
    Texture1D {
        format: dxgi::Format,
        component_mapping: ComponentMapping,
        texture_1d: Tex1DSrv,
    },
    Texture1DArray {
        format: dxgi::Format,
        component_mapping: ComponentMapping,
        texture_1d_array: Tex1DArraySrv,
    },
    Texture2D {
        format: dxgi::Format,
        component_mapping: ComponentMapping,
        texture_2d: Tex2DSrv,
    },
    Texture2DArray {
        format: dxgi::Format,
        component_mapping: ComponentMapping,
        texture_2d_array: Tex2DArraySrv,
    },
    Texture2DMS {
        format: dxgi::Format,
        component_mapping: ComponentMapping,
        texture_2dms: Tex2DMSSrv,
    },
    Texture2DMSArray {
        format: dxgi::Format,
        component_mapping: ComponentMapping,
        texture_2dms_array: Tex2DMSArraySrv,
    },
    Texture3D {
        format: dxgi::Format,
        component_mapping: ComponentMapping,
        texture_3d: Tex3DSrv,
    },
    TextureCube {
        format: dxgi::Format,
        component_mapping: ComponentMapping,
        texture_cube: TexCubeSrv,
    },
    TextureCubeArray {
        format: dxgi::Format,
        component_mapping: ComponentMapping,
        texture_cube_array: TexCubeArraySrv,
    },
    RaytracingAccelerationStructure {
        format: dxgi::Format,
        component_mapping: ComponentMapping,
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
                Format: format.into(),
                ViewDimension: D3D12_SRV_DIMENSION::D3D12_SRV_DIMENSION_BUFFER,
                Shader4ComponentMapping: component_mapping.into(),
                Anonymous: D3D12_SHADER_RESOURCE_VIEW_DESC_0 {
                    Buffer: unsafe { transmute(buffer) },
                },
            },
            ShaderResourceViewDesc::Texture1D {
                format,
                component_mapping,
                texture_1d,
            } => D3D12_SHADER_RESOURCE_VIEW_DESC {
                Format: format.into(),
                ViewDimension: D3D12_SRV_DIMENSION::D3D12_SRV_DIMENSION_TEXTURE1D,
                Shader4ComponentMapping: component_mapping.into(),
                Anonymous: D3D12_SHADER_RESOURCE_VIEW_DESC_0 {
                    Texture1D: unsafe { transmute(texture_1d) },
                },
            },
            ShaderResourceViewDesc::Texture1DArray {
                format,
                component_mapping,
                texture_1d_array,
            } => D3D12_SHADER_RESOURCE_VIEW_DESC {
                Format: format.into(),
                ViewDimension: D3D12_SRV_DIMENSION::D3D12_SRV_DIMENSION_TEXTURE1DARRAY,
                Shader4ComponentMapping: component_mapping.into(),
                Anonymous: D3D12_SHADER_RESOURCE_VIEW_DESC_0 {
                    Texture1DArray: unsafe { transmute(texture_1d_array) },
                },
            },
            ShaderResourceViewDesc::Texture2D {
                format,
                component_mapping,
                texture_2d,
            } => D3D12_SHADER_RESOURCE_VIEW_DESC {
                Format: format.into(),
                ViewDimension: D3D12_SRV_DIMENSION::D3D12_SRV_DIMENSION_TEXTURE2D,
                Shader4ComponentMapping: component_mapping.into(),
                Anonymous: D3D12_SHADER_RESOURCE_VIEW_DESC_0 {
                    Texture2D: unsafe { transmute(texture_2d) },
                },
            },
            ShaderResourceViewDesc::Texture2DArray {
                format,
                component_mapping,
                texture_2d_array,
            } => D3D12_SHADER_RESOURCE_VIEW_DESC {
                Format: format.into(),
                ViewDimension: D3D12_SRV_DIMENSION::D3D12_SRV_DIMENSION_TEXTURE2DARRAY,
                Shader4ComponentMapping: component_mapping.into(),
                Anonymous: D3D12_SHADER_RESOURCE_VIEW_DESC_0 {
                    Texture2DArray: unsafe { transmute(texture_2d_array) },
                },
            },
            ShaderResourceViewDesc::Texture2DMS {
                format,
                component_mapping,
                texture_2dms,
            } => D3D12_SHADER_RESOURCE_VIEW_DESC {
                Format: format.into(),
                ViewDimension: D3D12_SRV_DIMENSION::D3D12_SRV_DIMENSION_TEXTURE2DMS,
                Shader4ComponentMapping: component_mapping.into(),
                Anonymous: D3D12_SHADER_RESOURCE_VIEW_DESC_0 {
                    Texture2DMS: unsafe { transmute(texture_2dms) },
                },
            },
            ShaderResourceViewDesc::Texture2DMSArray {
                format,
                component_mapping,
                texture_2dms_array,
            } => D3D12_SHADER_RESOURCE_VIEW_DESC {
                Format: format.into(),
                ViewDimension: D3D12_SRV_DIMENSION::D3D12_SRV_DIMENSION_TEXTURE2DMSARRAY,
                Shader4ComponentMapping: component_mapping.into(),
                Anonymous: D3D12_SHADER_RESOURCE_VIEW_DESC_0 {
                    Texture2DMSArray: unsafe { transmute(texture_2dms_array) },
                },
            },
            ShaderResourceViewDesc::Texture3D {
                format,
                component_mapping,
                texture_3d,
            } => D3D12_SHADER_RESOURCE_VIEW_DESC {
                Format: format.into(),
                ViewDimension: D3D12_SRV_DIMENSION::D3D12_SRV_DIMENSION_TEXTURE3D,
                Shader4ComponentMapping: component_mapping.into(),
                Anonymous: D3D12_SHADER_RESOURCE_VIEW_DESC_0 {
                    Texture3D: unsafe { transmute(texture_3d) },
                },
            },
            ShaderResourceViewDesc::TextureCube {
                format,
                component_mapping,
                texture_cube,
            } => D3D12_SHADER_RESOURCE_VIEW_DESC {
                Format: format.into(),
                ViewDimension: D3D12_SRV_DIMENSION::D3D12_SRV_DIMENSION_TEXTURECUBE,
                Shader4ComponentMapping: component_mapping.into(),
                Anonymous: D3D12_SHADER_RESOURCE_VIEW_DESC_0 {
                    TextureCube: unsafe { transmute(texture_cube) },
                },
            },
            ShaderResourceViewDesc::TextureCubeArray {
                format,
                component_mapping,
                texture_cube_array,
            } => D3D12_SHADER_RESOURCE_VIEW_DESC {
                Format: format.into(),
                ViewDimension: D3D12_SRV_DIMENSION::D3D12_SRV_DIMENSION_TEXTURECUBEARRAY,
                Shader4ComponentMapping: component_mapping.into(),
                Anonymous: D3D12_SHADER_RESOURCE_VIEW_DESC_0 {
                    TextureCubeArray: unsafe { transmute(texture_cube_array) },
                },
            },
            ShaderResourceViewDesc::RaytracingAccelerationStructure {
                format,
                component_mapping,
                raytracing_acceleration_structure,
            } => D3D12_SHADER_RESOURCE_VIEW_DESC {
                Format: format.into(),
                ViewDimension:
                    D3D12_SRV_DIMENSION::D3D12_SRV_DIMENSION_RAYTRACING_ACCELERATION_STRUCTURE,
                Shader4ComponentMapping: component_mapping.into(),
                Anonymous: D3D12_SHADER_RESOURCE_VIEW_DESC_0 {
                    RaytracingAccelerationStructure: unsafe {
                        transmute(raytracing_acceleration_structure)
                    },
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
    #[inline]
    fn default() -> Self {
        Self::NONE
    }
}

windows_raw::flags_bitwise_impl!(BufferSrvFlags);

#[repr(C)]
#[derive(Clone, Default, PartialEq, Debug)]
pub struct Tex1DSrv {
    pub most_detailed_mip: u32,
    pub mip_levels: u32,
    pub resource_min_lod_clamp: f32,
}

#[repr(C)]
#[derive(Clone, Default, PartialEq, Debug)]
pub struct Tex1DArraySrv {
    pub most_detailed_mip: u32,
    pub mip_levels: u32,
    pub first_array_slice: u32,
    pub array_size: u32,
    pub resource_min_lod_clamp: f32,
}

#[repr(C)]
#[derive(Clone, Default, PartialEq, Debug)]
pub struct Tex2DSrv {
    pub most_detailed_mip: u32,
    pub mip_levels: u32,
    pub plane_slice: u32,
    pub resource_min_lod_clamp: f32,
}

#[repr(C)]
#[derive(Clone, Default, PartialEq, Debug)]
pub struct Tex2DArraySrv {
    pub most_detailed_mip: u32,
    pub mip_levels: u32,
    pub first_array_slice: u32,
    pub array_size: u32,
    pub plane_slice: u32,
    pub resource_min_lod_clamp: f32,
}

#[repr(C)]
#[derive(Clone, Default, Eq, PartialEq, Debug)]
pub struct Tex2DMSSrv {
    pub _unused: u32,
}

#[repr(C)]
#[derive(Clone, Default, Eq, PartialEq, Debug)]
pub struct Tex2DMSArraySrv {
    pub first_array_slice: u32,
    pub array_size: u32,
}

#[repr(C)]
#[derive(Clone, Default, PartialEq, Debug)]
pub struct Tex3DSrv {
    pub most_detailed_mip: u32,
    pub mip_levels: u32,
    pub resource_min_lod_clamp: f32,
}

#[repr(C)]
#[derive(Clone, Default, PartialEq, Debug)]
pub struct TexCubeSrv {
    pub most_detailed_mip: u32,
    pub mip_levels: u32,
    pub resource_min_lod_clamp: f32,
}

#[repr(C)]
#[derive(Clone, Default, PartialEq, Debug)]
pub struct TexCubeArraySrv {
    pub most_detailed_mip: u32,
    pub mip_levels: u32,
    pub first_2d_array_face: u32,
    pub num_cubes: u32,
    pub resource_min_lod_clamp: f32,
}

#[repr(C)]
#[derive(Clone, Default, Eq, PartialEq, Debug)]
pub struct RaytracingAccelerationStructureSrv {
    pub location: u64,
}
