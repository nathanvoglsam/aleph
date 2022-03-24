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

use dx12::dxgi;
use interfaces::gpu::{
    ClearValue, ResourceStates, TextureCreateError, TextureDesc, TextureDimension, TextureFormat,
};

/// Internal function for converting texture format to DXGI_FORMAT
#[inline]
pub fn texture_format_to_dxgi(format: TextureFormat) -> dxgi::Format {
    match format {
        TextureFormat::R8Unorm => dxgi::Format::R8Unorm,
        TextureFormat::R8Snorm => dxgi::Format::R8Snorm,
        TextureFormat::R8Uint => dxgi::Format::R8Uint,
        TextureFormat::R8Sint => dxgi::Format::R8Sint,
        TextureFormat::R16Uint => dxgi::Format::R16Uint,
        TextureFormat::R16Sint => dxgi::Format::R16Sint,
        TextureFormat::R16Unorm => dxgi::Format::R16Unorm,
        TextureFormat::R16Snorm => dxgi::Format::R16Snorm,
        TextureFormat::R16Float => dxgi::Format::R16Float,
        TextureFormat::Rg8Unorm => dxgi::Format::R8G8Unorm,
        TextureFormat::Rg8Snorm => dxgi::Format::R8G8Snorm,
        TextureFormat::Rg8Uint => dxgi::Format::R8G8Uint,
        TextureFormat::Rg8Sint => dxgi::Format::R8G8Sint,
        TextureFormat::R32Uint => dxgi::Format::R32Uint,
        TextureFormat::R32Sint => dxgi::Format::R32Sint,
        TextureFormat::R32Float => dxgi::Format::R32Float,
        TextureFormat::Rg16Uint => dxgi::Format::R16G16Uint,
        TextureFormat::Rg16Sint => dxgi::Format::R16G16Sint,
        TextureFormat::Rg16Unorm => dxgi::Format::R16G16Unorm,
        TextureFormat::Rg16Snorm => dxgi::Format::R16G16Snorm,
        TextureFormat::Rg16Float => dxgi::Format::R16G16Float,
        TextureFormat::Rgba8Unorm => dxgi::Format::R8G8B8A8Unorm,
        TextureFormat::Rgba8UnormSrgb => dxgi::Format::R8G8B8A8UnormSRGB,
        TextureFormat::Rgba8Snorm => dxgi::Format::R8G8B8A8Snorm,
        TextureFormat::Rgba8Uint => dxgi::Format::R8G8B8A8Uint,
        TextureFormat::Rgba8Sint => dxgi::Format::R8G8B8A8Sint,
        TextureFormat::Bgra8Unorm => dxgi::Format::B8G8R8A8Unorm,
        TextureFormat::Bgra8UnormSrgb => dxgi::Format::B8G8R8A8UnormSRGB,
        TextureFormat::Rgb10a2Unorm => dxgi::Format::R10G10B10A2Unorm,
        TextureFormat::Rg11b10Float => dxgi::Format::R11G11B10Float,
        TextureFormat::Rg32Uint => dxgi::Format::R32G32B32Uint,
        TextureFormat::Rg32Sint => dxgi::Format::R32G32B32Sint,
        TextureFormat::Rg32Float => dxgi::Format::R32G32B32Float,
        TextureFormat::Rgba16Uint => dxgi::Format::R16G16B16A16Uint,
        TextureFormat::Rgba16Sint => dxgi::Format::R16G16B16A16Sint,
        TextureFormat::Rgba16Unorm => dxgi::Format::R16G16B16A16Unorm,
        TextureFormat::Rgba16Snorm => dxgi::Format::R16G16B16A16Snorm,
        TextureFormat::Rgba16Float => dxgi::Format::R16G16B16A16Float,
        TextureFormat::Rgba32Uint => dxgi::Format::R32G32B32A32Uint,
        TextureFormat::Rgba32Sint => dxgi::Format::R32G32B32A32Sint,
        TextureFormat::Rgba32Float => dxgi::Format::R32G32B32A32Float,
        TextureFormat::Depth32Float => dxgi::Format::D32Float,
        TextureFormat::Depth24Stencil8 => dxgi::Format::D24UnormS8Uint,
    }
}

