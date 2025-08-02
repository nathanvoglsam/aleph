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
use objc2_metal::*;

pub const fn format_to_pixel_mtl(format: Format) -> MTLPixelFormat {
    match format {
        Format::R8Unorm => MTLPixelFormat::R8Unorm,
        Format::R8Snorm => MTLPixelFormat::R8Snorm,
        Format::R8Uint => MTLPixelFormat::R8Uint,
        Format::R8Sint => MTLPixelFormat::R8Sint,
        Format::R16Uint => MTLPixelFormat::R16Uint,
        Format::R16Sint => MTLPixelFormat::R16Sint,
        Format::R16Unorm => MTLPixelFormat::R16Unorm,
        Format::R16Snorm => MTLPixelFormat::R16Snorm,
        Format::R16Float => MTLPixelFormat::R16Float,
        Format::R32Uint => MTLPixelFormat::R32Uint,
        Format::R32Sint => MTLPixelFormat::R32Sint,
        Format::R32Float => MTLPixelFormat::R32Float,
        Format::Rg8Unorm => MTLPixelFormat::RG8Unorm,
        Format::Rg8Snorm => MTLPixelFormat::RG8Snorm,
        Format::Rg8Uint => MTLPixelFormat::RG8Uint,
        Format::Rg8Sint => MTLPixelFormat::RG8Sint,
        Format::Rg16Uint => MTLPixelFormat::RG16Uint,
        Format::Rg16Sint => MTLPixelFormat::RG16Sint,
        Format::Rg16Unorm => MTLPixelFormat::RG16Unorm,
        Format::Rg16Snorm => MTLPixelFormat::RG16Snorm,
        Format::Rg16Float => MTLPixelFormat::RG16Float,
        Format::Rg32Uint => MTLPixelFormat::RG32Uint,
        Format::Rg32Sint => MTLPixelFormat::RG32Sint,
        Format::Rg32Float => MTLPixelFormat::RG32Float,
        Format::Rgb32Uint => unimplemented!(),
        Format::Rgb32Sint => unimplemented!(),
        Format::Rgb32Float => unimplemented!(),
        Format::Rgba8Unorm => MTLPixelFormat::RGBA8Unorm,
        Format::Rgba8UnormSrgb => MTLPixelFormat::RGBA8Unorm_sRGB,
        Format::Rgba8Snorm => MTLPixelFormat::RGBA8Snorm,
        Format::Rgba8Uint => MTLPixelFormat::RGBA8Uint,
        Format::Rgba8Sint => MTLPixelFormat::RGBA8Sint,
        Format::Bgra8Unorm => MTLPixelFormat::BGRA8Unorm,
        Format::Bgra8UnormSrgb => MTLPixelFormat::BGRA8Unorm_sRGB,
        Format::Rgb10a2Unorm => MTLPixelFormat::RGB10A2Unorm,
        Format::Rg11b10Float => MTLPixelFormat::RG11B10Float,
        Format::Rgb9e5Float => MTLPixelFormat::RGB9E5Float,
        Format::Rgba16Uint => MTLPixelFormat::RGBA16Uint,
        Format::Rgba16Sint => MTLPixelFormat::RGBA16Sint,
        Format::Rgba16Unorm => MTLPixelFormat::RGBA16Unorm,
        Format::Rgba16Snorm => MTLPixelFormat::RGBA16Snorm,
        Format::Rgba16Float => MTLPixelFormat::RGBA16Float,
        Format::Rgba32Uint => MTLPixelFormat::RGBA32Uint,
        Format::Rgba32Sint => MTLPixelFormat::RGBA32Sint,
        Format::Rgba32Float => MTLPixelFormat::RGBA32Float,
        Format::Depth32Float => MTLPixelFormat::Depth32Float,
        Format::Depth32FloatStencil8 => MTLPixelFormat::Depth32Float_Stencil8,
        Format::Depth24Stencil8 => MTLPixelFormat::Depth24Unorm_Stencil8,
    }
}

