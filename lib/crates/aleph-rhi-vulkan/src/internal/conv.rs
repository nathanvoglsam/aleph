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
use ash::vk;

/// Internal function for converting texture format to VkFormat
pub const fn texture_format_to_vk(format: Format) -> vk::Format {
    match format {
        Format::R8Unorm => vk::Format::R8_UNORM,
        Format::R8Snorm => vk::Format::R8_SNORM,
        Format::R8Uint => vk::Format::R8_UINT,
        Format::R8Sint => vk::Format::R8_SINT,
        Format::R16Uint => vk::Format::R16_UINT,
        Format::R16Sint => vk::Format::R16_SINT,
        Format::R16Unorm => vk::Format::R16_UNORM,
        Format::R16Snorm => vk::Format::R16_SNORM,
        Format::R16Float => vk::Format::R16_SFLOAT,
        Format::Rg8Unorm => vk::Format::R8G8_UNORM,
        Format::Rg8Snorm => vk::Format::R8G8_SNORM,
        Format::Rg8Uint => vk::Format::R8G8_UINT,
        Format::Rg8Sint => vk::Format::R8G8_SINT,
        Format::R32Uint => vk::Format::R32_UINT,
        Format::R32Sint => vk::Format::R32_SINT,
        Format::R32Float => vk::Format::R32_SFLOAT,
        Format::Rg16Uint => vk::Format::R16G16_UINT,
        Format::Rg16Sint => vk::Format::R16G16_SINT,
        Format::Rg16Unorm => vk::Format::R16G16_UNORM,
        Format::Rg16Snorm => vk::Format::R16G16_SNORM,
        Format::Rg16Float => vk::Format::R16G16_SFLOAT,
        Format::Rgba8Unorm => vk::Format::R8G8B8A8_UNORM,
        Format::Rgba8UnormSrgb => vk::Format::R8G8B8A8_SRGB,
        Format::Rgba8Snorm => vk::Format::R8G8B8A8_SNORM,
        Format::Rgba8Uint => vk::Format::R8G8B8A8_UINT,
        Format::Rgba8Sint => vk::Format::R8G8B8A8_SINT,
        Format::Bgra8Unorm => vk::Format::B8G8R8A8_UNORM,
        Format::Bgra8UnormSrgb => vk::Format::B8G8R8A8_SRGB,
        Format::Rgb10a2Unorm => vk::Format::A2B10G10R10_UNORM_PACK32,
        Format::Rg11b10Float => vk::Format::B10G11R11_UFLOAT_PACK32,
        Format::Rg32Uint => vk::Format::R32G32B32_UINT,
        Format::Rg32Sint => vk::Format::R32G32B32_SINT,
        Format::Rg32Float => vk::Format::R32G32B32_SFLOAT,
        Format::Rgba16Uint => vk::Format::R16G16B16A16_UINT,
        Format::Rgba16Sint => vk::Format::R16G16B16A16_SINT,
        Format::Rgba16Unorm => vk::Format::R16G16B16A16_UNORM,
        Format::Rgba16Snorm => vk::Format::R16G16B16A16_SNORM,
        Format::Rgba16Float => vk::Format::R16G16B16A16_SFLOAT,
        Format::Rgba32Uint => vk::Format::R32G32B32A32_UINT,
        Format::Rgba32Sint => vk::Format::R32G32B32A32_SINT,
        Format::Rgba32Float => vk::Format::R32G32B32A32_SFLOAT,
        Format::Depth32Float => vk::Format::D32_SFLOAT,
        Format::Depth24Stencil8 => vk::Format::D24_UNORM_S8_UINT,
    }
}

/// Internal function for converting present mode to VkPresentModeKHR
pub const fn present_mode_to_vk(mode: PresentationMode) -> vk::PresentModeKHR {
    match mode {
        PresentationMode::Immediate => vk::PresentModeKHR::IMMEDIATE,
        PresentationMode::Mailbox => vk::PresentModeKHR::MAILBOX,
        PresentationMode::Fifo => vk::PresentModeKHR::FIFO,
    }
}

