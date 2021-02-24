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

use crate::renderer::egui::constant::ConstantObjects;
use crate::renderer::egui::global::GlobalObjects;
use std::ffi::CString;
use std::sync::Arc;

pub struct PerFrameFontObjects {
    pub staging_buffer: dx12::Resource,

    pub staged_pool: Arc<Pool>,
    pub staged_image: dx12::Resource,
    pub staged_view: ImageView,
    pub staged_size: (u32, u32),
    pub staged_hash: u64,

    pub descriptor_set: DescriptorSet,
}

impl PerFrameFontObjects {
    pub fn new(
        device: &dx12::Device,
        constant: &ConstantObjects,
        allocator: &Arc<Allocator>,
    ) -> Self {
        unsafe {
            let dimensions = (4096, 4096);
            let staging_buffer = Self::create_staging_buffer(allocator, dimensions);
            let staged_pool = Self::create_staged_pool(allocator, dimensions);

            let layout = constant.pipeline_layout.set_layouts()[0];
            let pool = constant.descriptor_pool;
            let descriptor_set = Self::allocate_descriptor_set(device, layout, pool);

            Self {
                staging_buffer,
                staged_pool,
                staged_image: (Default::default(), Default::default()),
                staged_view: Default::default(),
                staged_size: (0, 0),
                staged_hash: 0,
                descriptor_set,
            }
        }
    }

    pub unsafe fn upload_texture(
        &mut self,
        device: &dx12::Device,
        allocator: &Allocator,
        command_buffer: CommandBuffer,
        global: &GlobalObjects,
        texture: &egui::Texture,
    ) {
        debug_assert_eq!(texture.pixels.len(), texture.width * texture.height);

        let dimensions = (texture.width as u32, texture.height as u32);
        if self.requires_new_image(dimensions) && texture.version != self.staged_hash {
            self.free_staged_resources(device, allocator);
            self.create_staged_resources(device, allocator, dimensions);
            self.update_descriptor_set(device, global);
            self.staged_size = dimensions;
            self.staged_hash = texture.version
        }

        let ptr = allocator
            .map_memory(&self.staging_buffer.1)
            .expect("Failed to map font upload memory");
        ptr.copy_from_nonoverlapping(texture.pixels.as_ptr(), texture.pixels.len());
        allocator.unmap_memory(&self.staging_buffer.1);

        self.record_buffer_staging(device, command_buffer, dimensions);
    }

    unsafe fn record_buffer_staging(
        &self,
        device: &dx12::Device,
        command_buffer: CommandBuffer,
        dimensions: (u32, u32),
    ) {
        //
        // Transition the image from raw state to the state needed for copying into the image
        //
        let memory_barriers = [];
        let buffer_memory_barriers = [];
        let subresource_range = ImageSubresourceRangeBuilder::new()
            .aspect_mask(ImageAspectFlags::COLOR)
            .level_count(1)
            .layer_count(1)
            .base_mip_level(0)
            .base_array_layer(0);
        let image_memory_barriers = [ImageMemoryBarrierBuilder::new()
            .image(self.staged_image.0)
            .subresource_range(*subresource_range)
            .src_access_mask(AccessFlags::default())
            .dst_access_mask(AccessFlags::TRANSFER_WRITE)
            .old_layout(ImageLayout::UNDEFINED)
            .new_layout(ImageLayout::TRANSFER_DST_OPTIMAL)];
        device.loader().cmd_pipeline_barrier(
            command_buffer,
            PipelineStageFlags::HOST,
            PipelineStageFlags::TRANSFER,
            None,
            &memory_barriers,
            &buffer_memory_barriers,
            &image_memory_barriers,
        );

        //
        // Copy from the upload buffer into the image
        //
        let subresource = ImageSubresourceLayersBuilder::new()
            .aspect_mask(ImageAspectFlags::COLOR)
            .base_array_layer(0)
            .layer_count(1)
            .mip_level(0);
        let mut extent = Extent3D::default();
        extent.width = dimensions.0;
        extent.height = dimensions.1;
        extent.depth = 1;
        let region = BufferImageCopyBuilder::new()
            .image_subresource(*subresource)
            .image_extent(extent);
        let regions = [region];
        device.loader().cmd_copy_buffer_to_image(
            command_buffer,
            self.staging_buffer.0,
            self.staged_image.0,
            ImageLayout::TRANSFER_DST_OPTIMAL,
            &regions,
        );

        //
        // Transition the image into a state that can be used for being sampled from
        //
        let memory_barriers = [];
        let buffer_memory_barriers = [];
        let subresource_range = ImageSubresourceRangeBuilder::new()
            .aspect_mask(ImageAspectFlags::COLOR)
            .level_count(1)
            .layer_count(1)
            .base_mip_level(0)
            .base_array_layer(0);
        let image_barrier = ImageMemoryBarrierBuilder::new()
            .image(self.staged_image.0)
            .subresource_range(*subresource_range)
            .src_access_mask(AccessFlags::TRANSFER_WRITE)
            .dst_access_mask(AccessFlags::SHADER_READ)
            .old_layout(ImageLayout::TRANSFER_DST_OPTIMAL)
            .new_layout(ImageLayout::SHADER_READ_ONLY_OPTIMAL);
        let image_memory_barriers = [image_barrier];
        device.loader().cmd_pipeline_barrier(
            command_buffer,
            PipelineStageFlags::TRANSFER,
            PipelineStageFlags::ALL_GRAPHICS,
            None,
            &memory_barriers,
            &buffer_memory_barriers,
            &image_memory_barriers,
        );
    }

