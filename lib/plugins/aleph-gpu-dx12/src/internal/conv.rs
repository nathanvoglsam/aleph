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

use aleph_windows::Win32::Graphics::Direct3D12::{
    D3D12_BARRIER_ACCESS, D3D12_BARRIER_LAYOUT, D3D12_BARRIER_SYNC,
};
use dx12::dxgi;
use interfaces::gpu::{
    BarrierAccess, BarrierSync, BlendFactor, BlendOp, CompareOp, CullMode,
    DescriptorShaderVisibility, Format, FrontFaceOrder, ImageLayout, OptimalClearValue,
    PolygonMode, PrimitiveTopology, QueueType, ResourceStates, SamplerAddressMode,
    SamplerBorderColor, SamplerFilter, SamplerMipFilter, StencilOp, TextureCreateError,
    TextureDesc, TextureDimension,
};

/// Internal function for converting texture format to DXGI_FORMAT
pub const fn texture_format_to_dxgi(format: Format) -> dxgi::Format {
    match format {
        Format::R8Unorm => dxgi::Format::R8Unorm,
        Format::R8Snorm => dxgi::Format::R8Snorm,
        Format::R8Uint => dxgi::Format::R8Uint,
        Format::R8Sint => dxgi::Format::R8Sint,
        Format::R16Uint => dxgi::Format::R16Uint,
        Format::R16Sint => dxgi::Format::R16Sint,
        Format::R16Unorm => dxgi::Format::R16Unorm,
        Format::R16Snorm => dxgi::Format::R16Snorm,
        Format::R16Float => dxgi::Format::R16Float,
        Format::Rg8Unorm => dxgi::Format::R8G8Unorm,
        Format::Rg8Snorm => dxgi::Format::R8G8Snorm,
        Format::Rg8Uint => dxgi::Format::R8G8Uint,
        Format::Rg8Sint => dxgi::Format::R8G8Sint,
        Format::R32Uint => dxgi::Format::R32Uint,
        Format::R32Sint => dxgi::Format::R32Sint,
        Format::R32Float => dxgi::Format::R32Float,
        Format::Rg16Uint => dxgi::Format::R16G16Uint,
        Format::Rg16Sint => dxgi::Format::R16G16Sint,
        Format::Rg16Unorm => dxgi::Format::R16G16Unorm,
        Format::Rg16Snorm => dxgi::Format::R16G16Snorm,
        Format::Rg16Float => dxgi::Format::R16G16Float,
        Format::Rgba8Unorm => dxgi::Format::R8G8B8A8Unorm,
        Format::Rgba8UnormSrgb => dxgi::Format::R8G8B8A8UnormSRGB,
        Format::Rgba8Snorm => dxgi::Format::R8G8B8A8Snorm,
        Format::Rgba8Uint => dxgi::Format::R8G8B8A8Uint,
        Format::Rgba8Sint => dxgi::Format::R8G8B8A8Sint,
        Format::Bgra8Unorm => dxgi::Format::B8G8R8A8Unorm,
        Format::Bgra8UnormSrgb => dxgi::Format::B8G8R8A8UnormSRGB,
        Format::Rgb10a2Unorm => dxgi::Format::R10G10B10A2Unorm,
        Format::Rg11b10Float => dxgi::Format::R11G11B10Float,
        Format::Rg32Uint => dxgi::Format::R32G32B32Uint,
        Format::Rg32Sint => dxgi::Format::R32G32B32Sint,
        Format::Rg32Float => dxgi::Format::R32G32B32Float,
        Format::Rgba16Uint => dxgi::Format::R16G16B16A16Uint,
        Format::Rgba16Sint => dxgi::Format::R16G16B16A16Sint,
        Format::Rgba16Unorm => dxgi::Format::R16G16B16A16Unorm,
        Format::Rgba16Snorm => dxgi::Format::R16G16B16A16Snorm,
        Format::Rgba16Float => dxgi::Format::R16G16B16A16Float,
        Format::Rgba32Uint => dxgi::Format::R32G32B32A32Uint,
        Format::Rgba32Sint => dxgi::Format::R32G32B32A32Sint,
        Format::Rgba32Float => dxgi::Format::R32G32B32A32Float,
        Format::Depth32Float => dxgi::Format::D32Float,
        Format::Depth24Stencil8 => dxgi::Format::D24UnormS8Uint,
    }
}

