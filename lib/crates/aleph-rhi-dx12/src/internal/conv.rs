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

use aleph_rhi_api::*;
use aleph_rhi_impl_utils::conv::decode_u32_color_to_float;
use windows::Win32::Foundation::BOOL;
use windows::Win32::Graphics::Direct3D::*;
use windows::Win32::Graphics::Direct3D12::*;
use windows::Win32::Graphics::Dxgi::Common::*;

/// Internal function for converting texture format to DXGI_FORMAT
pub const fn texture_format_to_dxgi(format: Format) -> DXGI_FORMAT {
    match format {
        Format::R8Unorm => DXGI_FORMAT_R8_UNORM,
        Format::R8Snorm => DXGI_FORMAT_R8_SNORM,
        Format::R8Uint => DXGI_FORMAT_R8_UINT,
        Format::R8Sint => DXGI_FORMAT_R8_SINT,
        Format::R16Uint => DXGI_FORMAT_R16_UINT,
        Format::R16Sint => DXGI_FORMAT_R16_SINT,
        Format::R16Unorm => DXGI_FORMAT_R16_UNORM,
        Format::R16Snorm => DXGI_FORMAT_R16_SNORM,
        Format::R16Float => DXGI_FORMAT_R16_FLOAT,
        Format::R32Uint => DXGI_FORMAT_R32_UINT,
        Format::R32Sint => DXGI_FORMAT_R32_SINT,
        Format::R32Float => DXGI_FORMAT_R32_FLOAT,
        Format::Rg8Unorm => DXGI_FORMAT_R8G8_UNORM,
        Format::Rg8Snorm => DXGI_FORMAT_R8G8_SNORM,
        Format::Rg8Uint => DXGI_FORMAT_R8G8_UINT,
        Format::Rg8Sint => DXGI_FORMAT_R8G8_SINT,
        Format::Rg16Uint => DXGI_FORMAT_R16G16_UINT,
        Format::Rg16Sint => DXGI_FORMAT_R16G16_SINT,
        Format::Rg16Unorm => DXGI_FORMAT_R16G16_UNORM,
        Format::Rg16Snorm => DXGI_FORMAT_R16G16_SNORM,
        Format::Rg16Float => DXGI_FORMAT_R16G16_FLOAT,
        Format::Rg32Uint => DXGI_FORMAT_R32G32_UINT,
        Format::Rg32Sint => DXGI_FORMAT_R32G32_SINT,
        Format::Rg32Float => DXGI_FORMAT_R32G32_FLOAT,
        Format::Rgb32Uint => DXGI_FORMAT_R32G32B32_UINT,
        Format::Rgb32Sint => DXGI_FORMAT_R32G32B32_SINT,
        Format::Rgb32Float => DXGI_FORMAT_R32G32B32_FLOAT,
        Format::Rgba8Unorm => DXGI_FORMAT_R8G8B8A8_UNORM,
        Format::Rgba8UnormSrgb => DXGI_FORMAT_R8G8B8A8_UNORM_SRGB,
        Format::Rgba8Snorm => DXGI_FORMAT_R8G8B8A8_SNORM,
        Format::Rgba8Uint => DXGI_FORMAT_R8G8B8A8_UINT,
        Format::Rgba8Sint => DXGI_FORMAT_R8G8B8A8_SINT,
        Format::Bgra8Unorm => DXGI_FORMAT_B8G8R8A8_UNORM,
        Format::Bgra8UnormSrgb => DXGI_FORMAT_B8G8R8A8_UNORM_SRGB,
        Format::Rgb10a2Unorm => DXGI_FORMAT_R10G10B10A2_UNORM,
        Format::Rg11b10Float => DXGI_FORMAT_R11G11B10_FLOAT,
        Format::Rgba16Uint => DXGI_FORMAT_R16G16B16A16_UINT,
        Format::Rgba16Sint => DXGI_FORMAT_R16G16B16A16_SINT,
        Format::Rgba16Unorm => DXGI_FORMAT_R16G16B16A16_UNORM,
        Format::Rgba16Snorm => DXGI_FORMAT_R16G16B16A16_SNORM,
        Format::Rgba16Float => DXGI_FORMAT_R16G16B16A16_FLOAT,
        Format::Rgba32Uint => DXGI_FORMAT_R32G32B32A32_UINT,
        Format::Rgba32Sint => DXGI_FORMAT_R32G32B32A32_SINT,
        Format::Rgba32Float => DXGI_FORMAT_R32G32B32A32_FLOAT,
        Format::Depth32Float => DXGI_FORMAT_D32_FLOAT,
        Format::Depth24Stencil8 => DXGI_FORMAT_D24_UNORM_S8_UINT,
    }
}