    unsafe fn update_descriptor_set(
        &self,
        device: &aleph_vulkan_core::Device,
        global: &GlobalObjects,
    ) {
        let sampled_image_info = [DescriptorImageInfoBuilder::new()
            .image_view(self.staged_view)
            .image_layout(ImageLayout::SHADER_READ_ONLY_OPTIMAL)];
        let sampled_image = WriteDescriptorSetBuilder::new()
            .descriptor_type(DescriptorType::SAMPLED_IMAGE)
            .dst_set(self.descriptor_set)
            .image_info(&sampled_image_info)
            .dst_binding(0);

        let sampler_info = [DescriptorImageInfoBuilder::new().sampler(global.sampler)];
        let sampler = WriteDescriptorSetBuilder::new()
            .descriptor_type(DescriptorType::SAMPLER)
            .dst_set(self.descriptor_set)
            .image_info(&sampler_info)
            .dst_binding(1);
        device
            .loader()
            .update_descriptor_sets(&[sampled_image, sampler], &[])
    }

    unsafe fn create_staged_resources(
        &mut self,
        device: &aleph_vulkan_core::Device,
        allocator: &Allocator,
        dimensions: (u32, u32),
    ) {
        let image_create_info = Self::staged_image_create_info(dimensions);
        let alloc_create_info = Self::staged_image_alloc_info();
        let staged_image = allocator
            .create_image(&image_create_info, &alloc_create_info)
            .expect("Failed to create staged font image");

        let subresource_range = ImageSubresourceRangeBuilder::new()
            .aspect_mask(ImageAspectFlags::COLOR)
            .base_array_layer(0)
            .base_mip_level(0)
            .layer_count(1)
            .level_count(1);
        let create_info = ImageViewCreateInfoBuilder::new()
            .image(staged_image.0)
            .format(Format::R8_UNORM)
            .view_type(ImageViewType::_2D)
            .subresource_range(*subresource_range);
        let staged_view = device
            .loader()
            .create_image_view(&create_info, None, None)
            .expect("Failed to create image view");
        staged_view.add_debug_name(
            device,
            aleph_macros::cstr!(concat!(module_path!(), "::FontView")),
        );

        self.staged_image = staged_image;
        self.staged_view = staged_view;
    }

    unsafe fn free_staged_resources(
        &mut self,
        device: &aleph_vulkan_core::Device,
        allocator: &Allocator,
    ) {
        device
            .loader()
            .destroy_image_view(Some(self.staged_view), None);
        allocator.destroy_image(self.staged_image.0, self.staged_image.1);
    }

