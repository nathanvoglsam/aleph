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

bitflags::bitflags! {
    /// Bitflags that specifies how a resource (texture OR buffer) is used, with semantics specific
    /// to frame graph.
    ///
    /// These flags specify how a resource is used within a frame graph pass but in a more specific
    /// and explicit way that also allows the frame graph to determine the set of usage flags that
    /// are needed for the resource.
    ///
    /// # Background
    ///
    /// [BufferUsageFlags] and [TextureUsageFlags] are not specific enough to be used for
    /// synchronization on their own as they don't include all usages for a texture as far as
    /// synchronization is concerned. These flags were constructed in a way that allows deriving
    /// valid barriers state ([BarrierAccess], [ImageLayout]) as well as usage flags for
    /// constructing transient resources. This includes needed non-synchronizing
    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
    pub struct ResourceAccessFlags: u32 {
        /// The default, empty bitflags.
        const NONE = 0x0;

        /// Using the resource as a copy source in the current pass.
        ///
        /// This access is valid for buffers and textures.
        const COPY_SOURCE = 0x1;

        /// Using the resource as a copy dest in the current pass.
        ///
        /// This access is valid for buffers and textures.
        const COPY_DEST = 0x2;

        /// Using the resource as a vertex buffer within the current pass.
        ///
        /// This access is valid only for buffers.
        const VERTEX_BUFFER = 0x4;

        /// Using the resource as an index buffer within the current pass.
        ///
        /// This access is valid only for buffers.
        const INDEX_BUFFER = 0x8;

        /// Using the resource as a constant buffer within the current pass.
        ///
        /// This access is valid only for buffers.
        const CONSTANT_BUFFER = 0x10;

        /// Using the resource as the source for indirect craw args within the current pass.
        ///
        /// This access is valid only for buffers.
        const INDIRECT_DRAW_ARGS = 0x20;

        /// Using the resource as the build input for acceleration structure construction within the
        /// current pass.
        ///
        /// This access is valid only for buffers.
        const ACCELERATION_STRUCTURE_BUILD_INPUT = 0x40;

        /// Using the resource as an acceleration structure within the current pass. This can imply
        /// both read and write access.
        ///
        /// This access is valid only for buffers.
        const ACCELERATION_STRUCTURE_STORAGE = 0x80;

        /// Using the resource as a general read-only shader resource within the current pass. This
        /// will not allow or imply more specific usage for buffers. Using the more specific access
        /// flags is required for usage as a constant/index/vertex/texel buffer.
        ///
        /// This access is valid for buffers and textures.
        const SHADER_RESOURCE = 0x100;

        /// Using the resource for unordered access within the current pass. This will always imply
        /// writable access.
        ///
        /// This access is valid for buffers and textures.
        const UNORDERED_ACCESS = 0x200;

        /// Using the resource as a render target or 'attachment' within the current pass. This can
        /// be a read-only or read-write attachment.
        ///
        /// This access is valid only for textures.
        const RENDER_TARGET = 0x400;

        /// Flags the resource will be used as a cube map face. This access doesn't imply any
        /// synchronization, it is only used for flagging a feature that must be explicitly enabled
        /// for an image.
        ///
        /// This access is valid only for textures.
        const CUBE_FACE = 0x800;

        /// A mask of all the access flags valid to use on buffers
        const BUFFER_ACCESS_MASK =
            Self::COPY_SOURCE.bits()
            | Self::COPY_DEST.bits()
            | Self::VERTEX_BUFFER.bits()
            | Self::INDEX_BUFFER.bits()
            | Self::CONSTANT_BUFFER.bits()
            | Self::INDIRECT_DRAW_ARGS.bits()
            | Self::ACCELERATION_STRUCTURE_BUILD_INPUT.bits()
            | Self::ACCELERATION_STRUCTURE_STORAGE.bits()
            | Self::SHADER_RESOURCE.bits()
            | Self::UNORDERED_ACCESS.bits();

        /// A mask of all the access flags valid to use on textures
        const TEXTURE_ACCESS_MASK =
            Self::COPY_SOURCE.bits()
            | Self::COPY_DEST.bits()
            | Self::SHADER_RESOURCE.bits()
            | Self::UNORDERED_ACCESS.bits()
            | Self::RENDER_TARGET.bits()
            | Self::CUBE_FACE.bits();

        /// Mask that represents all read access flags.
        const READ_ACCESS_MASK =
            Self::COPY_SOURCE.bits()
            | Self::VERTEX_BUFFER.bits()
            | Self::INDEX_BUFFER.bits()
            | Self::CONSTANT_BUFFER.bits()
            | Self::INDIRECT_DRAW_ARGS.bits()
            | Self::ACCELERATION_STRUCTURE_BUILD_INPUT.bits()
            | Self::ACCELERATION_STRUCTURE_STORAGE.bits()
            | Self::SHADER_RESOURCE.bits()
            | Self::RENDER_TARGET.bits();

        /// Mask that represents all write access flags.
        const WRITE_ACCESS_MASK =
            Self::COPY_DEST.bits()
            | Self::ACCELERATION_STRUCTURE_STORAGE.bits()
            | Self::UNORDERED_ACCESS.bits()
            | Self::RENDER_TARGET.bits();
    }
}