pub const fn shader_visibility_to_dx12(
    visibility: DescriptorShaderVisibility,
) -> dx12::ShaderVisibility {
    // Visibility translates almost directly. 'Compute' maps to 'All' in d3d12 as 'Compute' is
    // the only stage active in a compute dispatch so d3d12 lacks a compute specifier.
    match visibility {
        DescriptorShaderVisibility::All => dx12::ShaderVisibility::All,
        DescriptorShaderVisibility::Compute => dx12::ShaderVisibility::All, // TODO: Verify
        DescriptorShaderVisibility::Vertex => dx12::ShaderVisibility::Vertex,
        DescriptorShaderVisibility::Hull => dx12::ShaderVisibility::Hull,
        DescriptorShaderVisibility::Domain => dx12::ShaderVisibility::Domain,
        DescriptorShaderVisibility::Geometry => dx12::ShaderVisibility::Geometry,
        DescriptorShaderVisibility::Fragment => dx12::ShaderVisibility::Pixel,
        DescriptorShaderVisibility::Amplification => dx12::ShaderVisibility::Amplification,
        DescriptorShaderVisibility::Mesh => dx12::ShaderVisibility::Mesh,
    }
}

pub const fn border_color_to_dx12(color: SamplerBorderColor) -> dx12::StaticBorderColor {
    match color {
        SamplerBorderColor::BlackTransparent => dx12::StaticBorderColor::TransparentBlack,
        SamplerBorderColor::BlackOpaque => dx12::StaticBorderColor::OpaqueBlack,
        SamplerBorderColor::WhiteOpaque => dx12::StaticBorderColor::OpaqueWhite,
    }
}

pub const fn polygon_mode_to_dx12(polygon_mode: PolygonMode) -> dx12::FillMode {
    match polygon_mode {
        PolygonMode::Fill => dx12::FillMode::Solid,
        PolygonMode::Line => dx12::FillMode::Wireframe,
    }
}

pub const fn cull_mode_to_dx12(cull_mode: CullMode) -> dx12::CullMode {
    match cull_mode {
        CullMode::None => dx12::CullMode::None,
        CullMode::Back => dx12::CullMode::Back,
        CullMode::Front => dx12::CullMode::Front,
    }
}

pub const fn front_face_order_to_dx12(front_face: FrontFaceOrder) -> dx12::Bool {
    match front_face {
        FrontFaceOrder::CounterClockwise => dx12::Bool::TRUE,
        FrontFaceOrder::Clockwise => dx12::Bool::FALSE,
    }
}

pub const fn blend_factor_to_dx12(factor: BlendFactor) -> dx12::Blend {
    match factor {
        BlendFactor::Zero => dx12::Blend::Zero,
        BlendFactor::One => dx12::Blend::One,
        BlendFactor::SrcColor => dx12::Blend::SrcColor,
        BlendFactor::OneMinusSrcColor => dx12::Blend::SrcColorInv,
        BlendFactor::DstColor => dx12::Blend::DestColor,
        BlendFactor::OneMinusDstColor => dx12::Blend::DestColorInv,
        BlendFactor::SrcAlpha => dx12::Blend::SrcAlpha,
        BlendFactor::OneMinusSrcAlpha => dx12::Blend::SrcAlphaInv,
        BlendFactor::DstAlpha => dx12::Blend::DestAlpha,
        BlendFactor::OneMinusDstAlpha => dx12::Blend::DestAlphaInv,
        BlendFactor::SrcAlphaSaturate => dx12::Blend::SrcAlphaSaturated,
        BlendFactor::BlendFactor => dx12::Blend::BlendFactor,
        BlendFactor::OneMinusBlendFactor => dx12::Blend::BlendFactorInv,
    }
}