    fn staged_image_create_info(dimensions: (u32, u32)) -> ImageCreateInfoBuilder<'static> {
        ImageCreateInfoBuilder::new()
            .format(Format::R8_UNORM)
            .initial_layout(ImageLayout::UNDEFINED)
            .samples(SampleCountFlagBits::_1)
            .tiling(ImageTiling::OPTIMAL)
            .usage(ImageUsageFlags::SAMPLED | ImageUsageFlags::TRANSFER_DST)
            .sharing_mode(SharingMode::EXCLUSIVE)
            .image_type(ImageType::_2D)
            .mip_levels(1)
            .array_layers(1)
            .extent(Extent3D {
                width: dimensions.0,
                height: dimensions.1,
                depth: 1,
            })
    }

    fn staged_image_alloc_info() -> AllocationCreateInfoBuilder {
        AllocationCreateInfoBuilder::new().usage(MemoryUsage::GPUOnly)
    }

    unsafe fn create_staging_buffer(
        allocator: &Arc<Allocator>,
        dimensions: (u32, u32),
    ) -> (Buffer, Allocation) {
        let size = dimensions.0 * dimensions.1;

        let buffer_create_info = BufferCreateInfoBuilder::new()
            .usage(BufferUsageFlags::TRANSFER_SRC)
            .sharing_mode(SharingMode::EXCLUSIVE)
            .size(size as _);
        let alloc_create_info = AllocationCreateInfoBuilder::new()
            .usage(MemoryUsage::CPUOnly)
            .build();

        allocator
            .create_buffer(&buffer_create_info, &alloc_create_info)
            .expect("Failed to create staging buffer")
    }

    unsafe fn create_staged_pool(allocator: &Arc<Allocator>, dimensions: (u32, u32)) -> Arc<Pool> {
        let size = dimensions.0 * dimensions.1;

        let memory_type_index = {
            let image_create_info = Self::staged_image_create_info(dimensions);
            let allocation_create_info = AllocationCreateInfoBuilder::new()
                .usage(MemoryUsage::GPUOnly)
                .build();
            allocator
                .find_memory_type_index_for_image_info(&image_create_info, &allocation_create_info)
                .expect("Failed to find memory index")
        };

        aleph_vulkan_alloc::PoolBuilder::new()
            .flags(aleph_vulkan_alloc::PoolCreateFlag::LINEAR_ALGORITHM_BIT)
            .block_size(size as _)
            .memory_type_index(memory_type_index)
            .min_block_count(1)
            .max_block_count(1)
            .frame_in_use_count(2)
            .build(allocator)
            .expect("Failed to create memory pool")
    }

    unsafe fn allocate_descriptor_set(
        device: &aleph_vulkan_core::Device,
        layout: DescriptorSetLayout,
        pool: DescriptorPool,
    ) -> DescriptorSet {
        let set_layouts = [layout];
        let allocate_info = DescriptorSetAllocateInfoBuilder::new()
            .descriptor_pool(pool)
            .set_layouts(&set_layouts);

        let descriptor_set = device
            .loader()
            .allocate_descriptor_sets(&allocate_info)
            .expect("Failed to allocate descriptor sets")[0];

        let name = format!("{}::DescriptorSet", module_path!());
        let name = CString::new(name).unwrap();
        descriptor_set.add_debug_name(device, &name);

        descriptor_set
    }

    fn requires_new_image(&self, dimensions: (u32, u32)) -> bool {
        dimensions.0 != self.staged_size.0 || dimensions.1 != self.staged_size.1
    }
}

///
/// This represents the resources needed for rendering a single egui frame in parallel. This
/// separation allows for multiple frames in flight
///
pub struct PerFrameObjects {
    pub swap_image: SwapImage,
    pub framebuffer: Framebuffer,
    pub vtx_buffer: (Buffer, Allocation),
    pub idx_buffer: (Buffer, Allocation),
    pub font_objects: PerFrameFontObjects,
}

impl PerFrameObjects {
    pub fn init(
        device: &aleph_vulkan_core::Device,
        allocator: &Arc<Allocator>,
        constant: &ConstantObjects,
        swapchain: &aleph_vulkan_core::Swapchain,
        index: usize,
        render_pass: RenderPass,
    ) -> Self {
        let swap_image = swapchain.images()[index].clone();
        let framebuffer = Self::create_framebuffer(device, render_pass, &swap_image, index);

        let vtx_buffer = unsafe {
            let buffer_create_info = BufferCreateInfoBuilder::new()
                .usage(BufferUsageFlags::VERTEX_BUFFER)
                .sharing_mode(SharingMode::EXCLUSIVE)
                .size(Self::vertex_buffer_size() as _);
            let alloc_create_info = AllocationCreateInfoBuilder::new()
                .usage(MemoryUsage::CPUToGPU)
                .build();
            allocator
                .create_buffer(&buffer_create_info, &alloc_create_info)
                .expect("Failed to create vertex buffer")
        };

        let idx_buffer = unsafe {
            let buffer_create_info = BufferCreateInfoBuilder::new()
                .usage(BufferUsageFlags::INDEX_BUFFER)
                .sharing_mode(SharingMode::EXCLUSIVE)
                .size(Self::index_buffer_size() as _);
            let allocation_create_info = AllocationCreateInfoBuilder::new()
                .usage(MemoryUsage::CPUToGPU)
                .build();
            allocator
                .create_buffer(&buffer_create_info, &allocation_create_info)
                .expect("Failed to create index buffer")
        };

        let font_objects = PerFrameFontObjects::new(device, constant, allocator);

        Self {
            swap_image,
            framebuffer,
            vtx_buffer,
            idx_buffer,
            font_objects,
        }
    }

    pub fn create_framebuffer(
        device: &aleph_vulkan_core::Device,
        render_pass: RenderPass,
        swap_image: &SwapImage,
        index: usize,
    ) -> Framebuffer {
        let attachments = [swap_image.image_view()];
        let create_info = FramebufferCreateInfoBuilder::new()
            .render_pass(render_pass)
            .width(swap_image.width())
            .height(swap_image.height())
            .attachments(&attachments)
            .layers(1);
        unsafe {
            let framebuffer = device
                .loader()
                .create_framebuffer(&create_info, None, None)
                .expect("Failed to create framebuffer");

            let name = format!("{}::{}::FrameBuffer", module_path!(), index);
            let name = CString::new(name).unwrap();
            framebuffer.add_debug_name(device, &name);

            framebuffer
        }
    }

    pub fn vertex_buffer_size() -> usize {
        1024 * 1024 * 4
    }

    pub fn index_buffer_size() -> usize {
        1024 * 1024 * 2
    }
}