impl Default for ResourceAccessFlags {
    fn default() -> ResourceAccessFlags {
        ResourceAccessFlags::NONE
    }
}

impl ResourceAccessFlags {
    /// Gets the image layout that is compatible with the specified access flags
    pub fn image_layout(&self, read_only: bool, format: Format) -> ImageLayout {
        debug_assert!(Self::TEXTURE_ACCESS_MASK.contains(*self));
        if cfg!(debug_assertions) {
            // We need to filter out any non-synchronizing access flags to allow this debug check
            // to make sense. We need to check that only a single usage has been specified as an
            // image within a pass can only be in a single image layout.
            let no_cube_face = Self::CUBE_FACE.complement();
            let access_flags = self.intersection(no_cube_face);
            debug_assert!(
                access_flags.bits().count_ones() <= 1,
                "Only a single synchronizing access flag can be specified for images"
            );
        }
        if self.contains(Self::COPY_SOURCE) {
            return ImageLayout::CopySrc;
        }
        if self.contains(Self::COPY_DEST) {
            return ImageLayout::CopyDst;
        }
        if self.contains(Self::SHADER_RESOURCE) {
            return ImageLayout::ShaderReadOnly;
        }
        if self.contains(Self::UNORDERED_ACCESS) {
            return ImageLayout::UnorderedAccess;
        }
        if self.contains(Self::RENDER_TARGET) {
            return if format.is_depth_stencil() {
                if read_only {
                    ImageLayout::DepthStencilReadOnly
                } else {
                    ImageLayout::DepthStencilAttachment
                }
            } else {
                ImageLayout::ColorAttachment
            };
        }
        ImageLayout::Undefined
    }

    pub fn buffer_usage_flags(&self) -> BufferUsageFlags {
        debug_assert!(Self::BUFFER_ACCESS_MASK.contains(*self));
        let mut out = BufferUsageFlags::NONE;
        if self.contains(Self::COPY_SOURCE) {
            out |= BufferUsageFlags::COPY_SOURCE;
        }
        if self.contains(Self::COPY_DEST) {
            out |= BufferUsageFlags::COPY_DEST;
        }
        if self.contains(Self::VERTEX_BUFFER) {
            out |= BufferUsageFlags::VERTEX_BUFFER;
        }
        if self.contains(Self::INDEX_BUFFER) {
            out |= BufferUsageFlags::INDEX_BUFFER;
        }
        if self.contains(Self::CONSTANT_BUFFER) {
            out |= BufferUsageFlags::CONSTANT_BUFFER;
        }
        if self.contains(Self::SHADER_RESOURCE) {
            out |= BufferUsageFlags::SHADER_RESOURCE;
        }
        if self.contains(Self::INDIRECT_DRAW_ARGS) {
            out |= BufferUsageFlags::INDIRECT_DRAW_ARGS;
        }
        if self.contains(Self::ACCELERATION_STRUCTURE_BUILD_INPUT) {
            out |= BufferUsageFlags::ACCELERATION_STRUCTURE_BUILD_INPUT;
        }
        if self.contains(Self::ACCELERATION_STRUCTURE_STORAGE) {
            out |= BufferUsageFlags::ACCELERATION_STRUCTURE_STORAGE;
        }
        if self.contains(Self::UNORDERED_ACCESS) {
            out |= BufferUsageFlags::UNORDERED_ACCESS;
        }
        out
    }