pub const fn blend_op_to_dx12(op: BlendOp) -> dx12::BlendOp {
    match op {
        BlendOp::Add => dx12::BlendOp::Add,
        BlendOp::Subtract => dx12::BlendOp::Subtract,
        BlendOp::ReverseSubtract => dx12::BlendOp::SubtractReverse,
        BlendOp::Min => dx12::BlendOp::Min,
        BlendOp::Max => dx12::BlendOp::Max,
    }
}

pub const fn stencil_op_to_dx12(op: StencilOp) -> dx12::StencilOp {
    match op {
        StencilOp::Keep => dx12::StencilOp::Keep,
        StencilOp::Zero => dx12::StencilOp::Zero,
        StencilOp::Replace => dx12::StencilOp::Replace,
        StencilOp::IncrementClamp => dx12::StencilOp::IncrementSaturate,
        StencilOp::DecrementClamp => dx12::StencilOp::DecrementSaturate,
        StencilOp::Invert => dx12::StencilOp::Invert,
        StencilOp::IncrementWrap => dx12::StencilOp::Increment,
        StencilOp::DecrementWrap => dx12::StencilOp::Decrement,
    }
}

pub const fn compare_op_to_dx12(op: CompareOp) -> dx12::ComparisonFunc {
    match op {
        CompareOp::Never => dx12::ComparisonFunc::Never,
        CompareOp::Always => dx12::ComparisonFunc::Always,
        CompareOp::Equal => dx12::ComparisonFunc::Equal,
        CompareOp::NotEqual => dx12::ComparisonFunc::NotEqual,
        CompareOp::Less => dx12::ComparisonFunc::Less,
        CompareOp::LessEqual => dx12::ComparisonFunc::LessEqual,
        CompareOp::Greater => dx12::ComparisonFunc::Greater,
        CompareOp::GreaterOrEqual => dx12::ComparisonFunc::GreaterEqual,
    }
}

pub const fn primitive_topology_to_dx12(
    primitive_topology: PrimitiveTopology,
) -> (dx12::PrimitiveTopologyType, dx12::PrimitiveTopology) {
    match primitive_topology {
        PrimitiveTopology::PointList => (
            dx12::PrimitiveTopologyType::Point,
            dx12::PrimitiveTopology::PointList,
        ),
        PrimitiveTopology::LineList => (
            dx12::PrimitiveTopologyType::Line,
            dx12::PrimitiveTopology::LineList,
        ),
        PrimitiveTopology::LineStrip => (
            dx12::PrimitiveTopologyType::Line,
            dx12::PrimitiveTopology::LineStrip,
        ),
        PrimitiveTopology::TriangleList => (
            dx12::PrimitiveTopologyType::Triangle,
            dx12::PrimitiveTopology::TriangleList,
        ),
        PrimitiveTopology::TriangleStrip => (
            dx12::PrimitiveTopologyType::Triangle,
            dx12::PrimitiveTopology::TriangleStrip,
        ),
    }
}

pub const fn sampler_address_mode_to_dx12(mode: SamplerAddressMode) -> dx12::TextureAddressMode {
    match mode {
        SamplerAddressMode::Wrap => dx12::TextureAddressMode::Wrap,
        SamplerAddressMode::Mirror => dx12::TextureAddressMode::Mirror,
        SamplerAddressMode::Clamp => dx12::TextureAddressMode::Clamp,
        SamplerAddressMode::Border => dx12::TextureAddressMode::Border,
        SamplerAddressMode::MirrorOnce => dx12::TextureAddressMode::MirrorOnce,
    }
}

pub const fn queue_type_to_dx12(queue_type: QueueType) -> dx12::CommandListType {
    match queue_type {
        QueueType::General => dx12::CommandListType::Direct,
        QueueType::Compute => dx12::CommandListType::Compute,
        QueueType::Transfer => dx12::CommandListType::Copy,
    }
}