pub fn resource_state_to_dx12(state: ResourceStates) -> dx12::ResourceStates {
    if state == ResourceStates::COMMON {
        return dx12::ResourceStates::COMMON;
    }

    let mut out = dx12::ResourceStates::default();
    if state.contains(ResourceStates::CONSTANT_BUFFER) {
        out |= dx12::ResourceStates::VERTEX_AND_CONSTANT_BUFFER
    }
    if state.contains(ResourceStates::VERTEX_BUFFER) {
        out |= dx12::ResourceStates::VERTEX_AND_CONSTANT_BUFFER
    }
    if state.contains(ResourceStates::INDEX_BUFFER) {
        out |= dx12::ResourceStates::INDEX_BUFFER
    }
    if state.contains(ResourceStates::INDIRECT_ARGUMENT) {
        out |= dx12::ResourceStates::INDIRECT_ARGUMENT
    }
    if state.contains(ResourceStates::SHADER_RESOURCE) {
        out |= dx12::ResourceStates::PIXEL_SHADER_RESOURCE
            | dx12::ResourceStates::NON_PIXEL_SHADER_RESOURCE
    }
    if state.contains(ResourceStates::UNORDERED_ACCESS) {
        out |= dx12::ResourceStates::UNORDERED_ACCESS
    }
    if state.contains(ResourceStates::RENDER_TARGET) {
        out |= dx12::ResourceStates::RENDER_TARGET
    }
    if state.contains(ResourceStates::DEPTH_WRITE) {
        out |= dx12::ResourceStates::DEPTH_WRITE
    }
    if state.contains(ResourceStates::DEPTH_READ) {
        out |= dx12::ResourceStates::DEPTH_READ
    }
    if state.contains(ResourceStates::STREAM_OUT) {
        out |= dx12::ResourceStates::STREAM_OUT
    }
    if state.contains(ResourceStates::COPY_DEST) {
        out |= dx12::ResourceStates::COPY_DEST
    }
    if state.contains(ResourceStates::COPY_SOURCE) {
        out |= dx12::ResourceStates::COPY_SOURCE
    }
    if state.contains(ResourceStates::RESOLVE_DEST) {
        out |= dx12::ResourceStates::RESOLVE_DEST
    }
    if state.contains(ResourceStates::RESOLVE_SOURCE) {
        out |= dx12::ResourceStates::RESOLVE_SOURCE
    }
    if state.contains(ResourceStates::PRESENT) {
        out |= dx12::ResourceStates::PRESENT
    }
    if state.contains(ResourceStates::ACCEL_STRUCT_READ) {
        out |= dx12::ResourceStates::RAYTRACING_ACCELERATION_STRUCTURE
    }
    if state.contains(ResourceStates::ACCEL_STRUCT_WRITE) {
        out |= dx12::ResourceStates::RAYTRACING_ACCELERATION_STRUCTURE
    }
    if state.contains(ResourceStates::ACCEL_STRUCT_BUILD_INPUT) {
        out |= dx12::ResourceStates::NON_PIXEL_SHADER_RESOURCE
    }
    if state.contains(ResourceStates::ACCEL_STRUCT_BUILD_BLAS) {
        out |= dx12::ResourceStates::RAYTRACING_ACCELERATION_STRUCTURE
    }
    if state.contains(ResourceStates::SHADING_RATE_SURFACE) {
        out |= dx12::ResourceStates::SHADING_RATE_SOURCE
    }

    out
}

