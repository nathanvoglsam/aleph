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

pub trait ResourceUsageFlagsExt {
    fn image_layout(&self, read_only: bool, format: Format) -> ImageLayout;
    fn default_barrier_sync(&self, read_only: bool, format: Format) -> BarrierSync;
    fn barrier_access_for_read(&self, format: Format) -> BarrierAccess;
    fn barrier_access_for_write(&self, format: Format) -> BarrierAccess;
    fn is_valid_texture_usage(&self) -> bool;
}

impl ResourceUsageFlagsExt for ResourceUsageFlags {
    /// Gets the image layout that is compatible with the specified access flags
    fn image_layout(&self, read_only: bool, format: Format) -> ImageLayout {
        debug_assert!(
            self.is_valid_texture_usage(),
            "{:?} is not valid texture usage",
            *self
        );

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

    fn default_barrier_sync(&self, read_only: bool, format: Format) -> BarrierSync {
        let mut sync = BarrierSync::NONE;
        if self.contains(Self::COPY_SOURCE) {
            sync |= BarrierSync::COPY;
        }
        if self.contains(Self::COPY_DEST) {
            sync |= BarrierSync::COPY;
        }
        if self.contains(Self::VERTEX_BUFFER) {
            sync |= BarrierSync::VERTEX_SHADING;
        }
        if self.contains(Self::INDEX_BUFFER) {
            sync |= BarrierSync::INDEX_INPUT;
        }
        if self.contains(Self::CONSTANT_BUFFER) {
            sync |= BarrierSync::PIXEL_SHADING
                | BarrierSync::VERTEX_SHADING
                | BarrierSync::RAYTRACING
                | BarrierSync::COMPUTE_SHADING;
        }
        if self.contains(Self::SHADER_RESOURCE) {
            sync |= BarrierSync::PIXEL_SHADING
                | BarrierSync::VERTEX_SHADING
                | BarrierSync::RAYTRACING
                | BarrierSync::COMPUTE_SHADING;
        }
        if self.contains(Self::UNORDERED_ACCESS) {
            sync |= BarrierSync::PIXEL_SHADING
                | BarrierSync::VERTEX_SHADING
                | BarrierSync::RAYTRACING
                | BarrierSync::COMPUTE_SHADING;
        }
        if self.contains(Self::INDIRECT_DRAW_ARGS) {
            sync |= BarrierSync::EXECUTE_INDIRECT;
        }
        if self.contains(Self::ACCELERATION_STRUCTURE_BUILD_INPUT) {
            sync |= BarrierSync::BUILD_RAYTRACING_ACCELERATION_STRUCTURE;
        }
        if self.contains(Self::ACCELERATION_STRUCTURE_STORAGE) {
            sync |= BarrierSync::RAYTRACING;
            if !read_only {
                sync |= BarrierSync::BUILD_RAYTRACING_ACCELERATION_STRUCTURE;
            }
        }
        if self.contains(Self::RENDER_TARGET) {
            if format.is_depth_stencil() {
                sync |= BarrierSync::DEPTH_STENCIL;
            } else {
                sync |= BarrierSync::RENDER_TARGET;
            }
        }
        sync
    }

    fn barrier_access_for_read(&self, format: Format) -> BarrierAccess {
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
            if format.is_depth_stencil() {
                out |= BarrierAccess::DEPTH_STENCIL_READ
            } else {
                out |= BarrierAccess::RENDER_TARGET_READ
            }
        }
        out
    }

    fn barrier_access_for_write(&self, format: Format) -> BarrierAccess {
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
            if format.is_depth_stencil() {
                out |= BarrierAccess::DEPTH_STENCIL_WRITE
            } else {
                out |= BarrierAccess::RENDER_TARGET_WRITE
            }
        }
        out
    }

    fn is_valid_texture_usage(&self) -> bool {
        if !ResourceUsageFlags::TEXTURE_USAGE_MASK.contains(*self) {
            return false;
        }

        // We need to filter out any non-synchronizing access flags to allow this debug check
        // to make sense. We need to check that only a single usage has been specified as an
        // image within a pass can only be in a single image layout.
        let access_flags = *self & (!Self::CUBE_FACE);
        if access_flags.bits().count_ones() > 1 {
            return false;
        }

        true
    }
}