pub const fn shader_visibility_to_dx12(
    visibility: DescriptorShaderVisibility,
) -> D3D12_SHADER_VISIBILITY {
    // Visibility translates almost directly. 'Compute' maps to 'All' in d3d12 as 'Compute' is
    // the only stage active in a compute dispatch so d3d12 lacks a compute specifier.
    match visibility {
        DescriptorShaderVisibility::All => D3D12_SHADER_VISIBILITY_ALL,
        DescriptorShaderVisibility::Compute => D3D12_SHADER_VISIBILITY_ALL,
        DescriptorShaderVisibility::Vertex => D3D12_SHADER_VISIBILITY_VERTEX,
        DescriptorShaderVisibility::Hull => D3D12_SHADER_VISIBILITY_HULL,
        DescriptorShaderVisibility::Domain => D3D12_SHADER_VISIBILITY_DOMAIN,
        DescriptorShaderVisibility::Geometry => D3D12_SHADER_VISIBILITY_GEOMETRY,
        DescriptorShaderVisibility::Fragment => D3D12_SHADER_VISIBILITY_PIXEL,
        DescriptorShaderVisibility::Amplification => D3D12_SHADER_VISIBILITY_AMPLIFICATION,
        DescriptorShaderVisibility::Mesh => D3D12_SHADER_VISIBILITY_MESH,
    }
}

pub const fn border_color_to_dx12_static(color: SamplerBorderColor) -> D3D12_STATIC_BORDER_COLOR {
    match color {
        SamplerBorderColor::BlackTransparent => D3D12_STATIC_BORDER_COLOR_TRANSPARENT_BLACK,
        SamplerBorderColor::BlackOpaque => D3D12_STATIC_BORDER_COLOR_OPAQUE_BLACK,
        SamplerBorderColor::WhiteOpaque => D3D12_STATIC_BORDER_COLOR_OPAQUE_WHITE,
    }
}

pub const fn border_color_to_dx12(color: SamplerBorderColor) -> [f32; 4] {
    const BLACK_TRANSPARENT: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
    const BLACK_OPAQUE: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
    const WHITE_OPAQUE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

    match color {
        SamplerBorderColor::BlackTransparent => BLACK_TRANSPARENT,
        SamplerBorderColor::BlackOpaque => BLACK_OPAQUE,
        SamplerBorderColor::WhiteOpaque => WHITE_OPAQUE,
    }
}

pub const fn polygon_mode_to_dx12(polygon_mode: PolygonMode) -> D3D12_FILL_MODE {
    match polygon_mode {
        PolygonMode::Fill => D3D12_FILL_MODE_SOLID,
        PolygonMode::Line => D3D12_FILL_MODE_WIREFRAME,
    }
}

pub const fn cull_mode_to_dx12(cull_mode: CullMode) -> D3D12_CULL_MODE {
    match cull_mode {
        CullMode::None => D3D12_CULL_MODE_NONE,
        CullMode::Back => D3D12_CULL_MODE_BACK,
        CullMode::Front => D3D12_CULL_MODE_FRONT,
    }
}

pub const fn front_face_order_to_dx12(front_face: FrontFaceOrder) -> BOOL {
    match front_face {
        FrontFaceOrder::CounterClockwise => BOOL(1),
        FrontFaceOrder::Clockwise => BOOL(0),
    }
}

pub const fn blend_factor_to_dx12(factor: BlendFactor) -> D3D12_BLEND {
    match factor {
        BlendFactor::Zero => D3D12_BLEND_ZERO,
        BlendFactor::One => D3D12_BLEND_ONE,
        BlendFactor::SrcColor => D3D12_BLEND_SRC_COLOR,
        BlendFactor::OneMinusSrcColor => D3D12_BLEND_INV_SRC_COLOR,
        BlendFactor::DstColor => D3D12_BLEND_DEST_COLOR,
        BlendFactor::OneMinusDstColor => D3D12_BLEND_INV_DEST_COLOR,
        BlendFactor::SrcAlpha => D3D12_BLEND_SRC_ALPHA,
        BlendFactor::OneMinusSrcAlpha => D3D12_BLEND_INV_SRC_ALPHA,
        BlendFactor::DstAlpha => D3D12_BLEND_DEST_ALPHA,
        BlendFactor::OneMinusDstAlpha => D3D12_BLEND_INV_DEST_ALPHA,
        BlendFactor::SrcAlphaSaturate => D3D12_BLEND_SRC_ALPHA_SAT,
        BlendFactor::BlendFactor => D3D12_BLEND_BLEND_FACTOR,
        BlendFactor::OneMinusBlendFactor => D3D12_BLEND_INV_BLEND_FACTOR,
    }
}

pub const fn blend_op_to_dx12(op: BlendOp) -> D3D12_BLEND_OP {
    match op {
        BlendOp::Add => D3D12_BLEND_OP_ADD,
        BlendOp::Subtract => D3D12_BLEND_OP_SUBTRACT,
        BlendOp::ReverseSubtract => D3D12_BLEND_OP_REV_SUBTRACT,
        BlendOp::Min => D3D12_BLEND_OP_MIN,
        BlendOp::Max => D3D12_BLEND_OP_MAX,
    }
}

pub const fn stencil_op_to_dx12(op: StencilOp) -> D3D12_STENCIL_OP {
    match op {
        StencilOp::Keep => D3D12_STENCIL_OP_KEEP,
        StencilOp::Zero => D3D12_STENCIL_OP_ZERO,
        StencilOp::Replace => D3D12_STENCIL_OP_REPLACE,
        StencilOp::IncrementClamp => D3D12_STENCIL_OP_INCR_SAT,
        StencilOp::DecrementClamp => D3D12_STENCIL_OP_DECR_SAT,
        StencilOp::Invert => D3D12_STENCIL_OP_INVERT,
        StencilOp::IncrementWrap => D3D12_STENCIL_OP_INCR,
        StencilOp::DecrementWrap => D3D12_STENCIL_OP_DECR,
    }
}