pub const fn pixel_mtl_to_format(format: MTLPixelFormat) -> Format {
    match format {
        MTLPixelFormat::R8Unorm => Format::R8Unorm,
        MTLPixelFormat::R8Snorm => Format::R8Snorm,
        MTLPixelFormat::R8Uint => Format::R8Uint,
        MTLPixelFormat::R8Sint => Format::R8Sint,
        MTLPixelFormat::R16Uint => Format::R16Uint,
        MTLPixelFormat::R16Sint => Format::R16Sint,
        MTLPixelFormat::R16Unorm => Format::R16Unorm,
        MTLPixelFormat::R16Snorm => Format::R16Snorm,
        MTLPixelFormat::R16Float => Format::R16Float,
        MTLPixelFormat::R32Uint => Format::R32Uint,
        MTLPixelFormat::R32Sint => Format::R32Sint,
        MTLPixelFormat::R32Float => Format::R32Float,
        MTLPixelFormat::RG8Unorm => Format::Rg8Unorm,
        MTLPixelFormat::RG8Snorm => Format::Rg8Snorm,
        MTLPixelFormat::RG8Uint => Format::Rg8Uint,
        MTLPixelFormat::RG8Sint => Format::Rg8Sint,
        MTLPixelFormat::RG16Uint => Format::Rg16Uint,
        MTLPixelFormat::RG16Sint => Format::Rg16Sint,
        MTLPixelFormat::RG16Unorm => Format::Rg16Unorm,
        MTLPixelFormat::RG16Snorm => Format::Rg16Snorm,
        MTLPixelFormat::RG16Float => Format::Rg16Float,
        MTLPixelFormat::RG32Uint => Format::Rg32Uint,
        MTLPixelFormat::RG32Sint => Format::Rg32Sint,
        MTLPixelFormat::RG32Float => Format::Rg32Float,
        MTLPixelFormat::RGBA8Unorm => Format::Rgba8Unorm,
        MTLPixelFormat::RGBA8Unorm_sRGB => Format::Rgba8UnormSrgb,
        MTLPixelFormat::RGBA8Snorm => Format::Rgba8Snorm,
        MTLPixelFormat::RGBA8Uint => Format::Rgba8Uint,
        MTLPixelFormat::RGBA8Sint => Format::Rgba8Sint,
        MTLPixelFormat::BGRA8Unorm => Format::Bgra8Unorm,
        MTLPixelFormat::BGRA8Unorm_sRGB => Format::Bgra8UnormSrgb,
        MTLPixelFormat::RGB10A2Unorm => Format::Rgb10a2Unorm,
        MTLPixelFormat::RG11B10Float => Format::Rg11b10Float,
        MTLPixelFormat::RGB9E5Float => Format::Rgb9e5Float,
        MTLPixelFormat::RGBA16Uint => Format::Rgba16Uint,
        MTLPixelFormat::RGBA16Sint => Format::Rgba16Sint,
        MTLPixelFormat::RGBA16Unorm => Format::Rgba16Unorm,
        MTLPixelFormat::RGBA16Snorm => Format::Rgba16Snorm,
        MTLPixelFormat::RGBA16Float => Format::Rgba16Float,
        MTLPixelFormat::RGBA32Uint => Format::Rgba32Uint,
        MTLPixelFormat::RGBA32Sint => Format::Rgba32Sint,
        MTLPixelFormat::RGBA32Float => Format::Rgba32Float,
        MTLPixelFormat::Depth32Float => Format::Depth32Float,
        MTLPixelFormat::Depth32Float_Stencil8 => Format::Depth32FloatStencil8,
        MTLPixelFormat::Depth24Unorm_Stencil8 => Format::Depth24Stencil8,
        _ => unimplemented!(),
    }
}

pub const fn rect_to_mtl_scissor_rect(rect: &Rect) -> MTLScissorRect {
    MTLScissorRect {
        x: rect.x as usize,
        y: rect.y as usize,
        width: rect.w as usize,
        height: rect.h as usize,
    }
}