pub const fn sampler_filters_to_dx12(
    min: SamplerFilter,
    mag: SamplerFilter,
    mip: SamplerMipFilter,
    comparison: bool,
    anisotropic: bool,
) -> dx12::Filter {
    type F = SamplerFilter;
    type MF = SamplerMipFilter;
    type DF = dx12::Filter;
    match (anisotropic, comparison, min, mag, mip) {
        (false, false, F::Nearest, F::Nearest, MF::Nearest) => DF::MinMagMipPoint,
        (false, false, F::Nearest, F::Nearest, MF::Linear) => DF::MinMagPointMipLinear,
        (false, false, F::Nearest, F::Linear, MF::Nearest) => DF::MinPointMagLinearMipPoint,
        (false, false, F::Nearest, F::Linear, MF::Linear) => DF::MinPointMagMipLinear,
        (false, false, F::Linear, F::Nearest, MF::Nearest) => DF::MinLinearMagMipPoint,
        (false, false, F::Linear, F::Nearest, MF::Linear) => DF::MinLinearMagPointMipLinear,
        (false, false, F::Linear, F::Linear, MF::Nearest) => DF::MinMagLinearMipPoint,
        (false, false, F::Linear, F::Linear, MF::Linear) => DF::MinMagMipLinear,
        (false, true, F::Nearest, F::Nearest, MF::Nearest) => DF::ComparisonMinMagMipPoint,
        (false, true, F::Nearest, F::Nearest, MF::Linear) => DF::ComparisonMinMagPointMipLinear,
        (false, true, F::Nearest, F::Linear, MF::Nearest) => {
            DF::ComparisonMinPointMagLinearMipPoint
        }
        (false, true, F::Nearest, F::Linear, MF::Linear) => DF::ComparisonMinPointMagMipLinear,
        (false, true, F::Linear, F::Nearest, MF::Nearest) => DF::ComparisonMinLinearMagMipPoint,
        (false, true, F::Linear, F::Nearest, MF::Linear) => {
            DF::ComparisonMinLinearMagPointMipLinear
        }
        (false, true, F::Linear, F::Linear, MF::Nearest) => DF::ComparisonMinMagLinearMipPoint,
        (false, true, F::Linear, F::Linear, MF::Linear) => DF::ComparisonMinMagMipLinear,
        (true, false, _, _, _) => DF::Anisotropic,
        (true, true, _, _, _) => DF::ComparisonAnisotropic,
    }
}

pub fn image_layout_to_dx12(layout: ImageLayout) -> D3D12_BARRIER_LAYOUT {
    match layout {
        ImageLayout::Undefined => D3D12_BARRIER_LAYOUT::UNDEFINED,
        ImageLayout::Common => D3D12_BARRIER_LAYOUT::COMMON,
        ImageLayout::PresentSrc => D3D12_BARRIER_LAYOUT::PRESENT,
        ImageLayout::ColorAttachmentOptimal => D3D12_BARRIER_LAYOUT::RENDER_TARGET,
        ImageLayout::DepthStencilAttachmentOptimal => D3D12_BARRIER_LAYOUT::DEPTH_STENCIL_WRITE,
        ImageLayout::DepthStencilReadOnlyOptimal => D3D12_BARRIER_LAYOUT::DEPTH_STENCIL_READ,
        ImageLayout::ShaderReadOnlyOptimal => D3D12_BARRIER_LAYOUT::SHADER_RESOURCE,
        ImageLayout::CopySrc => D3D12_BARRIER_LAYOUT::COPY_SOURCE,
        ImageLayout::CopyDst => D3D12_BARRIER_LAYOUT::COPY_DEST,
        ImageLayout::UnorderedAccess => D3D12_BARRIER_LAYOUT::UNORDERED_ACCESS,
        ImageLayout::ResolveSource => D3D12_BARRIER_LAYOUT::RESOLVE_SOURCE,
        ImageLayout::ResolveDest => D3D12_BARRIER_LAYOUT::RESOLVE_DEST,
    }
}

macro_rules! translate_flag_onto {
    ($src:ident, $dst:ident, $src_flag:expr, $dst_flag:expr) => {
        if ($src.contains($src_flag)) {
            $dst = $dst | $dst_flag;
        }
    };
}

