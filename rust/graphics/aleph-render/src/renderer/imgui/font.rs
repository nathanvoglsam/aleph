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

use super::ImguiGlobal;
use imgui::{FontConfig, FontSource};
use aleph_vulkan_alloc::{Allocation, AllocationCreateInfoBuilder, Allocator, MemoryUsage};
use aleph_vulkan_core::erupt::vk1_0::{
    AccessFlags, BufferCreateInfoBuilder, BufferImageCopyBuilder, BufferUsageFlags,
    CommandBufferAllocateInfoBuilder, CommandBufferBeginInfoBuilder, CommandBufferLevel,
    CommandBufferUsageFlags, CommandPoolCreateFlags, CommandPoolCreateInfoBuilder,
    DependencyFlagBits, DescriptorImageInfoBuilder, DescriptorType, Extent3D, Fence, Filter,
    Format, Image, ImageAspectFlags, ImageCreateInfoBuilder, ImageLayout,
    ImageMemoryBarrierBuilder, ImageSubresourceLayersBuilder, ImageSubresourceRangeBuilder,
    ImageTiling, ImageType, ImageUsageFlags, ImageView, ImageViewCreateInfoBuilder, ImageViewType,
    PipelineStageFlags, SampleCountFlagBits, Sampler, SamplerAddressMode, SamplerCreateInfoBuilder,
    SamplerMipmapMode, SharingMode, SubmitInfoBuilder, Vk10DeviceLoaderExt,
    WriteDescriptorSetBuilder,
};
use aleph_vulkan_core::DebugName;

pub struct ImguiFont {
    pub sampler: Sampler,
    pub allocation: Allocation,
    pub dimensions: (u32, u32),
    pub image: Image,
    pub image_view: ImageView,
}

impl ImguiFont {
    pub fn init(
        mut fonts: imgui::FontAtlasRefMut,
        global: &ImguiGlobal,
        device: &aleph_vulkan_core::Device,
        allocator: &Allocator,
    ) -> Self {
        fonts.clear_fonts();

        let mut config = FontConfig::default();
        config.name = Some("Cascadia Code 16pt".to_owned());
        let sources = [FontSource::TtfData {
            data: aleph_vulkan::embedded::data::fonts::cascadia_code(),
            size_pixels: 16.0,
            config: Some(config),
        }];
        fonts.add_font(&sources);

        let mut config = FontConfig::default();
        config.name = Some("Cascadia Code 20pt".to_owned());
        let sources = [FontSource::TtfData {
            data: aleph_vulkan::embedded::data::fonts::cascadia_code(),
            size_pixels: 20.0,
            config: Some(config),
        }];
        fonts.add_font(&sources);

        let mut config = FontConfig::default();
        config.name = Some("Cascadia Code 24pt".to_owned());
        let sources = [FontSource::TtfData {
            data: aleph_vulkan::embedded::data::fonts::cascadia_code(),
            size_pixels: 24.0,
            config: Some(config),
        }];
        fonts.add_font(&sources);

        let mut config = FontConfig::default();
        config.name = Some("Cascadia Code 36pt".to_owned());
        let sources = [FontSource::TtfData {
            data: aleph_vulkan::embedded::data::fonts::cascadia_code(),
            size_pixels: 36.0,
            config: Some(config),
        }];
        fonts.add_font(&sources);

        let data = fonts.build_rgba32_texture();
        let dimensions = (data.width, data.height);
        let data = data.data;

        let sampler = Self::create_sampler(device);
        let (image, allocation) = Self::create_image(allocator, dimensions);
        let image_view = Self::create_image_view(device, image);

        unsafe {
            let sampled_image_info = [DescriptorImageInfoBuilder::new()
                .image_view(image_view)
                .image_layout(ImageLayout::SHADER_READ_ONLY_OPTIMAL)];
            let sampled_image = WriteDescriptorSetBuilder::new()
                .descriptor_type(DescriptorType::SAMPLED_IMAGE)
                .dst_set(global.descriptor_set)
                .image_info(&sampled_image_info)
                .dst_binding(0);

            let sampler_info = [DescriptorImageInfoBuilder::new().sampler(sampler)];
            let sampler = WriteDescriptorSetBuilder::new()
                .descriptor_type(DescriptorType::SAMPLER)
                .dst_set(global.descriptor_set)
                .image_info(&sampler_info)
                .dst_binding(1);
            device
                .loader()
                .update_descriptor_sets(&[sampled_image, sampler], &[])
        }

        let font = ImguiFont {
            sampler,
            allocation,
            dimensions,
            image,
            image_view,
        };

        font.upload_font(device, allocator, data);

        font
    }

