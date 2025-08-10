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

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum ImageLayout {
    /// Specifies that the layout is unknown.
    Undefined,

    /// Supports all types of read device access. Writable access is not possible through this
    /// layout.
    Common,

    /// Must only be used for presenting a presentable image for display.
    PresentSrc,

    /// Layout suitable for use as a color attachment render target, for either read-only or
    /// read/write access.
    ColorAttachment,

    /// Layout suitable for use as a depth stencil attachment render target, for either read-only
    /// or read/write access.
    DepthStencilAttachment,

    /// Similar to [ImageLayout::ShaderReadOnly]. Allow usage as a read-only shader resource
    /// as well as a read-only depth/stencil attachment.
    DepthStencilReadOnly,

    /// Specifies a layout allowing read-only access in a shader.
    ShaderReadOnly,

    /// Must only be used as a source image of a copy command.
    CopySrc,

    /// Must only be used as a destination image of a copy command.
    CopyDst,

    /// Layout suitable for access as a UAV (read/write shader resource). May not be used as a
    /// render target or any other usage.
    UnorderedAccess,

    /// The layout an image is required to be in for multi-sample resolve operations as the source
    /// of a resolve.
    ResolveSource,

    /// The layout an image is required to be in for multi-sample resolve operations as the
    /// destinations of a resolve.
    ResolveDest,

    /// Must only be used as a fragment shading rate attachment or shading rate image.
    ShadingRateAttachment,
}

impl std::fmt::Display for ImageLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageLayout::Undefined => f.write_str("Undefined"),
            ImageLayout::Common => f.write_str("Common"),
            ImageLayout::PresentSrc => f.write_str("PresentSrc"),
            ImageLayout::ColorAttachment => f.write_str("ColorAttachment"),
            ImageLayout::DepthStencilAttachment => f.write_str("DepthStencilAttachment"),
            ImageLayout::DepthStencilReadOnly => f.write_str("DepthStencilReadOnly"),
            ImageLayout::ShaderReadOnly => f.write_str("ShaderReadOnly"),
            ImageLayout::CopySrc => f.write_str("CopySrc"),
            ImageLayout::CopyDst => f.write_str("CopyDst"),
            ImageLayout::UnorderedAccess => f.write_str("UnorderedAccess"),
            ImageLayout::ResolveSource => f.write_str("ResolveSource"),
            ImageLayout::ResolveDest => f.write_str("ResolveDest"),
            ImageLayout::ShadingRateAttachment => f.write_str("ShadingRateAttachment"),
        }
    }
}

impl Default for ImageLayout {
    #[inline(always)]
    fn default() -> Self {
        ImageLayout::Undefined
    }
}