pub fn barrier_sync_to_dx12(sync: BarrierSync) -> D3D12_BARRIER_SYNC {
    let mut out = D3D12_BARRIER_SYNC::empty();
    translate_flag_onto!(sync, out, BarrierSync::ALL, D3D12_BARRIER_SYNC::ALL);
    translate_flag_onto!(sync, out, BarrierSync::DRAW, D3D12_BARRIER_SYNC::DRAW);
    translate_flag_onto!(
        sync,
        out,
        BarrierSync::INPUT_ASSEMBLER,
        D3D12_BARRIER_SYNC::INPUT_ASSEMBLER
    );
    translate_flag_onto!(
        sync,
        out,
        BarrierSync::VERTEX_SHADING,
        D3D12_BARRIER_SYNC::VERTEX_SHADING
    );
    translate_flag_onto!(
        sync,
        out,
        BarrierSync::PIXEL_SHADING,
        D3D12_BARRIER_SYNC::PIXEL_SHADING
    );
    translate_flag_onto!(
        sync,
        out,
        BarrierSync::DEPTH_STENCIL,
        D3D12_BARRIER_SYNC::DEPTH_STENCIL
    );
    translate_flag_onto!(
        sync,
        out,
        BarrierSync::RENDER_TARGET,
        D3D12_BARRIER_SYNC::RENDER_TARGET
    );
    translate_flag_onto!(
        sync,
        out,
        BarrierSync::COMPUTE_SHADING,
        D3D12_BARRIER_SYNC::COMPUTE_SHADING
    );
    translate_flag_onto!(
        sync,
        out,
        BarrierSync::RAYTRACING,
        D3D12_BARRIER_SYNC::RAYTRACING
    );
    translate_flag_onto!(sync, out, BarrierSync::COPY, D3D12_BARRIER_SYNC::COPY);
    translate_flag_onto!(sync, out, BarrierSync::RESOLVE, D3D12_BARRIER_SYNC::RESOLVE);
    translate_flag_onto!(
        sync,
        out,
        BarrierSync::EXECUTE_INDIRECT,
        D3D12_BARRIER_SYNC::EXECUTE_INDIRECT
    );
    translate_flag_onto!(
        sync,
        out,
        BarrierSync::CLEAR_UNORDERED_ACCESS_VIEW,
        D3D12_BARRIER_SYNC::CLEAR_UNORDERED_ACCESS_VIEW
    );
    translate_flag_onto!(
        sync,
        out,
        BarrierSync::BUILD_RAYTRACING_ACCELERATION_STRUCTURE,
        (D3D12_BARRIER_SYNC::BUILD_RAYTRACING_ACCELERATION_STRUCTURE
            | D3D12_BARRIER_SYNC::EMIT_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO)
    );
    translate_flag_onto!(
        sync,
        out,
        BarrierSync::COPY_RAYTRACING_ACCELERATION_STRUCTURE,
        D3D12_BARRIER_SYNC::COPY_RAYTRACING_ACCELERATION_STRUCTURE
    );

    out
}