    pub fn create_sampler(device: &aleph_vulkan_core::Device) -> Sampler {
        let create_info = SamplerCreateInfoBuilder::new()
            .address_mode_u(SamplerAddressMode::REPEAT)
            .address_mode_v(SamplerAddressMode::REPEAT)
            .address_mode_w(SamplerAddressMode::REPEAT)
            .anisotropy_enable(false)
            .min_filter(Filter::LINEAR)
            .mag_filter(Filter::LINEAR)
            .mipmap_mode(SamplerMipmapMode::LINEAR)
            .min_lod(-1000.0)
            .max_lod(1000.0);
        unsafe {
            let sampler = device
                .loader()
                .create_sampler(&create_info, None, None)
                .expect("Failed to create sampler");
            sampler.add_debug_name(
                device,
                aleph_macros::cstr!(concat!(module_path!(), "::FontSampler")),
            );
            sampler
        }
    }

    pub fn create_image(allocator: &Allocator, dimensions: (u32, u32)) -> (Image, Allocation) {
        let mut extent = Extent3D::default();
        extent.width = dimensions.0;
        extent.height = dimensions.1;
        extent.depth = 1;
        let image_create_info = ImageCreateInfoBuilder::new()
            .format(Format::R8G8B8A8_UNORM)
            .initial_layout(ImageLayout::UNDEFINED)
            .samples(SampleCountFlagBits::_1)
            .tiling(ImageTiling::OPTIMAL)
            .usage(ImageUsageFlags::SAMPLED | ImageUsageFlags::TRANSFER_DST)
            .sharing_mode(SharingMode::EXCLUSIVE)
            .image_type(ImageType::_2D)
            .mip_levels(1)
            .array_layers(1)
            .extent(extent);
        let alloc_create_info = AllocationCreateInfoBuilder::new()
            .usage(MemoryUsage::GPUOnly)
            .build();
        unsafe {
            let image = allocator
                .create_image(&image_create_info, &alloc_create_info)
                .expect("Failed to create image");
            image.0.add_debug_name(
                allocator.device(),
                aleph_macros::cstr!(concat!(module_path!(), "::FontImage")),
            );
            image
        }
    }

    pub fn create_image_view(device: &aleph_vulkan_core::Device, image: Image) -> ImageView {
        let subresource_range = ImageSubresourceRangeBuilder::new()
            .aspect_mask(ImageAspectFlags::COLOR)
            .base_array_layer(0)
            .base_mip_level(0)
            .layer_count(1)
            .level_count(1);
        let create_info = ImageViewCreateInfoBuilder::new()
            .image(image)
            .format(Format::R8G8B8A8_UNORM)
            .view_type(ImageViewType::_2D)
            .subresource_range(*subresource_range);
        unsafe {
            let image_view = device
                .loader()
                .create_image_view(&create_info, None, None)
                .expect("Failed to create image view");
            image_view.add_debug_name(
                device,
                aleph_macros::cstr!(concat!(module_path!(), "::FontView")),
            );
            image_view
        }
    }