pub const fn compare_op_to_dx12(op: CompareOp) -> D3D12_COMPARISON_FUNC {
    match op {
        CompareOp::Never => D3D12_COMPARISON_FUNC_NEVER,
        CompareOp::Always => D3D12_COMPARISON_FUNC_ALWAYS,
        CompareOp::Equal => D3D12_COMPARISON_FUNC_EQUAL,
        CompareOp::NotEqual => D3D12_COMPARISON_FUNC_NOT_EQUAL,
        CompareOp::Less => D3D12_COMPARISON_FUNC_LESS,
        CompareOp::LessEqual => D3D12_COMPARISON_FUNC_LESS_EQUAL,
        CompareOp::Greater => D3D12_COMPARISON_FUNC_GREATER,
        CompareOp::GreaterOrEqual => D3D12_COMPARISON_FUNC_GREATER_EQUAL,
    }
}

pub const fn primitive_topology_to_dx12(
    primitive_topology: PrimitiveTopology,
) -> (D3D12_PRIMITIVE_TOPOLOGY_TYPE, D3D_PRIMITIVE_TOPOLOGY) {
    match primitive_topology {
        PrimitiveTopology::PointList => (
            D3D12_PRIMITIVE_TOPOLOGY_TYPE_POINT,
            D3D_PRIMITIVE_TOPOLOGY_POINTLIST,
        ),
        PrimitiveTopology::LineList => (
            D3D12_PRIMITIVE_TOPOLOGY_TYPE_LINE,
            D3D_PRIMITIVE_TOPOLOGY_LINELIST,
        ),
        PrimitiveTopology::LineStrip => (
            D3D12_PRIMITIVE_TOPOLOGY_TYPE_LINE,
            D3D_PRIMITIVE_TOPOLOGY_LINESTRIP,
        ),
        PrimitiveTopology::TriangleList => (
            D3D12_PRIMITIVE_TOPOLOGY_TYPE_TRIANGLE,
            D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST,
        ),
        PrimitiveTopology::TriangleStrip => (
            D3D12_PRIMITIVE_TOPOLOGY_TYPE_TRIANGLE,
            D3D_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP,
        ),
    }
}

pub const fn sampler_address_mode_to_dx12(mode: SamplerAddressMode) -> D3D12_TEXTURE_ADDRESS_MODE {
    match mode {
        SamplerAddressMode::Wrap => D3D12_TEXTURE_ADDRESS_MODE_WRAP,
        SamplerAddressMode::Mirror => D3D12_TEXTURE_ADDRESS_MODE_MIRROR,
        SamplerAddressMode::Clamp => D3D12_TEXTURE_ADDRESS_MODE_CLAMP,
        SamplerAddressMode::Border => D3D12_TEXTURE_ADDRESS_MODE_BORDER,
        SamplerAddressMode::MirrorOnce => D3D12_TEXTURE_ADDRESS_MODE_MIRROR_ONCE,
    }
}

pub const fn queue_type_to_dx12(queue_type: QueueType) -> D3D12_COMMAND_LIST_TYPE {
    match queue_type {
        QueueType::General => D3D12_COMMAND_LIST_TYPE_DIRECT,
        QueueType::Compute => D3D12_COMMAND_LIST_TYPE_COMPUTE,
        QueueType::Transfer => D3D12_COMMAND_LIST_TYPE_COPY,
    }
}