pub const fn viewport_to_mtl(viewport: &Viewport) -> MTLViewport {
    MTLViewport {
        originX: viewport.x as f64,
        originY: viewport.y as f64,
        width: viewport.width as f64,
        height: viewport.height as f64,
        znear: viewport.min_depth as f64,
        zfar: viewport.max_depth as f64,
    }
}

pub const fn index_type_to_mtl(index_type: IndexType) -> MTLIndexType {
    match index_type {
        IndexType::U16 => MTLIndexType::UInt16,
        IndexType::U32 => MTLIndexType::UInt32,
    }
}

pub const fn index_type_to_size(index_type: IndexType) -> usize {
    match index_type {
        IndexType::U16 => 2,
        IndexType::U32 => 4,
    }
}

pub const fn primitive_topology_to_mtl(v: PrimitiveTopology) -> MTLPrimitiveType {
    match v {
        PrimitiveTopology::PointList => MTLPrimitiveType::Point,
        PrimitiveTopology::LineList => MTLPrimitiveType::Line,
        PrimitiveTopology::LineStrip => MTLPrimitiveType::LineStrip,
        PrimitiveTopology::TriangleList => MTLPrimitiveType::Triangle,
        PrimitiveTopology::TriangleStrip => MTLPrimitiveType::TriangleStrip,
    }
}

pub const fn primitive_topology_to_mtl_class(v: PrimitiveTopology) -> MTLPrimitiveTopologyClass {
    match v {
        PrimitiveTopology::PointList => MTLPrimitiveTopologyClass::Point,
        PrimitiveTopology::LineList => MTLPrimitiveTopologyClass::Line,
        PrimitiveTopology::LineStrip => MTLPrimitiveTopologyClass::Line,
        PrimitiveTopology::TriangleList => MTLPrimitiveTopologyClass::Triangle,
        PrimitiveTopology::TriangleStrip => MTLPrimitiveTopologyClass::Triangle,
    }
}

pub const fn sampler_filter_to_mtl(v: SamplerFilter) -> MTLSamplerMinMagFilter {
    match v {
        SamplerFilter::Nearest => MTLSamplerMinMagFilter::Nearest,
        SamplerFilter::Linear => MTLSamplerMinMagFilter::Linear,
    }
}

pub const fn sampler_mip_filter_to_mtl(v: SamplerMipFilter) -> MTLSamplerMipFilter {
    match v {
        SamplerMipFilter::Nearest => MTLSamplerMipFilter::Nearest,
        SamplerMipFilter::Linear => MTLSamplerMipFilter::Linear,
    }
}

pub const fn address_mode_to_mtl(v: SamplerAddressMode) -> MTLSamplerAddressMode {
    match v {
        SamplerAddressMode::Wrap => MTLSamplerAddressMode::Repeat,
        SamplerAddressMode::Mirror => MTLSamplerAddressMode::MirrorRepeat,
        SamplerAddressMode::Clamp => MTLSamplerAddressMode::ClampToEdge,
        SamplerAddressMode::Border => MTLSamplerAddressMode::ClampToBorderColor,
        SamplerAddressMode::MirrorOnce => MTLSamplerAddressMode::MirrorClampToEdge,
    }
}

pub const fn compare_op_to_mtl(v: CompareOp) -> MTLCompareFunction {
    match v {
        CompareOp::Never => MTLCompareFunction::Never,
        CompareOp::Always => MTLCompareFunction::Always,
        CompareOp::Equal => MTLCompareFunction::Equal,
        CompareOp::NotEqual => MTLCompareFunction::NotEqual,
        CompareOp::Less => MTLCompareFunction::Less,
        CompareOp::LessEqual => MTLCompareFunction::LessEqual,
        CompareOp::Greater => MTLCompareFunction::Greater,
        CompareOp::GreaterOrEqual => MTLCompareFunction::GreaterEqual,
    }
}

pub const fn border_color_to_mtl(v: SamplerBorderColor) -> MTLSamplerBorderColor {
    match v {
        SamplerBorderColor::BlackTransparent => MTLSamplerBorderColor::TransparentBlack,
        SamplerBorderColor::BlackOpaque => MTLSamplerBorderColor::OpaqueBlack,
        SamplerBorderColor::WhiteOpaque => MTLSamplerBorderColor::OpaqueWhite,
    }
}