    pub fn upload_font(
        &self,
        device: &aleph_vulkan_core::Device,
        allocator: &Allocator,
        data: &[u8],
    ) {
        //
        // Creating then immediately destroying a command pool isn't very efficient, but I don't
        // care that much. It's just for uploading the ImGui font texture which will happen once
        //
        let create_info = CommandPoolCreateInfoBuilder::new()
            .queue_family_index(device.general_family().index)
            .flags(CommandPoolCreateFlags::TRANSIENT);
        let command_pool = unsafe {
            device
                .loader()
                .create_command_pool(&create_info, None, None)
        }
        .expect("Failed to create command pool");

        let allocate_info = CommandBufferAllocateInfoBuilder::new()
            .command_pool(command_pool)
            .command_buffer_count(1)
            .level(CommandBufferLevel::PRIMARY);
        let command_buffer = unsafe { device.loader().allocate_command_buffers(&allocate_info) }
            .expect("Failed to allocate command buffer")[0];

        let size = self.dimensions.0 * self.dimensions.1 * 4;
        let size = size as u64;
        let buffer_create_info = BufferCreateInfoBuilder::new()
            .usage(BufferUsageFlags::TRANSFER_SRC)
            .sharing_mode(SharingMode::EXCLUSIVE)
            .size(size);
        let alloc_create_info = AllocationCreateInfoBuilder::new()
            .usage(MemoryUsage::CPUOnly)
            .build();
        let (upload_buffer, upload_allocation) =
            unsafe { allocator.create_buffer(&buffer_create_info, &alloc_create_info) }
                .expect("Failed to create font image staging buffer");

        unsafe {
            let mem = allocator
                .map_memory(&upload_allocation)
                .expect("Failed to map upload memory");
            mem.copy_from(data.as_ptr(), data.len());
            allocator.unmap_memory(&upload_allocation);
        }

        unsafe {
            let begin_info = CommandBufferBeginInfoBuilder::new()
                .flags(CommandBufferUsageFlags::ONE_TIME_SUBMIT);
            device
                .loader()
                .begin_command_buffer(command_buffer, &begin_info)
                .expect("Failed to begin command buffer");

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
                .image(self.image)
                .subresource_range(*subresource_range)
                .src_access_mask(AccessFlags::default())
                .dst_access_mask(AccessFlags::TRANSFER_WRITE)
                .old_layout(ImageLayout::UNDEFINED)
                .new_layout(ImageLayout::TRANSFER_DST_OPTIMAL)];
            device.loader().cmd_pipeline_barrier(
                command_buffer,
                PipelineStageFlags::HOST,
                PipelineStageFlags::TRANSFER,
                DependencyFlagBits(0).bitmask(),
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
            extent.width = self.dimensions.0;
            extent.height = self.dimensions.1;
            extent.depth = 1;
            let region = BufferImageCopyBuilder::new()
                .image_subresource(*subresource)
                .image_extent(extent);
            let regions = [region];
            device.loader().cmd_copy_buffer_to_image(
                command_buffer,
                upload_buffer,
                self.image,
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
                .image(self.image)
                .subresource_range(*subresource_range)
                .src_access_mask(AccessFlags::TRANSFER_WRITE)
                .dst_access_mask(AccessFlags::SHADER_READ)
                .old_layout(ImageLayout::TRANSFER_DST_OPTIMAL)
                .new_layout(ImageLayout::SHADER_READ_ONLY_OPTIMAL);
            let image_memory_barriers = [image_barrier];
            device.loader().cmd_pipeline_barrier(
                command_buffer,
                PipelineStageFlags::TRANSFER,
                PipelineStageFlags::FRAGMENT_SHADER,
                DependencyFlagBits(0).bitmask(),
                &memory_barriers,
                &buffer_memory_barriers,
                &image_memory_barriers,
            );

            device
                .loader()
                .end_command_buffer(command_buffer)
                .expect("Failed to end command buffer");
        }

        unsafe {
            let command_buffers = [command_buffer];
            let submit = SubmitInfoBuilder::new().command_buffers(&command_buffers);
            let submits = [submit];
            device
                .loader()
                .queue_submit(device.general_queue(), &submits, Fence::null())
                .expect("Failed to submit command buffer");
            device
                .loader()
                .queue_wait_idle(device.general_queue())
                .expect("Failed to wait on queue idle");
        }

        unsafe {
            allocator.destroy_buffer(upload_buffer, upload_allocation);
            device.loader().destroy_command_pool(command_pool, None);
        }
    }

    pub unsafe fn destroy(&self, device: &aleph_vulkan_core::Device, allocator: &Allocator) {
        device.loader().destroy_image_view(self.image_view, None);
        allocator.destroy_image(self.image, self.allocation);
        device.loader().destroy_sampler(self.sampler, None);
    }
}