pub const fn sampler_filters_to_dx12(
    min: SamplerFilter,
    mag: SamplerFilter,
    mip: SamplerMipFilter,
    comparison: bool,
    anisotropic: bool,
) -> D3D12_FILTER {
    type F = SamplerFilter;
    type MF = SamplerMipFilter;
    match (anisotropic, comparison, min, mag, mip) {
        (false, false, F::Nearest, F::Nearest, MF::Nearest) => D3D12_FILTER_MIN_MAG_MIP_POINT,
        (false, false, F::Nearest, F::Nearest, MF::Linear) => D3D12_FILTER_MIN_MAG_POINT_MIP_LINEAR,
        (false, false, F::Nearest, F::Linear, MF::Nearest) => {
            D3D12_FILTER_MIN_POINT_MAG_LINEAR_MIP_POINT
        }
        (false, false, F::Nearest, F::Linear, MF::Linear) => D3D12_FILTER_MIN_POINT_MAG_MIP_LINEAR,
        (false, false, F::Linear, F::Nearest, MF::Nearest) => D3D12_FILTER_MIN_LINEAR_MAG_MIP_POINT,
        (false, false, F::Linear, F::Nearest, MF::Linear) => {
            D3D12_FILTER_MIN_LINEAR_MAG_POINT_MIP_LINEAR
        }
        (false, false, F::Linear, F::Linear, MF::Nearest) => D3D12_FILTER_MIN_MAG_LINEAR_MIP_POINT,
        (false, false, F::Linear, F::Linear, MF::Linear) => D3D12_FILTER_MIN_MAG_MIP_LINEAR,
        (false, true, F::Nearest, F::Nearest, MF::Nearest) => {
            D3D12_FILTER_COMPARISON_MIN_MAG_MIP_POINT
        }
        (false, true, F::Nearest, F::Nearest, MF::Linear) => {
            D3D12_FILTER_COMPARISON_MIN_MAG_POINT_MIP_LINEAR
        }
        (false, true, F::Nearest, F::Linear, MF::Nearest) => {
            D3D12_FILTER_COMPARISON_MIN_POINT_MAG_LINEAR_MIP_POINT
        }
        (false, true, F::Nearest, F::Linear, MF::Linear) => {
            D3D12_FILTER_COMPARISON_MIN_POINT_MAG_MIP_LINEAR
        }
        (false, true, F::Linear, F::Nearest, MF::Nearest) => {
            D3D12_FILTER_COMPARISON_MIN_LINEAR_MAG_MIP_POINT
        }
        (false, true, F::Linear, F::Nearest, MF::Linear) => {
            D3D12_FILTER_COMPARISON_MIN_LINEAR_MAG_POINT_MIP_LINEAR
        }
        (false, true, F::Linear, F::Linear, MF::Nearest) => {
            D3D12_FILTER_COMPARISON_MIN_MAG_LINEAR_MIP_POINT
        }
        (false, true, F::Linear, F::Linear, MF::Linear) => {
            D3D12_FILTER_COMPARISON_MIN_MAG_MIP_LINEAR
        }
        (true, false, _, _, _) => D3D12_FILTER_ANISOTROPIC,
        (true, true, _, _, _) => D3D12_FILTER_COMPARISON_ANISOTROPIC,
    }
}

pub const fn image_layout_to_dx12(
    layout: ImageLayout,
    queue_type: Option<QueueType>,
) -> D3D12_BARRIER_LAYOUT {
    match queue_type {
        Some(QueueType::General) => match layout {
            ImageLayout::Undefined => D3D12_BARRIER_LAYOUT_UNDEFINED,
            ImageLayout::Common => D3D12_BARRIER_LAYOUT_DIRECT_QUEUE_GENERIC_READ,
            ImageLayout::PresentSrc => D3D12_BARRIER_LAYOUT_PRESENT,
            ImageLayout::ColorAttachment => D3D12_BARRIER_LAYOUT_RENDER_TARGET,
            ImageLayout::DepthStencilAttachment => D3D12_BARRIER_LAYOUT_DEPTH_STENCIL_WRITE,
            ImageLayout::DepthStencilReadOnly => D3D12_BARRIER_LAYOUT_DEPTH_STENCIL_READ,
            ImageLayout::ShaderReadOnly => D3D12_BARRIER_LAYOUT_DIRECT_QUEUE_SHADER_RESOURCE,
            ImageLayout::CopySrc => D3D12_BARRIER_LAYOUT_DIRECT_QUEUE_COPY_SOURCE,
            ImageLayout::CopyDst => D3D12_BARRIER_LAYOUT_DIRECT_QUEUE_COPY_DEST,
            ImageLayout::UnorderedAccess => D3D12_BARRIER_LAYOUT_DIRECT_QUEUE_UNORDERED_ACCESS,
            ImageLayout::ResolveSource => D3D12_BARRIER_LAYOUT_RESOLVE_SOURCE,
            ImageLayout::ResolveDest => D3D12_BARRIER_LAYOUT_RESOLVE_DEST,
            ImageLayout::ShadingRateAttachment => D3D12_BARRIER_LAYOUT_SHADING_RATE_SOURCE,
        },
        Some(QueueType::Compute) => match layout {
            ImageLayout::Undefined => D3D12_BARRIER_LAYOUT_UNDEFINED,
            ImageLayout::Common => D3D12_BARRIER_LAYOUT_COMPUTE_QUEUE_GENERIC_READ,
            ImageLayout::PresentSrc => D3D12_BARRIER_LAYOUT_PRESENT,
            ImageLayout::ColorAttachment => D3D12_BARRIER_LAYOUT_RENDER_TARGET,
            ImageLayout::DepthStencilAttachment => D3D12_BARRIER_LAYOUT_DEPTH_STENCIL_WRITE,
            ImageLayout::DepthStencilReadOnly => D3D12_BARRIER_LAYOUT_DEPTH_STENCIL_READ,
            ImageLayout::ShaderReadOnly => D3D12_BARRIER_LAYOUT_COMPUTE_QUEUE_SHADER_RESOURCE,
            ImageLayout::CopySrc => D3D12_BARRIER_LAYOUT_COMPUTE_QUEUE_COPY_SOURCE,
            ImageLayout::CopyDst => D3D12_BARRIER_LAYOUT_COMPUTE_QUEUE_COPY_DEST,
            ImageLayout::UnorderedAccess => D3D12_BARRIER_LAYOUT_COMPUTE_QUEUE_UNORDERED_ACCESS,
            ImageLayout::ResolveSource => D3D12_BARRIER_LAYOUT_RESOLVE_SOURCE,
            ImageLayout::ResolveDest => D3D12_BARRIER_LAYOUT_RESOLVE_DEST,
            ImageLayout::ShadingRateAttachment => D3D12_BARRIER_LAYOUT_SHADING_RATE_SOURCE,
        },
        Some(QueueType::Transfer) | None => match layout {
            ImageLayout::Undefined => D3D12_BARRIER_LAYOUT_UNDEFINED,
            ImageLayout::Common => D3D12_BARRIER_LAYOUT_GENERIC_READ,
            ImageLayout::PresentSrc => D3D12_BARRIER_LAYOUT_PRESENT,
            ImageLayout::ColorAttachment => D3D12_BARRIER_LAYOUT_RENDER_TARGET,
            ImageLayout::DepthStencilAttachment => D3D12_BARRIER_LAYOUT_DEPTH_STENCIL_WRITE,
            ImageLayout::DepthStencilReadOnly => D3D12_BARRIER_LAYOUT_DEPTH_STENCIL_READ,
            ImageLayout::ShaderReadOnly => D3D12_BARRIER_LAYOUT_SHADER_RESOURCE,
            ImageLayout::CopySrc => D3D12_BARRIER_LAYOUT_COPY_SOURCE,
            ImageLayout::CopyDst => D3D12_BARRIER_LAYOUT_COPY_DEST,
            ImageLayout::UnorderedAccess => D3D12_BARRIER_LAYOUT_UNORDERED_ACCESS,
            ImageLayout::ResolveSource => D3D12_BARRIER_LAYOUT_RESOLVE_SOURCE,
            ImageLayout::ResolveDest => D3D12_BARRIER_LAYOUT_RESOLVE_DEST,
            ImageLayout::ShadingRateAttachment => D3D12_BARRIER_LAYOUT_SHADING_RATE_SOURCE,
        },
    }
}