pub const fn u_offset_to_mtl_origin(v: &UOffset3D) -> MTLOrigin {
    MTLOrigin {
        x: v.x as usize,
        y: v.y as usize,
        z: v.z as usize,
    }
}

pub const fn extent_to_mtl_size(v: &Extent3D) -> MTLSize {
    MTLSize {
        width: v.width as usize,
        height: v.height as usize,
        depth: v.depth as usize,
    }
}

pub const fn cull_mode_to_mtl(v: CullMode) -> MTLCullMode {
    match v {
        CullMode::None => MTLCullMode::None,
        CullMode::Back => MTLCullMode::Back,
        CullMode::Front => MTLCullMode::Front,
    }
}

pub const fn front_face_order_to_mtl(v: FrontFaceOrder) -> MTLWinding {
    match v {
        FrontFaceOrder::CounterClockwise => MTLWinding::CounterClockwise,
        FrontFaceOrder::Clockwise => MTLWinding::Clockwise,
    }
}

pub const fn polygon_mode_to_mtl(v: PolygonMode) -> MTLTriangleFillMode {
    match v {
        PolygonMode::Fill => MTLTriangleFillMode::Fill,
        PolygonMode::Line => MTLTriangleFillMode::Lines,
    }
}

pub const fn stencil_op_to_mtl(v: StencilOp) -> MTLStencilOperation {
    match v {
        StencilOp::Keep => MTLStencilOperation::Keep,
        StencilOp::Zero => MTLStencilOperation::Zero,
        StencilOp::Replace => MTLStencilOperation::Replace,
        StencilOp::IncrementClamp => MTLStencilOperation::IncrementClamp,
        StencilOp::DecrementClamp => MTLStencilOperation::DecrementClamp,
        StencilOp::Invert => MTLStencilOperation::Invert,
        StencilOp::IncrementWrap => MTLStencilOperation::IncrementWrap,
        StencilOp::DecrementWrap => MTLStencilOperation::DecrementWrap,
    }
}

pub const fn attachment_store_op_to_mtl(v: AttachmentStoreOp) -> MTLStoreAction {
    match v {
        AttachmentStoreOp::Store => MTLStoreAction::Store,
        AttachmentStoreOp::DontCare => MTLStoreAction::DontCare,
        AttachmentStoreOp::None => unimplemented!(),
    }
}

pub const fn write_mask_to_mtl(v: ColorComponentFlags) -> MTLColorWriteMask {
    if v.bits() == ColorComponentFlags::all().bits() {
        return MTLColorWriteMask::All;
    }

    let mut out = MTLColorWriteMask::empty();
    if v.contains(ColorComponentFlags::R) {
        out.0 |= MTLColorWriteMask::Red.0;
    }
    if v.contains(ColorComponentFlags::G) {
        out.0 |= MTLColorWriteMask::Green.0;
    }
    if v.contains(ColorComponentFlags::B) {
        out.0 |= MTLColorWriteMask::Blue.0;
    }
    if v.contains(ColorComponentFlags::A) {
        out.0 |= MTLColorWriteMask::Alpha.0;
    }

    out
}

pub const fn blend_op_to_mtl(v: BlendOp) -> MTLBlendOperation {
    match v {
        BlendOp::Add => MTLBlendOperation::Add,
        BlendOp::Subtract => MTLBlendOperation::Subtract,
        BlendOp::ReverseSubtract => MTLBlendOperation::ReverseSubtract,
        BlendOp::Min => MTLBlendOperation::Min,
        BlendOp::Max => MTLBlendOperation::Max,
    }
}