    pub fn texture_usage_flags(&self) -> TextureUsageFlags {
        debug_assert!(Self::TEXTURE_ACCESS_MASK.contains(*self));
        let mut out = TextureUsageFlags::NONE;
        if self.contains(Self::COPY_SOURCE) {
            out |= TextureUsageFlags::COPY_SOURCE
        }
        if self.contains(Self::COPY_DEST) {
            out |= TextureUsageFlags::COPY_DEST
        }
        if self.contains(Self::SHADER_RESOURCE) {
            out |= TextureUsageFlags::SHADER_RESOURCE
        }
        if self.contains(Self::UNORDERED_ACCESS) {
            out |= TextureUsageFlags::UNORDERED_ACCESS
        }
        if self.contains(Self::RENDER_TARGET) {
            out |= TextureUsageFlags::RENDER_TARGET
        }
        if self.contains(Self::CUBE_FACE) {
            out |= TextureUsageFlags::CUBE_FACE
        }
        out
    }

    pub fn barrier_access_for_read(&self) -> BarrierAccess {
        debug_assert!(Self::READ_ACCESS_MASK.contains(*self));

        let mut out = BarrierAccess::NONE;
        if self.contains(Self::COPY_SOURCE) {
            out |= BarrierAccess::COPY_READ
        }
        if self.contains(Self::VERTEX_BUFFER) {
            out |= BarrierAccess::VERTEX_BUFFER_READ
        }
        if self.contains(Self::INDEX_BUFFER) {
            out |= BarrierAccess::INDEX_BUFFER_READ
        }
        if self.contains(Self::CONSTANT_BUFFER) {
            out |= BarrierAccess::CONSTANT_BUFFER_READ
        }
        if self.contains(Self::INDIRECT_DRAW_ARGS) {
            out |= BarrierAccess::INDIRECT_COMMAND_READ
        }
        if self.contains(Self::ACCELERATION_STRUCTURE_BUILD_INPUT) {
            out |= BarrierAccess::RAYTRACING_ACCELERATION_STRUCTURE_READ
        }
        if self.contains(Self::ACCELERATION_STRUCTURE_STORAGE) {
            out |= BarrierAccess::RAYTRACING_ACCELERATION_STRUCTURE_READ
        }
        if self.contains(Self::SHADER_RESOURCE) {
            out |= BarrierAccess::SHADER_READ
        }
        if self.contains(Self::RENDER_TARGET) {
            out |= BarrierAccess::RENDER_TARGET_READ
        }
        out
    }

    pub fn barrier_access_for_write(&self) -> BarrierAccess {
        debug_assert!(Self::WRITE_ACCESS_MASK.contains(*self));

        let mut out = BarrierAccess::NONE;
        if self.contains(Self::COPY_DEST) {
            out |= BarrierAccess::COPY_WRITE
        }
        if self.contains(Self::ACCELERATION_STRUCTURE_STORAGE) {
            out |= BarrierAccess::RAYTRACING_ACCELERATION_STRUCTURE_WRITE
        }
        if self.contains(Self::UNORDERED_ACCESS) {
            out |= BarrierAccess::SHADER_WRITE
        }
        if self.contains(Self::RENDER_TARGET) {
            out |= BarrierAccess::RENDER_TARGET_WRITE
        }
        out
    }
}