macro_rules! translate_flag_onto {
    ($src:ident, $dst:ident, $src_flag:expr, $dst_flag:expr) => {
        #[allow(clippy::assign_op_pattern)]
        if ($src.contains($src_flag)) {
            $dst = $dst | $dst_flag;
        }
    };
}

pub fn barrier_sync_to_dx12(sync: BarrierSync) -> D3D12_BARRIER_SYNC {
    let mut out = D3D12_BARRIER_SYNC::default();
    translate_flag_onto!(sync, out, BarrierSync::ALL, D3D12_BARRIER_SYNC_ALL);
    translate_flag_onto!(sync, out, BarrierSync::DRAW, D3D12_BARRIER_SYNC_DRAW);
    translate_flag_onto!(
        sync,
        out,
        BarrierSync::INDEX_INPUT,
        D3D12_BARRIER_SYNC_INDEX_INPUT
    );
    translate_flag_onto!(
        sync,
        out,
        BarrierSync::VERTEX_SHADING,
        D3D12_BARRIER_SYNC_VERTEX_SHADING
    );
    translate_flag_onto!(
        sync,
        out,
        BarrierSync::PIXEL_SHADING,
        D3D12_BARRIER_SYNC_PIXEL_SHADING
    );
    translate_flag_onto!(
        sync,
        out,
        BarrierSync::DEPTH_STENCIL,
        D3D12_BARRIER_SYNC_DEPTH_STENCIL
    );
    translate_flag_onto!(
        sync,
        out,
        BarrierSync::RENDER_TARGET,
        D3D12_BARRIER_SYNC_RENDER_TARGET
    );
    translate_flag_onto!(
        sync,
        out,
        BarrierSync::COMPUTE_SHADING,
        D3D12_BARRIER_SYNC_COMPUTE_SHADING
    );
    translate_flag_onto!(
        sync,
        out,
        BarrierSync::RAYTRACING,
        D3D12_BARRIER_SYNC_RAYTRACING
    );
    translate_flag_onto!(sync, out, BarrierSync::COPY, D3D12_BARRIER_SYNC_COPY);
    translate_flag_onto!(sync, out, BarrierSync::RESOLVE, D3D12_BARRIER_SYNC_RESOLVE);
    translate_flag_onto!(
        sync,
        out,
        BarrierSync::EXECUTE_INDIRECT,
        D3D12_BARRIER_SYNC_EXECUTE_INDIRECT
    );
    translate_flag_onto!(
        sync,
        out,
        BarrierSync::CLEAR_UNORDERED_ACCESS_VIEW,
        D3D12_BARRIER_SYNC_CLEAR_UNORDERED_ACCESS_VIEW
    );
    translate_flag_onto!(
        sync,
        out,
        BarrierSync::BUILD_RAYTRACING_ACCELERATION_STRUCTURE,
        (D3D12_BARRIER_SYNC_BUILD_RAYTRACING_ACCELERATION_STRUCTURE
            | D3D12_BARRIER_SYNC_EMIT_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO)
    );
    translate_flag_onto!(
        sync,
        out,
        BarrierSync::COPY_RAYTRACING_ACCELERATION_STRUCTURE,
        D3D12_BARRIER_SYNC_COPY_RAYTRACING_ACCELERATION_STRUCTURE
    );

    out
}

