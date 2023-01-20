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

use crate::command_list::CommandList;
use crate::texture::Texture;
use interfaces::any::QueryInterface;
use interfaces::gpu::{
    BeginRenderingInfo, BufferBarrier, BufferCopyRegion, BufferToTextureCopyRegion, Color,
    DescriptorSetHandle, Format, GlobalBarrier, IBuffer, IComputeEncoder, IGeneralEncoder,
    IGraphicsPipeline, IPipelineLayout, ITexture, ITransferEncoder, ImageLayout, IndexType,
    InputAssemblyBufferBinding, PipelineBindPoint, Rect, TextureAspect, TextureBarrier,
    TextureDesc, TextureSubResourceSet, Viewport,
};

pub struct Encoder<'a> {
    pub(crate) _parent: &'a mut CommandList,
    pub(crate) inner: Box<dyn IGeneralEncoder>,
}

impl<'a> IGeneralEncoder for Encoder<'a> {
    unsafe fn bind_graphics_pipeline(&mut self, pipeline: &dyn IGraphicsPipeline) {
        self.inner.bind_graphics_pipeline(pipeline)
    }

    unsafe fn bind_vertex_buffers(
        &mut self,
        first_binding: u32,
        bindings: &[InputAssemblyBufferBinding],
    ) {
        self.inner.bind_vertex_buffers(first_binding, bindings)
    }

    unsafe fn bind_index_buffer(
        &mut self,
        index_type: IndexType,
        binding: &InputAssemblyBufferBinding,
    ) {
        self.inner.bind_index_buffer(index_type, binding)
    }

    unsafe fn set_viewports(&mut self, viewports: &[Viewport]) {
        self.inner.set_viewports(viewports)
    }

    unsafe fn set_scissor_rects(&mut self, rects: &[Rect]) {
        self.inner.set_scissor_rects(rects)
    }

    unsafe fn set_push_constant_block(&mut self, block_index: usize, data: &[u8]) {
        Self::validate_push_constant_data_buffer(data, block);
        self.inner.set_push_constant_block(block_index, data)
    }

    unsafe fn begin_rendering(&mut self, info: &BeginRenderingInfo) {
        Self::validate_rendering_attachments(info);
        self.inner.begin_rendering(info)
    }

    unsafe fn end_rendering(&mut self) {
        self.inner.end_rendering();
    }

    unsafe fn draw(
        &mut self,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    ) {
        self.inner
            .draw(vertex_count, instance_count, first_vertex, first_instance)
    }

    unsafe fn draw_indexed(
        &mut self,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        first_instance: u32,
        vertex_offset: i32,
    ) {
        self.inner.draw_indexed(
            index_count,
            instance_count,
            first_index,
            first_instance,
            vertex_offset,
        )
    }
}

impl<'a> IComputeEncoder for Encoder<'a> {
    unsafe fn bind_descriptor_sets(
        &mut self,
        pipeline_layout: &dyn IPipelineLayout,
        bind_point: PipelineBindPoint,
        first_set: u32,
        sets: &[DescriptorSetHandle],
    ) {
        self.inner
            .bind_descriptor_sets(pipeline_layout, bind_point, first_set, sets)
    }

    unsafe fn dispatch(&mut self, group_count_x: u32, group_count_y: u32, group_count_z: u32) {
        self.inner
            .dispatch(group_count_x, group_count_y, group_count_z)
    }
}

impl<'a> ITransferEncoder for Encoder<'a> {
    unsafe fn resource_barrier(
        &mut self,
        global_barriers: &[GlobalBarrier],
        buffer_barriers: &[BufferBarrier],
        texture_barriers: &[TextureBarrier],
    ) {
        texture_barriers.iter().for_each(|v| {
            Self::validate_sub_resource_range_against_texture(
                v.texture.desc(),
                &v.subresource_range,
            );
        });
        self.inner
            .resource_barrier(global_barriers, buffer_barriers, texture_barriers)
    }