pub fn barrier_access_to_dx12(access: BarrierAccess) -> D3D12_BARRIER_ACCESS {
    let mut out = D3D12_BARRIER_ACCESS::empty();
    translate_flag_onto!(
        access,
        out,
        BarrierAccess::VERTEX_BUFFER_READ,
        D3D12_BARRIER_ACCESS::VERTEX_BUFFER
    );
    translate_flag_onto!(
        access,
        out,
        BarrierAccess::INDEX_BUFFER_READ,
        D3D12_BARRIER_ACCESS::INDEX_BUFFER
    );
    translate_flag_onto!(
        access,
        out,
        BarrierAccess::CONSTANT_BUFFER_READ,
        D3D12_BARRIER_ACCESS::CONSTANT_BUFFER
    );
    translate_flag_onto!(
        access,
        out,
        BarrierAccess::INDIRECT_COMMAND_READ,
        D3D12_BARRIER_ACCESS::INDIRECT_ARGUMENT
    );
    translate_flag_onto!(
        access,
        out,
        BarrierAccess::SHADER_SAMPLED_READ,
        D3D12_BARRIER_ACCESS::SHADER_RESOURCE
    );
    translate_flag_onto!(
        access,
        out,
        BarrierAccess::RENDER_TARGET_READ,
        D3D12_BARRIER_ACCESS::RENDER_TARGET
    );
    translate_flag_onto!(
        access,
        out,
        BarrierAccess::RENDER_TARGET_WRITE,
        D3D12_BARRIER_ACCESS::RENDER_TARGET
    );
    translate_flag_onto!(
        access,
        out,
        BarrierAccess::DEPTH_STENCIL_READ,
        D3D12_BARRIER_ACCESS::DEPTH_STENCIL_READ
    );
    translate_flag_onto!(
        access,
        out,
        BarrierAccess::DEPTH_STENCIL_WRITE,
        D3D12_BARRIER_ACCESS::DEPTH_STENCIL_WRITE
    );
    translate_flag_onto!(
        access,
        out,
        BarrierAccess::COPY_READ,
        D3D12_BARRIER_ACCESS::COPY_SOURCE
    );
    translate_flag_onto!(
        access,
        out,
        BarrierAccess::COPY_WRITE,
        D3D12_BARRIER_ACCESS::COPY_DEST
    );
    translate_flag_onto!(
        access,
        out,
        BarrierAccess::RAYTRACING_ACCELERATION_STRUCTURE_READ,
        D3D12_BARRIER_ACCESS::RAYTRACING_ACCELERATION_STRUCTURE_READ
    );
    translate_flag_onto!(
        access,
        out,
        BarrierAccess::RAYTRACING_ACCELERATION_STRUCTURE_WRITE,
        D3D12_BARRIER_ACCESS::RAYTRACING_ACCELERATION_STRUCTURE_WRITE
    );

    out
}

pub fn resource_state_to_dx12(state: ResourceStates) -> dx12::ResourceStates {
    if state == ResourceStates::COMMON {
        return dx12::ResourceStates::COMMON;
    }

    let mut out = dx12::ResourceStates::default();
    if state.contains(ResourceStates::VERTEX_AND_CONSTANT_BUFFER) {
        out |= dx12::ResourceStates::VERTEX_AND_CONSTANT_BUFFER
    }
    if state.contains(ResourceStates::INDEX_BUFFER) {
        out |= dx12::ResourceStates::INDEX_BUFFER
    }
    if state.contains(ResourceStates::INDIRECT_ARGUMENT) {
        out |= dx12::ResourceStates::INDIRECT_ARGUMENT
    }
    if state.contains(ResourceStates::NON_PIXEL_SHADER_RESOURCE) {
        out |= dx12::ResourceStates::NON_PIXEL_SHADER_RESOURCE
    }
    if state.contains(ResourceStates::PIXEL_SHADER_RESOURCE) {
        out |= dx12::ResourceStates::PIXEL_SHADER_RESOURCE
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
    if state.contains(ResourceStates::PRESENT) {
        out |= dx12::ResourceStates::PRESENT
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
            OptimalClearValue::ColorF32 { r, g, b, a } => {
                if !desc.format.is_depth_stencil() {
                    Some(dx12::ClearValue::Color {
                        format,
                        color: [r, g, b, a],
                    })
                } else {
                    return Err(TextureCreateError::InvalidOptimalClearValue(clear));
                }
            }
            OptimalClearValue::ColorInt(v) => {
                if !desc.format.is_depth_stencil() {
                    Some(dx12::ClearValue::Color {
                        format,
                        color: decode_u32_color_to_float(v),
                    })
                } else {
                    return Err(TextureCreateError::InvalidOptimalClearValue(clear));
                }
            }
            OptimalClearValue::DepthStencil(depth, stencil) => {
                if desc.format.is_depth_stencil() {
                    Some(dx12::ClearValue::Depth {
                        format,
                        depth_stencil: dx12::DepthStencilValue { depth, stencil },
                    })
                } else {
                    return Err(TextureCreateError::InvalidOptimalClearValue(clear));
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