pub fn barrier_access_to_dx12(access: BarrierAccess) -> D3D12_BARRIER_ACCESS {
    let mut out = D3D12_BARRIER_ACCESS::default();
    if access.is_empty() {
        // RHI uses 0 set bits for no access like vulkan
        return D3D12_BARRIER_ACCESS_NO_ACCESS;
    }
    translate_flag_onto!(
        access,
        out,
        BarrierAccess::VERTEX_BUFFER_READ,
        D3D12_BARRIER_ACCESS_VERTEX_BUFFER
    );
    translate_flag_onto!(
        access,
        out,
        BarrierAccess::INDEX_BUFFER_READ,
        D3D12_BARRIER_ACCESS_INDEX_BUFFER
    );
    translate_flag_onto!(
        access,
        out,
        BarrierAccess::CONSTANT_BUFFER_READ,
        D3D12_BARRIER_ACCESS_CONSTANT_BUFFER
    );
    translate_flag_onto!(
        access,
        out,
        BarrierAccess::INDIRECT_COMMAND_READ,
        D3D12_BARRIER_ACCESS_INDIRECT_ARGUMENT
    );
    translate_flag_onto!(
        access,
        out,
        BarrierAccess::RENDER_TARGET_READ,
        D3D12_BARRIER_ACCESS_RENDER_TARGET
    );
    translate_flag_onto!(
        access,
        out,
        BarrierAccess::RENDER_TARGET_WRITE,
        D3D12_BARRIER_ACCESS_RENDER_TARGET
    );
    translate_flag_onto!(
        access,
        out,
        BarrierAccess::DEPTH_STENCIL_READ,
        D3D12_BARRIER_ACCESS_DEPTH_STENCIL_READ
    );
    translate_flag_onto!(
        access,
        out,
        BarrierAccess::DEPTH_STENCIL_WRITE,
        D3D12_BARRIER_ACCESS_DEPTH_STENCIL_WRITE
    );
    translate_flag_onto!(
        access,
        out,
        BarrierAccess::COPY_READ,
        D3D12_BARRIER_ACCESS_COPY_SOURCE
    );
    translate_flag_onto!(
        access,
        out,
        BarrierAccess::COPY_WRITE,
        D3D12_BARRIER_ACCESS_COPY_DEST
    );
    translate_flag_onto!(
        access,
        out,
        BarrierAccess::RAYTRACING_ACCELERATION_STRUCTURE_READ,
        D3D12_BARRIER_ACCESS_RAYTRACING_ACCELERATION_STRUCTURE_READ
    );
    translate_flag_onto!(
        access,
        out,
        BarrierAccess::RAYTRACING_ACCELERATION_STRUCTURE_WRITE,
        D3D12_BARRIER_ACCESS_RAYTRACING_ACCELERATION_STRUCTURE_WRITE
    );
    translate_flag_onto!(
        access,
        out,
        BarrierAccess::SHADER_READ,
        D3D12_BARRIER_ACCESS_SHADER_RESOURCE
    );
    translate_flag_onto!(
        access,
        out,
        BarrierAccess::SHADER_WRITE,
        D3D12_BARRIER_ACCESS_UNORDERED_ACCESS
    );

    out
}

pub fn texture_create_desc_to_dx12(
    desc: &TextureDesc,
) -> Result<D3D12_RESOURCE_DESC1, TextureCreateError> {
    let (dimension, depth_or_array_size) = match desc.dimension {
        TextureDimension::Texture1D => {
            if desc.array_size >= u16::MAX as _ {
                return Err(TextureCreateError::InvalidArraySize(desc.array_size));
            }
            (D3D12_RESOURCE_DIMENSION_TEXTURE1D, desc.array_size)
        }
        TextureDimension::Texture2D => {
            if desc.array_size >= u16::MAX as _ {
                return Err(TextureCreateError::InvalidArraySize(desc.array_size));
            }
            (D3D12_RESOURCE_DIMENSION_TEXTURE2D, desc.array_size)
        }
        TextureDimension::Texture3D => {
            if desc.depth >= u16::MAX as _ {
                return Err(TextureCreateError::InvalidDepth(desc.depth));
            }
            if desc.array_size >= 1 {
                return Err(TextureCreateError::InvalidArraySize(desc.array_size));
            }
            (D3D12_RESOURCE_DIMENSION_TEXTURE3D, desc.depth)
        }
    };

    if desc.mip_levels >= u16::MAX as _ {
        return Err(TextureCreateError::InvalidMipLevelCount(desc.mip_levels));
    }

    if !desc.sample_count.is_power_of_two() || desc.sample_count > 16 {
        return Err(TextureCreateError::InvalidSampleCount(desc.sample_count));
    }

    let mut flags = D3D12_RESOURCE_FLAGS::default();
    if desc.usage.contains(ResourceUsageFlags::RENDER_TARGET) {
        if desc.format.is_depth_stencil() {
            flags |= D3D12_RESOURCE_FLAG_ALLOW_DEPTH_STENCIL;
        } else {
            flags |= D3D12_RESOURCE_FLAG_ALLOW_RENDER_TARGET;
        }
    }

    if desc.usage.contains(ResourceUsageFlags::UNORDERED_ACCESS) {
        flags |= D3D12_RESOURCE_FLAG_ALLOW_UNORDERED_ACCESS;
    }

    let out = D3D12_RESOURCE_DESC1 {
        Dimension: dimension,
        Alignment: 0,
        Width: desc.width as u64,
        Height: desc.height,
        DepthOrArraySize: depth_or_array_size as u16,
        MipLevels: desc.mip_levels as u16,
        Format: texture_format_to_dxgi(desc.format),
        SampleDesc: DXGI_SAMPLE_DESC {
            Count: desc.sample_count,
            Quality: desc.sample_quality,
        },
        Layout: D3D12_TEXTURE_LAYOUT_UNKNOWN,
        Flags: flags,
        SamplerFeedbackMipRegion: Default::default(),
    };
    Ok(out)
}