    unsafe fn copy_buffer_regions(
        &mut self,
        src: &dyn IBuffer,
        dst: &dyn IBuffer,
        regions: &[BufferCopyRegion],
    ) {
        self.inner.copy_buffer_regions(src, dst, regions)
    }

    unsafe fn copy_buffer_to_texture(
        &mut self,
        src: &dyn IBuffer,
        dst: &dyn ITexture,
        dst_layout: ImageLayout,
        regions: &[BufferToTextureCopyRegion],
    ) {
        self.inner.copy_buffer_to_texture(src, dst, dst_layout, regions);
    }

    unsafe fn set_marker(&mut self, color: Color, message: &str) {
        self.inner.set_marker(color, message)
    }

    unsafe fn begin_event(&mut self, color: Color, message: &str) {
        self.inner.begin_event(color, message)
    }

    unsafe fn end_event(&mut self) {
        self.inner.end_event()
    }
}

impl<'a> Encoder<'a> {
    fn validate_aspect_against_texture_format(format: Format, aspect: &TextureAspect) {
        if aspect.contains(TextureAspect::COLOR) {
            debug_assert!(
                !format.is_depth_stencil(),
                "Texture of format {} has no 'Color' aspect",
                format
            );
        } else if aspect.contains(TextureAspect::DEPTH_STENCIL) {
            debug_assert!(
                format.is_depth() && format.is_stencil(),
                "Texture of format {} lacks both 'Depth' and 'Stencil' aspect",
                format
            );
        } else if aspect.intersects(TextureAspect::DEPTH_STENCIL) {
            debug_assert!(
                format.is_depth_stencil(),
                "Texture of format {} has no 'Depth' or 'Stencil' aspect",
                format
            );
        }
    }

    fn validate_buffer_to_texture_copy_buffer_layout(
        region: &BufferToTextureCopyRegion,
        footprint: &mut D3D12_PLACED_SUBRESOURCE_FOOTPRINT,
    ) {
        debug_assert!(region.src.extent.width > 0, "extent.width must be > 0");
        debug_assert!(region.src.extent.height > 0, "extent.height must be > 0");
        debug_assert!(region.src.extent.depth > 0, "extent.depth must be > 0");
        debug_assert!(
            footprint.Footprint.RowPitch % 256 == 0,
            "RowPitch must be a multiple of 256"
        );
    }

    fn validate_buffer_to_texture_copy_dest_region(
        dst: &Texture,
        format: Format,
        region: &&BufferToTextureCopyRegion,
        index: Option<u32>,
    ) {
        let dst_maximum = region.dst.origin.maximum_with_extent(&region.dst.extent);
        debug_assert!(
            dst_maximum.x <= dst.desc().width,
            "Destination region must not exceed destination width"
        );
        debug_assert!(
            dst_maximum.y <= dst.desc().height,
            "Destination region must not exceed destination height"
        );
        debug_assert!(
            dst_maximum.z <= dst.desc().depth,
            "Destination region must not exceed destination depth"
        );
        debug_assert!(
            index.is_some(),
            "Invalid format ({:#?}) and image aspect ({:#?}) combination",
            format,
            region.dst.aspect
        );
    }

    fn validate_push_constant_data_buffer(data: &[u8], block: &PushConstantBlockInfo) {
        debug_assert!(
            data.len() % 4 == 0,
            "Push Constant data must have len divisible by 4"
        );

        debug_assert!(
            data.len() <= block.size as usize,
            "Push Constant data larger than the specified block"
        );
    }

    fn validate_sub_resource_range_against_texture(
        desc: &TextureDesc,
        set: &TextureSubResourceSet,
    ) {
        Self::validate_aspect_against_texture_format(desc.format, &set.aspect);
        debug_assert!(!set.aspect.is_empty(), "Specified an empty aspect mask");
        Self::validate_sub_resource_mips_and_slices_against_texture(desc, set);
    }