bitflags::bitflags! {
    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
    pub struct BarrierSync: u64 {
        ///
        /// ## Vulkan
        ///
        /// - `NONE`
        ///
        /// ## D3D12
        ///
        /// - `NONE`
        ///
        const NONE = 0x0;

        ///
        /// ## Vulkan
        ///
        /// - `ALL_COMMANDS_BIT`
        ///
        /// ## D3D12
        ///
        /// - `ALL`
        ///
        const ALL  = 0x1;

        ///
        /// ## Vulkan
        ///
        /// - `ALL_GRAPHICS_BIT`
        ///
        /// ## D3D12
        ///
        /// - `DRAW`
        ///
        const DRAW  = 0x2;

        ///
        /// ## Vulkan
        ///
        /// - `INDEX_INPUT_BIT`
        ///
        /// ## D3D12
        ///
        /// - `INDEX_INPUT`
        ///
        const INDEX_INPUT = 0x4;

        ///
        /// ## Vulkan
        ///
        /// - `PRE_RASTERIZATION_SHADERS_BIT | VERTEX_ATTRIBUTE_INPUT`
        ///
        /// ## D3D12
        ///
        /// - `VERTEX_SHADING`
        ///
        const VERTEX_SHADING = 0x8;

        ///
        /// ## Vulkan
        ///
        /// - `FRAGMENT_SHADER_BIT`
        ///
        /// ## D3D12
        ///
        /// - `PIXEL_SHADING`
        ///
        const PIXEL_SHADING = 0x10;

        ///
        /// ## Vulkan
        ///
        /// - `EARLY_FRAGMENT_TESTS_BIT`
        /// - `LATE_FRAGMENT_TESTS_BIT`
        ///
        /// ## D3D12
        ///
        /// - `DEPTH_STENCIL`
        ///
        const DEPTH_STENCIL = 0x20;

        ///
        /// ## Vulkan
        ///
        /// - `COLOR_ATTACHMENT_OUTPUT_BIT`
        ///
        /// ## D3D12
        ///
        /// - `RENDER_TARGET`
        ///
        const RENDER_TARGET = 0x40;

        ///
        /// ## Vulkan
        ///
        /// - `COMPUTE_SHADER_BIT`
        ///
        /// ## D3D12
        ///
        /// - `COMPUTE_SHADING`
        ///
        const COMPUTE_SHADING = 0x80;

        ///
        /// ## Vulkan
        ///
        /// - `RAY_TRACING_SHADER_BIT`
        ///
        /// ## D3D12
        ///
        /// - `RAYTRACING`
        ///
        const RAYTRACING = 0x100;

        ///
        /// ## Vulkan
        ///
        /// - `COPY_BIT`
        ///
        /// ## D3D12
        ///
        /// - `COPY`
        ///
        const COPY = 0x200;

        ///
        /// ## Vulkan
        ///
        /// - `RESOLVE_BIT`
        ///
        /// ## D3D12
        ///
        /// - `RESOLVE`
        ///
        const RESOLVE = 0x400;

        ///
        /// ## Vulkan
        ///
        /// - `DRAW_INDIRECT_BIT`
        ///
        /// ## D3D12
        ///
        /// - `EXECUTE_INDIRECT`
        /// - `PREDICATION`
        ///
        const EXECUTE_INDIRECT = 0x800;

        // const ALL_SHADING = 0x1000;

        ///
        /// ## Warning
        ///
        /// I don't know if this is needed, or can be mapped in a sane way. This will describe what
        /// I think this should map to.
        ///
        /// We can just implement these with compute shaders
        ///
        /// ## Vulkan
        ///
        /// - `COMPUTE_SHADER`
        ///
        /// ## D3D12
        ///
        /// - `CLEAR_UNORDERED_ACCESS_VIEW`
        ///
        const CLEAR_UNORDERED_ACCESS_VIEW = 0x8000;

        ///
        /// ## Vulkan
        ///
        /// - `ACCELERATION_STRUCTURE_BUILD_BIT`
        ///
        /// ## D3D12
        ///
        /// - `BUILD_RAYTRACING_ACCELERATION_STRUCTURE`
        /// - `EMIT_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO`
        ///
        const BUILD_RAYTRACING_ACCELERATION_STRUCTURE = 0x800000;

        ///
        /// ## Vulkan
        ///
        /// - `ACCELERATION_STRUCTURE_COPY_BIT`
        ///
        /// ## D3D12
        ///
        /// - `COPY_RAYTRACING_ACCELERATION_STRUCTURE`
        ///
        const COPY_RAYTRACING_ACCELERATION_STRUCTURE = 0x1000000;
    }
}

impl Default for BarrierSync {
    #[inline(always)]
    fn default() -> Self {
        Self::NONE
    }
}