pub const fn blend_factor_to_mtl(v: BlendFactor) -> MTLBlendFactor {
    match v {
        BlendFactor::Zero => MTLBlendFactor::Zero,
        BlendFactor::One => MTLBlendFactor::One,
        BlendFactor::SrcColor => MTLBlendFactor::SourceColor,
        BlendFactor::OneMinusSrcColor => MTLBlendFactor::OneMinusSourceColor,
        BlendFactor::DstColor => MTLBlendFactor::DestinationColor,
        BlendFactor::OneMinusDstColor => MTLBlendFactor::OneMinusDestinationColor,
        BlendFactor::SrcAlpha => MTLBlendFactor::SourceAlpha,
        BlendFactor::OneMinusSrcAlpha => MTLBlendFactor::OneMinusSourceAlpha,
        BlendFactor::DstAlpha => MTLBlendFactor::DestinationAlpha,
        BlendFactor::OneMinusDstAlpha => MTLBlendFactor::OneMinusDestinationAlpha,
        BlendFactor::SrcAlphaSaturate => MTLBlendFactor::SourceAlphaSaturated,
        BlendFactor::BlendFactor => MTLBlendFactor::BlendColor,
        BlendFactor::OneMinusBlendFactor => MTLBlendFactor::OneMinusBlendColor,
    }
}

pub const fn alpha_blend_factor_to_mtl(v: BlendFactor) -> MTLBlendFactor {
    match v {
        BlendFactor::Zero => MTLBlendFactor::Zero,
        BlendFactor::One => MTLBlendFactor::One,
        BlendFactor::SrcColor => MTLBlendFactor::SourceColor,
        BlendFactor::OneMinusSrcColor => MTLBlendFactor::OneMinusSourceColor,
        BlendFactor::DstColor => MTLBlendFactor::DestinationColor,
        BlendFactor::OneMinusDstColor => MTLBlendFactor::OneMinusDestinationColor,
        BlendFactor::SrcAlpha => MTLBlendFactor::SourceAlpha,
        BlendFactor::OneMinusSrcAlpha => MTLBlendFactor::OneMinusSourceAlpha,
        BlendFactor::DstAlpha => MTLBlendFactor::DestinationAlpha,
        BlendFactor::OneMinusDstAlpha => MTLBlendFactor::OneMinusDestinationAlpha,
        BlendFactor::SrcAlphaSaturate => MTLBlendFactor::SourceAlphaSaturated,
        BlendFactor::BlendFactor => MTLBlendFactor::BlendAlpha,
        BlendFactor::OneMinusBlendFactor => MTLBlendFactor::OneMinusBlendAlpha,
    }
}

pub const fn image_view_type_to_mtl(v: ImageViewType) -> MTLTextureType {
    match v {
        ImageViewType::Tex1D => MTLTextureType::Type1D,
        ImageViewType::Tex2D => MTLTextureType::Type2D,
        ImageViewType::Tex3D => MTLTextureType::Type3D,
        ImageViewType::TexCube => MTLTextureType::TypeCube,
        ImageViewType::TexArray1D => MTLTextureType::Type1DArray,
        ImageViewType::TexArray2D => MTLTextureType::Type2DArray,
        ImageViewType::TexCubeArray => MTLTextureType::TypeCubeArray,
    }
}

pub const fn resource_usage_to_texture_usage_mtl(v: ResourceUsageFlags) -> MTLTextureUsage {
    let mut usage = MTLTextureUsage::empty();

    if v.contains(ResourceUsageFlags::COPY_SOURCE) {
        usage.0 |= MTLTextureUsage::ShaderRead.0;
    }
    if v.contains(ResourceUsageFlags::COPY_DEST) {
        usage.0 |= MTLTextureUsage::ShaderWrite.0;
    }
    if v.contains(ResourceUsageFlags::SHADER_RESOURCE) {
        usage.0 |= MTLTextureUsage::ShaderRead.0;
    }
    if v.contains(ResourceUsageFlags::SHADER_RESOURCE) {
        usage.0 |= MTLTextureUsage::ShaderRead.0;
    }
    if v.contains(ResourceUsageFlags::UNORDERED_ACCESS) {
        usage.0 |= MTLTextureUsage::ShaderWrite.0;
        usage.0 |= MTLTextureUsage::ShaderAtomic.0;
    }
    if v.contains(ResourceUsageFlags::RENDER_TARGET) {
        usage.0 |= MTLTextureUsage::RenderTarget.0;
    }

    usage
}