    fn validate_sub_resource_mips_and_slices_against_texture(
        desc: &TextureDesc,
        set: &TextureSubResourceSet,
    ) {
        debug_assert!(
            desc.array_size >= set.num_array_slices,
            "Specified access to more array slices than a texture has"
        );
        debug_assert!(
            desc.mip_levels >= set.num_mip_levels,
            "Specified access to more mip levels than a texture has"
        );
        debug_assert!(
            desc.array_size > set.base_array_slice,
            "Specified access to texture array outside of array bounds"
        );
        debug_assert!(
            desc.mip_levels > set.base_mip_level,
            "Specified access to mip levels outside of mip level bounds"
        );
        debug_assert!(
            desc.array_size >= set.base_array_slice + set.num_array_slices,
            "Specified access to texture array outside of array bounds"
        );
        debug_assert!(
            desc.mip_levels >= set.base_mip_level + set.num_mip_levels,
            "Specified access to mip levels outside of mip level bounds"
        );
    }

    fn validate_rendering_attachments(info: &BeginRenderingInfo) {
        debug_assert!(
            !info.color_attachments.is_empty() || info.depth_stencil_attachment.is_some(),
            "Trying to begin rendering rendering without specifying any attachments"
        );

        info.color_attachments.iter().for_each(|v| {
            let image = v
                .image
                .query_interface::<Texture>()
                .expect("Unknown ITexture implementation");
            debug_assert!(
                image.desc().is_render_target,
                "Used texture as render target when created with 'is_render_target = false'"
            );
            debug_assert!(
                !image.desc().format.is_depth_stencil(),
                "Used depth/stencil texture as a color attachment",
            );
            Self::validate_sub_resource_mips_and_slices_against_texture(
                image.desc(),
                &TextureSubResourceSet {
                    aspect: Default::default(),
                    base_mip_level: v.mip_level,
                    num_mip_levels: 1,
                    base_array_slice: v.base_array_slice,
                    num_array_slices: v.num_array_slices,
                },
            );
        });

        // Produce an iterator over all the (width,height) pairs for each color attachment
        let attachment_sizes = info.color_attachments.iter().map(|v| {
            let image = v
                .image
                .query_interface::<Texture>()
                .expect("Unknown ITexture implementation");
            (image.desc().width, image.desc().height)
        });

        // Reduce the sizes to a single item, asserting that they are all equal
        let attachment_size =
            attachment_sizes.reduce(|(a_width, a_height), (b_width, b_height)| {
                debug_assert_eq!(a_width, b_width, "All attachment widths must be equal");
                debug_assert_eq!(a_height, b_height, "All attachment heights must be equal");
                (a_width, a_height)
            });

        if let Some(v) = info.depth_stencil_attachment {
            let image = v
                .image
                .query_interface::<Texture>()
                .expect("Unknown ITexture implementation");

            debug_assert!(
                image.desc().is_render_target,
                "Used texture as depth/stencil target when created with 'is_render_target = false'"
            );

            debug_assert!(
                image.desc().format.is_depth_stencil(),
                "Used non depth/stencil texture as a depth/stencil attachment",
            );

            Self::validate_sub_resource_mips_and_slices_against_texture(
                image.desc(),
                &TextureSubResourceSet {
                    aspect: Default::default(),
                    base_mip_level: v.mip_level,
                    num_mip_levels: 1,
                    base_array_slice: v.base_array_slice,
                    num_array_slices: v.num_array_slices,
                },
            );

            // Check that the depth stencil dimensions match the color dimensions
            if let Some((width, height)) = attachment_size {
                let (d_width, d_height) = (image.desc().width, image.desc().height);
                debug_assert_eq!(width, d_width, "All attachment widths must be equal");
                debug_assert_eq!(height, d_height, "All attachment heights must be equal");
            }
        }
    }
}