pub const fn stencil_op_to_vk(op: StencilOp) -> vk::StencilOp {
    match op {
        StencilOp::Keep => vk::StencilOp::KEEP,
        StencilOp::Zero => vk::StencilOp::ZERO,
        StencilOp::Replace => vk::StencilOp::REPLACE,
        StencilOp::IncrementClamp => vk::StencilOp::INCREMENT_AND_CLAMP,
        StencilOp::DecrementClamp => vk::StencilOp::DECREMENT_AND_CLAMP,
        StencilOp::Invert => vk::StencilOp::INVERT,
        StencilOp::IncrementWrap => vk::StencilOp::INCREMENT_AND_WRAP,
        StencilOp::DecrementWrap => vk::StencilOp::DECREMENT_AND_WRAP,
    }
}

pub const fn compare_op_to_vk(op: CompareOp) -> vk::CompareOp {
    match op {
        CompareOp::Never => vk::CompareOp::NEVER,
        CompareOp::Always => vk::CompareOp::ALWAYS,
        CompareOp::Equal => vk::CompareOp::EQUAL,
        CompareOp::NotEqual => vk::CompareOp::NOT_EQUAL,
        CompareOp::Less => vk::CompareOp::LESS,
        CompareOp::LessEqual => vk::CompareOp::LESS_OR_EQUAL,
        CompareOp::Greater => vk::CompareOp::GREATER,
        CompareOp::GreaterOrEqual => vk::CompareOp::GREATER_OR_EQUAL,
    }
}

pub const fn primitive_topology_to_vk(topology: PrimitiveTopology) -> vk::PrimitiveTopology {
    match topology {
        PrimitiveTopology::PointList => vk::PrimitiveTopology::POINT_LIST,
        PrimitiveTopology::LineList => vk::PrimitiveTopology::LINE_LIST,
        PrimitiveTopology::LineStrip => vk::PrimitiveTopology::LINE_STRIP,
        PrimitiveTopology::TriangleList => vk::PrimitiveTopology::TRIANGLE_LIST,
        PrimitiveTopology::TriangleStrip => vk::PrimitiveTopology::TRIANGLE_STRIP,
    }
}

pub const fn polygon_mode_to_vk(mode: PolygonMode) -> vk::PolygonMode {
    match mode {
        PolygonMode::Fill => vk::PolygonMode::FILL,
        PolygonMode::Line => vk::PolygonMode::LINE,
    }
}

pub const fn front_face_order_to_vk(order: FrontFaceOrder) -> vk::FrontFace {
    match order {
        FrontFaceOrder::CounterClockwise => vk::FrontFace::COUNTER_CLOCKWISE,
        FrontFaceOrder::Clockwise => vk::FrontFace::CLOCKWISE,
    }
}

pub const fn cull_mode_to_vk(cull_mode: CullMode) -> vk::CullModeFlags {
    match cull_mode {
        CullMode::None => vk::CullModeFlags::NONE,
        CullMode::Back => vk::CullModeFlags::BACK,
        CullMode::Front => vk::CullModeFlags::FRONT,
    }
}

pub const fn vertex_input_rate_to_vk(input_rate: VertexInputRate) -> vk::VertexInputRate {
    match input_rate {
        VertexInputRate::PerVertex => vk::VertexInputRate::VERTEX,
        VertexInputRate::PerInstance => vk::VertexInputRate::INSTANCE,
    }
}