pub fn texture_create_desc_to_dx12(
    desc: &TextureDesc,
) -> Result<dx12::ResourceDesc, TextureCreateError> {
    let (dimension, depth_or_array_size) = match desc.dimension {
        TextureDimension::Texture1D => {
            if desc.array_size >= u16::MAX as _ {
                return Err(TextureCreateError::InvalidArraySize(desc.array_size));
            }
            (dx12::ResourceDimension::Texture1D, desc.array_size)
        }
        TextureDimension::Texture2D => {
            if desc.array_size >= u16::MAX as _ {
                return Err(TextureCreateError::InvalidArraySize(desc.array_size));
            }
            (dx12::ResourceDimension::Texture2D, desc.array_size)
        }
        TextureDimension::Texture3D => {
            if desc.depth >= u16::MAX as _ {
                return Err(TextureCreateError::InvalidDepth(desc.depth));
            }
            if desc.array_size >= 1 {
                return Err(TextureCreateError::InvalidArraySize(desc.array_size));
            }
            (dx12::ResourceDimension::Texture3D, desc.depth)
        }
    };

    if desc.mip_levels >= u16::MAX as _ {
        return Err(TextureCreateError::InvalidMipLevelCount(desc.mip_levels));
    }

    if !desc.sample_count.is_power_of_two() || desc.sample_count > 16 {
        return Err(TextureCreateError::InvalidSampleCount(desc.sample_count));
    }

    let mut flags = dx12::ResourceFlags::NONE;
    if desc.is_render_target {
        if desc.format.is_depth_stencil() {
            flags |= dx12::ResourceFlags::ALLOW_DEPTH_STENCIL;
        } else {
            flags |= dx12::ResourceFlags::ALLOW_RENDER_TARGET;
        }
    }

    if desc.allow_unordered_access {
        flags |= dx12::ResourceFlags::ALLOW_UNORDERED_ACCESS;
    }

    let out = dx12::ResourceDesc {
        dimension,
        alignment: 0,
        width: desc.width as u64,
        height: desc.height,
        depth_or_array_size: depth_or_array_size as u16,
        mip_levels: desc.mip_levels as u16,
        format: texture_format_to_dxgi(desc.format),
        sample_desc: dxgi::SampleDesc {
            count: desc.sample_count,
            quality: desc.sample_quality,
        },
        layout: dx12::TextureLayout::Unknown,
        flags,
    };
    Ok(out)
}

pub fn texture_create_clear_value_to_dx12(
    desc: &TextureDesc,
    format: dxgi::Format,
) -> Result<Option<dx12::ClearValue>, TextureCreateError> {
    let clear = if let Some(clear) = &desc.clear_value {
        let clear = clear.clone();
        match clear.clone() {
            ClearValue::ColorF32(color) => {
                if !desc.format.is_depth_stencil() {
                    Some(dx12::ClearValue::Color {
                        format,
                        color: [color.r, color.g, color.b, color.a],
                    })
                } else {
                    return Err(TextureCreateError::InvalidClearValue(clear));
                }
            }
            ClearValue::ColorInt(v) => {
                if !desc.format.is_depth_stencil() {
                    Some(dx12::ClearValue::Color {
                        format,
                        color: decode_u32_color_to_float(v),
                    })
                } else {
                    return Err(TextureCreateError::InvalidClearValue(clear));
                }
            }
            ClearValue::DepthStencil(depth, stencil) => {
                if desc.format.is_depth_stencil() {
                    Some(dx12::ClearValue::Depth {
                        format,
                        depth_stencil: dx12::DepthStencilValue { depth, stencil },
                    })
                } else {
                    return Err(TextureCreateError::InvalidClearValue(clear));
                }
            }
        }
    } else {
        None
    };

    Ok(clear)
}

#[allow(clippy::erasing_op, clippy::identity_op)]
pub fn decode_u32_color_to_float(v: u32) -> [f32; 4] {
    let a = ((v >> (8 * 0)) & 0xFF) as f32 / 255.0;
    let b = ((v >> (8 * 1)) & 0xFF) as f32 / 255.0;
    let g = ((v >> (8 * 2)) & 0xFF) as f32 / 255.0;
    let r = ((v >> (8 * 3)) & 0xFF) as f32 / 255.0;
    [r, g, b, a]
}