pub fn texture_create_clear_value_to_dx12(
    desc: &TextureDesc,
    format: DXGI_FORMAT,
) -> Result<Option<D3D12_CLEAR_VALUE>, TextureCreateError> {
    let clear = if let Some(clear) = &desc.clear_value {
        let clear = clear.clone();
        match clear.clone() {
            OptimalClearValue::ColorF32 { r, g, b, a } => {
                if !desc.format.is_depth_stencil() {
                    Some(D3D12_CLEAR_VALUE {
                        Format: format,
                        Anonymous: D3D12_CLEAR_VALUE_0 {
                            Color: [r, g, b, a],
                        },
                    })
                } else {
                    return Err(TextureCreateError::InvalidOptimalClearValue(clear));
                }
            }
            OptimalClearValue::ColorInt(v) => {
                if !desc.format.is_depth_stencil() {
                    Some(D3D12_CLEAR_VALUE {
                        Format: format,
                        Anonymous: D3D12_CLEAR_VALUE_0 {
                            Color: decode_u32_color_to_float(v),
                        },
                    })
                } else {
                    return Err(TextureCreateError::InvalidOptimalClearValue(clear));
                }
            }
            OptimalClearValue::DepthStencil(depth, stencil) => {
                if desc.format.is_depth_stencil() {
                    Some(D3D12_CLEAR_VALUE {
                        Format: format,
                        Anonymous: D3D12_CLEAR_VALUE_0 {
                            DepthStencil: D3D12_DEPTH_STENCIL_VALUE {
                                Depth: depth,
                                Stencil: stencil,
                            },
                        },
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

pub trait TranslateClearValue {
    fn translate_clear_value(&self, format: Option<impl Into<DXGI_FORMAT>>) -> D3D12_CLEAR_VALUE;
}

impl TranslateClearValue for ColorClearValue {
    #[inline]
    fn translate_clear_value(&self, format: Option<impl Into<DXGI_FORMAT>>) -> D3D12_CLEAR_VALUE {
        let values = match self {
            ColorClearValue::Float { r, g, b, a } => [*r, *g, *b, *a],
            ColorClearValue::Int(v) => decode_u32_color_to_float(*v),
        };

        D3D12_CLEAR_VALUE {
            Format: format.map(|v| v.into()).unwrap_or_default(),
            Anonymous: D3D12_CLEAR_VALUE_0 { Color: values },
        }
    }
}

impl TranslateClearValue for DepthStencilClearValue {
    #[inline]
    fn translate_clear_value(&self, format: Option<impl Into<DXGI_FORMAT>>) -> D3D12_CLEAR_VALUE {
        let v = D3D12_DEPTH_STENCIL_VALUE {
            Depth: self.depth,
            Stencil: self.stencil,
        };
        D3D12_CLEAR_VALUE {
            Format: format.map(|v| v.into()).unwrap_or_default(),
            Anonymous: D3D12_CLEAR_VALUE_0 { DepthStencil: v },
        }
    }
}

pub fn translate_beginning_access(
    load_op: &AttachmentLoadOp<impl TranslateClearValue>,
    format: Option<impl Into<DXGI_FORMAT>>,
) -> D3D12_RENDER_PASS_BEGINNING_ACCESS {
    match load_op {
        AttachmentLoadOp::Load => D3D12_RENDER_PASS_BEGINNING_ACCESS {
            Type: D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE_PRESERVE,
            Anonymous: Default::default(),
        },
        AttachmentLoadOp::Clear(clear_value) => D3D12_RENDER_PASS_BEGINNING_ACCESS {
            Type: D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE_CLEAR,
            Anonymous: D3D12_RENDER_PASS_BEGINNING_ACCESS_0 {
                Clear: D3D12_RENDER_PASS_BEGINNING_ACCESS_CLEAR_PARAMETERS {
                    ClearValue: clear_value.translate_clear_value(format),
                },
            },
        },
        AttachmentLoadOp::DontCare => D3D12_RENDER_PASS_BEGINNING_ACCESS {
            Type: D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE_DISCARD,
            Anonymous: Default::default(),
        },
        AttachmentLoadOp::None => D3D12_RENDER_PASS_BEGINNING_ACCESS {
            Type: D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE_NO_ACCESS,
            Anonymous: Default::default(),
        },
    }
}

pub fn translate_ending_access(store_op: &AttachmentStoreOp) -> D3D12_RENDER_PASS_ENDING_ACCESS {
    match store_op {
        AttachmentStoreOp::Store => D3D12_RENDER_PASS_ENDING_ACCESS {
            Type: D3D12_RENDER_PASS_ENDING_ACCESS_TYPE_PRESERVE,
            Anonymous: Default::default(),
        },
        AttachmentStoreOp::DontCare => D3D12_RENDER_PASS_ENDING_ACCESS {
            Type: D3D12_RENDER_PASS_ENDING_ACCESS_TYPE_DISCARD,
            Anonymous: Default::default(),
        },
        AttachmentStoreOp::None => D3D12_RENDER_PASS_ENDING_ACCESS {
            Type: D3D12_RENDER_PASS_ENDING_ACCESS_TYPE_NO_ACCESS,
            Anonymous: Default::default(),
        },
    }
}

pub fn translate_rendering_color_attachment(
    info: &RenderingColorAttachmentInfo,
    descriptor: impl Into<D3D12_CPU_DESCRIPTOR_HANDLE>,
    format: Option<impl Into<DXGI_FORMAT>>,
) -> D3D12_RENDER_PASS_RENDER_TARGET_DESC {
    D3D12_RENDER_PASS_RENDER_TARGET_DESC {
        cpuDescriptor: descriptor.into(),
        BeginningAccess: translate_beginning_access(&info.load_op, format),
        EndingAccess: translate_ending_access(&info.store_op),
    }
}

pub fn translate_rendering_depth_stencil_attachment(
    info: &RenderingDepthStencilAttachmentInfo,
    descriptor: impl Into<D3D12_CPU_DESCRIPTOR_HANDLE>,
    format: Option<impl Into<DXGI_FORMAT>>,
) -> D3D12_RENDER_PASS_DEPTH_STENCIL_DESC {
    let format = format.map(|v| v.into());
    D3D12_RENDER_PASS_DEPTH_STENCIL_DESC {
        cpuDescriptor: descriptor.into(),
        DepthBeginningAccess: translate_beginning_access(&info.depth_load_op, format),
        StencilBeginningAccess: translate_beginning_access(&info.stencil_load_op, format),
        DepthEndingAccess: translate_ending_access(&info.depth_store_op),
        StencilEndingAccess: translate_ending_access(&info.stencil_store_op),
    }
}

/// Returns (FirstPlane, NumPlanes) for the given barrier texture aspect
pub const fn translate_barrier_texture_aspect_to_plane_range(
    aspect: TextureAspect,
    _format: Format,
) -> (u32, u32) {
    if aspect.contains(TextureAspect::DEPTH_STENCIL) {
        // Depth/stencil formats will always have depth as plane 0 and stencil as plane
        // 1. We can emit (0, 2) when issuing a barrier for both aspects
        // unconditionally.
        (0, 2)
    } else if aspect.contains(TextureAspect::DEPTH) {
        // All depth formats we support have depth in plane 0, so we can always emit
        // (0, 1) without checking the format
        (0, 1)
    } else if aspect.contains(TextureAspect::STENCIL) {
        // We don't support any stencil-only formats, so stencil will always be plane 1.
        // This means we can always assume (1, 1) for stencil only aspect\
        (1, 1)
    } else if aspect.contains(TextureAspect::COLOR) {
        // All formats we support that have a color aspect have the color in plane zero.
        // This means we can always assume color aspect to be (0, 1)
        (0, 1)
    } else {
        (0, 0)
    }
}

pub const fn descriptor_type_to_dx12(v: DescriptorType) -> D3D12_DESCRIPTOR_RANGE_TYPE {
    match v {
        DescriptorType::Sampler => D3D12_DESCRIPTOR_RANGE_TYPE_SAMPLER,
        DescriptorType::TexelBuffer => D3D12_DESCRIPTOR_RANGE_TYPE_SRV,
        DescriptorType::TexelBufferRW => D3D12_DESCRIPTOR_RANGE_TYPE_UAV,
        DescriptorType::Texture => D3D12_DESCRIPTOR_RANGE_TYPE_SRV,
        DescriptorType::TextureRW => D3D12_DESCRIPTOR_RANGE_TYPE_UAV,
        DescriptorType::UniformBuffer => D3D12_DESCRIPTOR_RANGE_TYPE_CBV,
        DescriptorType::StructuredBuffer => D3D12_DESCRIPTOR_RANGE_TYPE_SRV,
        DescriptorType::StructuredBufferRW => D3D12_DESCRIPTOR_RANGE_TYPE_UAV,
        DescriptorType::ByteAddressBuffer => D3D12_DESCRIPTOR_RANGE_TYPE_SRV,
        DescriptorType::ByteAddressBufferRW => D3D12_DESCRIPTOR_RANGE_TYPE_UAV,
        DescriptorType::AccelerationStructure => D3D12_DESCRIPTOR_RANGE_TYPE_SRV,
        DescriptorType::InputAttachment => D3D12_DESCRIPTOR_RANGE_TYPE_SRV,
    }
}