pub const fn blend_factor_to_vk(factor: BlendFactor) -> vk::BlendFactor {
    match factor {
        BlendFactor::Zero => vk::BlendFactor::ZERO,
        BlendFactor::One => vk::BlendFactor::ONE,
        BlendFactor::SrcColor => vk::BlendFactor::SRC_COLOR,
        BlendFactor::OneMinusSrcColor => vk::BlendFactor::ONE_MINUS_SRC_COLOR,
        BlendFactor::DstColor => vk::BlendFactor::DST_COLOR,
        BlendFactor::OneMinusDstColor => vk::BlendFactor::ONE_MINUS_DST_COLOR,
        BlendFactor::SrcAlpha => vk::BlendFactor::SRC_ALPHA,
        BlendFactor::OneMinusSrcAlpha => vk::BlendFactor::ONE_MINUS_SRC_ALPHA,
        BlendFactor::DstAlpha => vk::BlendFactor::DST_ALPHA,
        BlendFactor::OneMinusDstAlpha => vk::BlendFactor::ONE_MINUS_DST_ALPHA,
        BlendFactor::SrcAlphaSaturate => vk::BlendFactor::SRC_ALPHA_SATURATE,
        BlendFactor::BlendFactor => vk::BlendFactor::CONSTANT_COLOR,
        BlendFactor::OneMinusBlendFactor => vk::BlendFactor::ONE_MINUS_CONSTANT_COLOR,
    }
}

pub const fn blend_op_to_vk(op: BlendOp) -> vk::BlendOp {
    match op {
        BlendOp::Add => vk::BlendOp::ADD,
        BlendOp::Subtract => vk::BlendOp::SUBTRACT,
        BlendOp::ReverseSubtract => vk::BlendOp::REVERSE_SUBTRACT,
        BlendOp::Min => vk::BlendOp::MIN,
        BlendOp::Max => vk::BlendOp::MAX,
    }
}