bitflags::bitflags! {
    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
    pub struct BarrierAccess: u64 {

        ///
        /// ## Vulkan
        ///
        /// - `NONE`
        ///
        /// ## D3D12
        ///
        /// - `NO_ACCESS`
        ///
        const NONE = 0x0;

        ///
        /// ## Vulkan
        ///
        /// - `VERTEX_ATTRIBUTE_READ_BIT`
        ///
        /// ## D3D12
        ///
        /// - `VERTEX_BUFFER`
        ///
        const VERTEX_BUFFER_READ = 0x1;

        ///
        /// ## Vulkan
        ///
        /// - `INDEX_READ_BIT`
        ///
        /// ## D3D12
        ///
        /// - `INDEX_BUFFER`
        ///
        const INDEX_BUFFER_READ = 0x2;

        ///
        /// ## Vulkan
        ///
        /// - `UNIFORM_READ_BIT`
        ///
        /// ## D3D12
        ///
        /// - `CONSTANT_BUFFER`
        ///
        const CONSTANT_BUFFER_READ = 0x4;

        ///
        /// ## Vulkan
        ///
        /// - `INDIRECT_COMMAND_READ_BIT`
        ///
        /// ## D3D12
        ///
        /// - `INDIRECT_ARGUMENT`
        ///
        const INDIRECT_COMMAND_READ = 0x8;

        ///
        /// ## Vulkan
        ///
        /// - `COLOR_ATTACHMENT_READ_BIT`
        ///
        /// ## D3D12
        ///
        /// - `RENDER_TARGET`
        ///
        const RENDER_TARGET_READ = 0x10;

        ///
        /// ## Vulkan
        ///
        /// - `COLOR_ATTACHMENT_WRITE_BIT`
        ///
        /// ## D3D12
        ///
        /// - `RENDER_TARGET`
        ///
        const RENDER_TARGET_WRITE = 0x20;

        ///
        /// ## Vulkan
        ///
        /// - `DEPTH_STENCIL_ATTACHMENT_READ_BIT`
        ///
        /// ## D3D12
        ///
        /// - `DEPTH_STENCIL_READ`
        ///
        const DEPTH_STENCIL_READ = 0x40;

        ///
        /// ## Vulkan
        ///
        /// - `DEPTH_STENCIL_ATTACHMENT_WRITE_BIT`
        ///
        /// ## D3D12
        ///
        /// - `DEPTH_STENCIL_WRITE`
        ///
        const DEPTH_STENCIL_WRITE = 0x80;

        ///
        /// ## Vulkan
        ///
        /// - `TRANSFER_READ_BIT`
        ///
        /// ## D3D12
        ///
        /// - `COPY_SOURCE`
        ///
        const COPY_READ = 0x100;

        ///
        /// ## Vulkan
        ///
        /// - `TRANSFER_WRITE_BIT`
        ///
        /// ## D3D12
        ///
        /// - `COPY_DEST`
        ///
        const COPY_WRITE = 0x200;

        ///
        /// ## Vulkan
        ///
        /// - `ACCELERATION_STRUCTURE_READ_BIT`
        ///
        /// ## D3D12
        ///
        /// - `RAYTRACING_ACCELERATION_STRUCTURE_READ`
        ///
        const RAYTRACING_ACCELERATION_STRUCTURE_READ = 0x400;

        ///
        /// ## Vulkan
        ///
        /// - `ACCELERATION_STRUCTURE_WRITE_BIT`
        ///
        /// ## D3D12
        ///
        /// - `RAYTRACING_ACCELERATION_STRUCTURE_WRITE`
        ///
        const RAYTRACING_ACCELERATION_STRUCTURE_WRITE = 0x800;

        ///
        /// ## Vulkan
        ///
        /// - `SHADER_READ_BIT`
        ///
        /// ## D3D12
        ///
        /// - `SHADER_RESOURCE`
        ///
        const SHADER_READ = 0x1000;

        ///
        /// ## Vulkan
        ///
        /// - `SHADER_WRITE_BIT`
        ///
        /// ## D3D12
        ///
        /// - `UNORDERED_ACCESS`
        ///
        const SHADER_WRITE = 0x2000;
    }
}

impl Default for BarrierAccess {
    #[inline(always)]
    fn default() -> Self {
        Self::NONE
    }
}