pub fn image_layout_to_vk(layout: ImageLayout) -> vk::ImageLayout {
    match layout {
        ImageLayout::Undefined => vk::ImageLayout::UNDEFINED,
        ImageLayout::Common => vk::ImageLayout::GENERAL,
        ImageLayout::PresentSrc => vk::ImageLayout::PRESENT_SRC_KHR,
        ImageLayout::ColorAttachmentOptimal => vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
        ImageLayout::DepthStencilAttachmentOptimal => {
            vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL
        }
        ImageLayout::DepthStencilReadOnlyOptimal => {
            vk::ImageLayout::DEPTH_STENCIL_READ_ONLY_OPTIMAL
        }
        ImageLayout::ShaderReadOnlyOptimal => vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
        ImageLayout::CopySrc => vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
        ImageLayout::CopyDst => vk::ImageLayout::TRANSFER_DST_OPTIMAL,
        ImageLayout::UnorderedAccess => vk::ImageLayout::GENERAL,
        ImageLayout::ResolveSource => todo!(),
        ImageLayout::ResolveDest => todo!(),
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

pub fn barrier_sync_to_vk2(sync: BarrierSync) -> vk::PipelineStageFlags2 {
    let mut out = vk::PipelineStageFlags2::empty();
    translate_flag_onto!(
        sync,
        out,
        BarrierSync::ALL,
        vk::PipelineStageFlags2::ALL_COMMANDS
    );
    translate_flag_onto!(
        sync,
        out,
        BarrierSync::DRAW,
        vk::PipelineStageFlags2::ALL_GRAPHICS
    );
    translate_flag_onto!(
        sync,
        out,
        BarrierSync::INDEX_INPUT,
        vk::PipelineStageFlags2::INDEX_INPUT
    );
    translate_flag_onto!(
        sync,
        out,
        BarrierSync::VERTEX_SHADING,
        (vk::PipelineStageFlags2::PRE_RASTERIZATION_SHADERS
            | vk::PipelineStageFlags2::VERTEX_ATTRIBUTE_INPUT)
    );
    translate_flag_onto!(
        sync,
        out,
        BarrierSync::PIXEL_SHADING,
        vk::PipelineStageFlags2::FRAGMENT_SHADER
    );
    translate_flag_onto!(
        sync,
        out,
        BarrierSync::DEPTH_STENCIL,
        (vk::PipelineStageFlags2::EARLY_FRAGMENT_TESTS
            | vk::PipelineStageFlags2::LATE_FRAGMENT_TESTS)
    );
    translate_flag_onto!(
        sync,
        out,
        BarrierSync::RENDER_TARGET,
        vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT
    );
    translate_flag_onto!(
        sync,
        out,
        BarrierSync::COMPUTE_SHADING,
        vk::PipelineStageFlags2::COMPUTE_SHADER
    );
    translate_flag_onto!(
        sync,
        out,
        BarrierSync::RAYTRACING,
        vk::PipelineStageFlags2::RAY_TRACING_SHADER_KHR
    );
    translate_flag_onto!(sync, out, BarrierSync::COPY, vk::PipelineStageFlags2::COPY);
    translate_flag_onto!(
        sync,
        out,
        BarrierSync::RESOLVE,
        vk::PipelineStageFlags2::RESOLVE
    );
    translate_flag_onto!(
        sync,
        out,
        BarrierSync::EXECUTE_INDIRECT,
        vk::PipelineStageFlags2::DRAW_INDIRECT
    );
    translate_flag_onto!(
        sync,
        out,
        BarrierSync::CLEAR_UNORDERED_ACCESS_VIEW,
        vk::PipelineStageFlags2::CLEAR
    );
    translate_flag_onto!(
        sync,
        out,
        BarrierSync::BUILD_RAYTRACING_ACCELERATION_STRUCTURE,
        vk::PipelineStageFlags2::ACCELERATION_STRUCTURE_BUILD_KHR
    );
    translate_flag_onto!(
        sync,
        out,
        BarrierSync::COPY_RAYTRACING_ACCELERATION_STRUCTURE,
        todo!()
    ); //vk::PipelineStageFlags2::ACCELERATION_STRUCTURE_COPY_KHR

    out
}

#[derive(Clone)]
pub struct SyncShaderFeatures {
    /// Whether tessellation shaders are enabled
    pub tessellation: bool,

    /// Whether geometry shaders are enabled
    pub geometry: bool,

    /// Whether mesh shaders are enabled
    pub mesh: bool,

    /// Whether task shaders are enabled
    pub task: bool,
}

pub fn barrier_sync_to_vk(
    sync: BarrierSync,
    enabled_shader_features: &SyncShaderFeatures,
) -> vk::PipelineStageFlags {
    let mut out = vk::PipelineStageFlags::empty();
    translate_flag_onto!(
        sync,
        out,
        BarrierSync::ALL,
        vk::PipelineStageFlags::ALL_COMMANDS
    );
    translate_flag_onto!(
        sync,
        out,
        BarrierSync::DRAW,
        vk::PipelineStageFlags::ALL_GRAPHICS
    );
    translate_flag_onto!(
        sync,
        out,
        BarrierSync::INDEX_INPUT,
        vk::PipelineStageFlags::VERTEX_INPUT
    );
    #[allow(clippy::assign_op_pattern)]
    if sync.contains(BarrierSync::VERTEX_SHADING) {
        out = out | vk::PipelineStageFlags::VERTEX_SHADER | vk::PipelineStageFlags::VERTEX_INPUT;

        if enabled_shader_features.tessellation {
            out = out | (vk::PipelineStageFlags::TESSELLATION_CONTROL_SHADER);
            out = out | (vk::PipelineStageFlags::TESSELLATION_EVALUATION_SHADER);
        }
        if enabled_shader_features.geometry {
            out = out | (vk::PipelineStageFlags::GEOMETRY_SHADER);
        }
        if enabled_shader_features.mesh {
            out = out | (vk::PipelineStageFlags::MESH_SHADER_NV)
        };
        if enabled_shader_features.task {
            out = out | (vk::PipelineStageFlags::TASK_SHADER_NV)
        };
    }
    translate_flag_onto!(
        sync,
        out,
        BarrierSync::PIXEL_SHADING,
        vk::PipelineStageFlags::FRAGMENT_SHADER
    );
    translate_flag_onto!(
        sync,
        out,
        BarrierSync::DEPTH_STENCIL,
        (vk::PipelineStageFlags::EARLY_FRAGMENT_TESTS
            | vk::PipelineStageFlags::LATE_FRAGMENT_TESTS)
    );
    translate_flag_onto!(
        sync,
        out,
        BarrierSync::RENDER_TARGET,
        vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT
    );
    translate_flag_onto!(
        sync,
        out,
        BarrierSync::COMPUTE_SHADING,
        vk::PipelineStageFlags::COMPUTE_SHADER
    );
    translate_flag_onto!(
        sync,
        out,
        BarrierSync::RAYTRACING,
        vk::PipelineStageFlags::RAY_TRACING_SHADER_KHR
    );
    translate_flag_onto!(
        sync,
        out,
        BarrierSync::COPY,
        vk::PipelineStageFlags::TRANSFER
    );
    translate_flag_onto!(
        sync,
        out,
        BarrierSync::RESOLVE,
        vk::PipelineStageFlags::TRANSFER
    );
    translate_flag_onto!(
        sync,
        out,
        BarrierSync::EXECUTE_INDIRECT,
        vk::PipelineStageFlags::DRAW_INDIRECT
    );
    translate_flag_onto!(
        sync,
        out,
        BarrierSync::CLEAR_UNORDERED_ACCESS_VIEW,
        vk::PipelineStageFlags::TRANSFER
    );
    translate_flag_onto!(
        sync,
        out,
        BarrierSync::BUILD_RAYTRACING_ACCELERATION_STRUCTURE,
        vk::PipelineStageFlags::ACCELERATION_STRUCTURE_BUILD_KHR
    );
    translate_flag_onto!(
        sync,
        out,
        BarrierSync::COPY_RAYTRACING_ACCELERATION_STRUCTURE,
        todo!()
    ); //vk::PipelineStageFlags2::ACCELERATION_STRUCTURE_COPY_KHR

    out
}

pub fn barrier_access_to_vk2(access: BarrierAccess) -> vk::AccessFlags2 {
    let mut out = vk::AccessFlags2::empty();
    translate_flag_onto!(
        access,
        out,
        BarrierAccess::VERTEX_BUFFER_READ,
        vk::AccessFlags2::VERTEX_ATTRIBUTE_READ
    );
    translate_flag_onto!(
        access,
        out,
        BarrierAccess::INDEX_BUFFER_READ,
        vk::AccessFlags2::INDEX_READ
    );
    translate_flag_onto!(
        access,
        out,
        BarrierAccess::CONSTANT_BUFFER_READ,
        vk::AccessFlags2::UNIFORM_READ
    );
    translate_flag_onto!(
        access,
        out,
        BarrierAccess::INDIRECT_COMMAND_READ,
        vk::AccessFlags2::INDIRECT_COMMAND_READ
    );
    translate_flag_onto!(
        access,
        out,
        BarrierAccess::SHADER_SAMPLED_READ,
        vk::AccessFlags2::SHADER_SAMPLED_READ
    );
    translate_flag_onto!(
        access,
        out,
        BarrierAccess::RENDER_TARGET_READ,
        vk::AccessFlags2::COLOR_ATTACHMENT_READ
    );
    translate_flag_onto!(
        access,
        out,
        BarrierAccess::RENDER_TARGET_WRITE,
        vk::AccessFlags2::COLOR_ATTACHMENT_WRITE
    );
    translate_flag_onto!(
        access,
        out,
        BarrierAccess::DEPTH_STENCIL_READ,
        vk::AccessFlags2::DEPTH_STENCIL_ATTACHMENT_READ
    );
    translate_flag_onto!(
        access,
        out,
        BarrierAccess::DEPTH_STENCIL_WRITE,
        vk::AccessFlags2::DEPTH_STENCIL_ATTACHMENT_WRITE
    );
    translate_flag_onto!(
        access,
        out,
        BarrierAccess::COPY_READ,
        vk::AccessFlags2::TRANSFER_READ
    );
    translate_flag_onto!(
        access,
        out,
        BarrierAccess::COPY_WRITE,
        vk::AccessFlags2::TRANSFER_WRITE
    );
    translate_flag_onto!(
        access,
        out,
        BarrierAccess::RAYTRACING_ACCELERATION_STRUCTURE_READ,
        vk::AccessFlags2::ACCELERATION_STRUCTURE_READ_KHR
    );
    translate_flag_onto!(
        access,
        out,
        BarrierAccess::RAYTRACING_ACCELERATION_STRUCTURE_WRITE,
        vk::AccessFlags2::ACCELERATION_STRUCTURE_WRITE_KHR
    );

    out
}

pub fn barrier_access_to_vk(access: BarrierAccess) -> vk::AccessFlags {
    let mut out = vk::AccessFlags::empty();
    translate_flag_onto!(
        access,
        out,
        BarrierAccess::VERTEX_BUFFER_READ,
        vk::AccessFlags::VERTEX_ATTRIBUTE_READ
    );
    translate_flag_onto!(
        access,
        out,
        BarrierAccess::INDEX_BUFFER_READ,
        vk::AccessFlags::INDEX_READ
    );
    translate_flag_onto!(
        access,
        out,
        BarrierAccess::CONSTANT_BUFFER_READ,
        vk::AccessFlags::UNIFORM_READ
    );
    translate_flag_onto!(
        access,
        out,
        BarrierAccess::INDIRECT_COMMAND_READ,
        vk::AccessFlags::INDIRECT_COMMAND_READ
    );
    translate_flag_onto!(
        access,
        out,
        BarrierAccess::SHADER_SAMPLED_READ,
        vk::AccessFlags::SHADER_READ
    );
    translate_flag_onto!(
        access,
        out,
        BarrierAccess::RENDER_TARGET_READ,
        vk::AccessFlags::COLOR_ATTACHMENT_READ
    );
    translate_flag_onto!(
        access,
        out,
        BarrierAccess::RENDER_TARGET_WRITE,
        vk::AccessFlags::COLOR_ATTACHMENT_WRITE
    );
    translate_flag_onto!(
        access,
        out,
        BarrierAccess::DEPTH_STENCIL_READ,
        vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_READ
    );
    translate_flag_onto!(
        access,
        out,
        BarrierAccess::DEPTH_STENCIL_WRITE,
        vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE
    );
    translate_flag_onto!(
        access,
        out,
        BarrierAccess::COPY_READ,
        vk::AccessFlags::TRANSFER_READ
    );
    translate_flag_onto!(
        access,
        out,
        BarrierAccess::COPY_WRITE,
        vk::AccessFlags::TRANSFER_WRITE
    );
    translate_flag_onto!(
        access,
        out,
        BarrierAccess::RAYTRACING_ACCELERATION_STRUCTURE_READ,
        vk::AccessFlags::ACCELERATION_STRUCTURE_READ_KHR
    );
    translate_flag_onto!(
        access,
        out,
        BarrierAccess::RAYTRACING_ACCELERATION_STRUCTURE_WRITE,
        vk::AccessFlags::ACCELERATION_STRUCTURE_WRITE_KHR
    );

    out
}

pub const fn load_op_to_vk<T>(load_op: &AttachmentLoadOp<T>) -> vk::AttachmentLoadOp {
    match load_op {
        AttachmentLoadOp::Load => vk::AttachmentLoadOp::LOAD,
        AttachmentLoadOp::Clear(_) => vk::AttachmentLoadOp::CLEAR,
        AttachmentLoadOp::DontCare => vk::AttachmentLoadOp::DONT_CARE,
        AttachmentLoadOp::None => vk::AttachmentLoadOp::NONE_EXT,
    }
}

pub const fn store_op_to_vk(store_op: &AttachmentStoreOp) -> vk::AttachmentStoreOp {
    match store_op {
        AttachmentStoreOp::Store => vk::AttachmentStoreOp::STORE,
        AttachmentStoreOp::DontCare => vk::AttachmentStoreOp::DONT_CARE,
        AttachmentStoreOp::None => vk::AttachmentStoreOp::NONE,
    }
}

pub const fn texture_copy_aspect_to_vk(aspect: TextureCopyAspect) -> vk::ImageAspectFlags {
    match aspect {
        TextureCopyAspect::Color => vk::ImageAspectFlags::COLOR,
        TextureCopyAspect::Depth => vk::ImageAspectFlags::DEPTH,
        TextureCopyAspect::Stencil => vk::ImageAspectFlags::STENCIL,
    }
}

pub const fn texture_aspect_to_vk(aspect: TextureAspect) -> vk::ImageAspectFlags {
    // # SAFETY #
    // It shouldn't be possible to construct a TextureAspect with invalid flags without unsafe code.
    // TextureAspect is a subset of vk::ImageAspectFlags so any TextureAspect flag is valid and
    // matches the vk::ImageAspectFlags value it represents. This means this is sound, in isolation
    // anyway.
    vk::ImageAspectFlags::from_raw(aspect.bits())
}

pub fn color_clear_to_vk(v: &ColorClearValue) -> vk::ClearColorValue {
    match v {
        ColorClearValue::Float { r, g, b, a } => vk::ClearColorValue {
            float32: [*r, *g, *b, *a],
        },
        ColorClearValue::Int(v) => vk::ClearColorValue {
            float32: decode_u32_color_to_float(*v),
        },
    }
}

pub const fn depth_stencil_clear_to_vk(v: DepthStencilClearValue) -> vk::ClearDepthStencilValue {
    vk::ClearDepthStencilValue {
        depth: v.depth,
        stencil: v.stencil as u32,
    }
}

pub const fn pipeline_bind_point_to_vk(v: PipelineBindPoint) -> vk::PipelineBindPoint {
    match v {
        PipelineBindPoint::Graphics => vk::PipelineBindPoint::GRAPHICS,
        PipelineBindPoint::Compute => vk::PipelineBindPoint::COMPUTE,
    }
}

pub const fn descriptor_shader_visibility_to_vk(
    v: DescriptorShaderVisibility,
) -> vk::ShaderStageFlags {
    match v {
        DescriptorShaderVisibility::All => vk::ShaderStageFlags::ALL,
        DescriptorShaderVisibility::Compute => vk::ShaderStageFlags::COMPUTE,
        DescriptorShaderVisibility::Vertex => vk::ShaderStageFlags::VERTEX,
        DescriptorShaderVisibility::Hull => vk::ShaderStageFlags::TESSELLATION_CONTROL,
        DescriptorShaderVisibility::Domain => vk::ShaderStageFlags::TESSELLATION_EVALUATION,
        DescriptorShaderVisibility::Geometry => vk::ShaderStageFlags::GEOMETRY,
        DescriptorShaderVisibility::Fragment => vk::ShaderStageFlags::FRAGMENT,
        DescriptorShaderVisibility::Amplification => vk::ShaderStageFlags::TASK_NV,
        DescriptorShaderVisibility::Mesh => vk::ShaderStageFlags::MESH_NV,
    }
}

pub const fn descriptor_type_to_vk(v: DescriptorType) -> vk::DescriptorType {
    match v {
        DescriptorType::Sampler => vk::DescriptorType::SAMPLER,
        DescriptorType::SampledImage => vk::DescriptorType::SAMPLED_IMAGE,
        DescriptorType::StorageImage => vk::DescriptorType::STORAGE_IMAGE,
        DescriptorType::UniformTexelBuffer => vk::DescriptorType::UNIFORM_TEXEL_BUFFER,
        DescriptorType::StorageTexelBuffer => vk::DescriptorType::STORAGE_TEXEL_BUFFER,
        DescriptorType::UniformBuffer => vk::DescriptorType::UNIFORM_BUFFER,
        DescriptorType::StorageBuffer => vk::DescriptorType::STORAGE_BUFFER,
        DescriptorType::StructuredBuffer => vk::DescriptorType::STORAGE_BUFFER,
        DescriptorType::InputAttachment => vk::DescriptorType::INPUT_ATTACHMENT,
    }
}

pub const fn sampler_filter_to_vk(v: SamplerFilter) -> vk::Filter {
    match v {
        SamplerFilter::Nearest => vk::Filter::NEAREST,
        SamplerFilter::Linear => vk::Filter::LINEAR,
    }
}

pub const fn sampler_mip_filter_to_vk(v: SamplerMipFilter) -> vk::SamplerMipmapMode {
    match v {
        SamplerMipFilter::Nearest => vk::SamplerMipmapMode::NEAREST,
        SamplerMipFilter::Linear => vk::SamplerMipmapMode::LINEAR,
    }
}

pub const fn sampler_address_mode_to_vk(v: SamplerAddressMode) -> vk::SamplerAddressMode {
    match v {
        SamplerAddressMode::Wrap => vk::SamplerAddressMode::REPEAT,
        SamplerAddressMode::Mirror => vk::SamplerAddressMode::MIRRORED_REPEAT,
        SamplerAddressMode::Clamp => vk::SamplerAddressMode::CLAMP_TO_EDGE,
        SamplerAddressMode::Border => vk::SamplerAddressMode::CLAMP_TO_BORDER,
        SamplerAddressMode::MirrorOnce => vk::SamplerAddressMode::MIRROR_CLAMP_TO_EDGE,
    }
}

pub const fn sampler_border_color_to_vk(v: SamplerBorderColor) -> vk::BorderColor {
    match v {
        SamplerBorderColor::BlackTransparent => vk::BorderColor::FLOAT_TRANSPARENT_BLACK,
        SamplerBorderColor::BlackOpaque => vk::BorderColor::FLOAT_OPAQUE_BLACK,
        SamplerBorderColor::WhiteOpaque => vk::BorderColor::FLOAT_OPAQUE_WHITE,
    }
}

pub const fn shader_type_to_vk(v: ShaderType) -> vk::ShaderStageFlags {
    match v {
        ShaderType::Compute => vk::ShaderStageFlags::COMPUTE,
        ShaderType::Vertex => vk::ShaderStageFlags::VERTEX,
        ShaderType::Hull => vk::ShaderStageFlags::TESSELLATION_CONTROL,
        ShaderType::Domain => vk::ShaderStageFlags::TESSELLATION_EVALUATION,
        ShaderType::Geometry => vk::ShaderStageFlags::GEOMETRY,
        ShaderType::Fragment => vk::ShaderStageFlags::FRAGMENT,
        ShaderType::Amplification => vk::ShaderStageFlags::TASK_NV,
        ShaderType::Mesh => vk::ShaderStageFlags::MESH_NV,
    }
}

pub const fn image_view_type_to_vk(v: ImageViewType) -> vk::ImageViewType {
    match v {
        ImageViewType::Tex1D => vk::ImageViewType::TYPE_1D,
        ImageViewType::Tex2D => vk::ImageViewType::TYPE_2D,
        ImageViewType::Tex3D => vk::ImageViewType::TYPE_3D,
        ImageViewType::TexCube => vk::ImageViewType::CUBE,
        ImageViewType::TexArray1D => vk::ImageViewType::TYPE_1D_ARRAY,
        ImageViewType::TexArray2D => vk::ImageViewType::TYPE_2D_ARRAY,
        ImageViewType::TexCubeArray => vk::ImageViewType::CUBE_ARRAY,
    }
}

pub const fn subresource_range_to_vk(v: &TextureSubResourceSet) -> vk::ImageSubresourceRange {
    vk::ImageSubresourceRange {
        aspect_mask: texture_aspect_to_vk(v.aspect),
        base_mip_level: v.base_mip_level,
        level_count: v.num_mip_levels,
        base_array_layer: v.base_array_slice,
        layer_count: v.num_array_slices,
    }
}
