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

use crate::renderer::GlobalObjects;
use dx12::{dxgi, D3D12Object};
use pix::RecordScopedEvent;

pub struct PerFrameObjects {
    pub vtx_buffer: dx12_alloc::Allocation,
    pub idx_buffer: dx12_alloc::Allocation,

    pub command_allocator: dx12::CommandAllocator,

    pub font_staging_allocation: dx12_alloc::Allocation,
    pub font_staging_resource: dx12::Resource,

    pub font_staged_pool: dx12_alloc::Pool,
    pub font_staged_image: Option<dx12_alloc::Allocation>,
    pub font_cpu_srv: dx12::CPUDescriptorHandle,
    pub font_gpu_srv: dx12::GPUDescriptorHandle,
    pub font_staged_size: (u32, u32),
    pub font_staged_hash: u64,
}

impl PerFrameObjects {
    pub fn new(
        device: &dx12::Device,
        allocator: &dx12_alloc::Allocator,
        global: &GlobalObjects,
        index: usize,
    ) -> Self {
        let alloc_desc = dx12_alloc::AllocationDesc::builder()
            .heap_type(dx12::HeapType::Upload)
            .build();
        let vtx_buffer = {
            let resource_desc = dx12::ResourceDesc::builder()
                .dimension(dx12::ResourceDimension::Buffer)
                .width(Self::vertex_buffer_size() as _)
                .build();
            allocator
                .create_resource(
                    &alloc_desc,
                    &resource_desc,
                    dx12::ResourceStates::GENERIC_READ,
                    None,
                )
                .unwrap()
        };
        vtx_buffer
            .get_resource()
            .unwrap()
            .set_name("egui::VtxBuffer")
            .unwrap();

        let idx_buffer = {
            let resource_desc = dx12::ResourceDesc::builder()
                .dimension(dx12::ResourceDimension::Buffer)
                .width(Self::index_buffer_size() as _)
                .build();
            allocator
                .create_resource(
                    &alloc_desc,
                    &resource_desc,
                    dx12::ResourceStates::GENERIC_READ,
                    None,
                )
                .unwrap()
        };
        idx_buffer
            .get_resource()
            .unwrap()
            .set_name("egui::IdxBuffer")
            .unwrap();

        let font_staging_allocation =
            unsafe { Self::create_font_staging_allocation(&allocator, (4096, 4096)) };
        let font_staging_resource = font_staging_allocation.get_resource().unwrap();
        font_staging_resource
            .set_name("egui::FontStagingBuffer")
            .unwrap();

        let font_staged_pool = unsafe { Self::create_staged_pool(&allocator, (4096, 4096)) };

        let size = device.get_descriptor_handle_increment_size(dx12::DescriptorHeapType::CbvSrvUav);
        let font_cpu_srv = global
            .srv_heap
            .get_cpu_descriptor_handle_for_heap_start()
            .unwrap()
            .add(index * size as usize);
        let font_gpu_srv = global
            .srv_heap
            .get_gpu_descriptor_handle_for_heap_start()
            .unwrap()
            .add(index as u64 * size as u64);

        let command_allocator = device
            .create_command_allocator(dx12::CommandListType::Direct)
            .unwrap();
        command_allocator
            .set_name("egui::CommandAllocator")
            .unwrap();

        Self {
            vtx_buffer,
            idx_buffer,
            command_allocator,
            font_staging_allocation,
            font_staging_resource,
            font_staged_pool,
            font_staged_image: None,
            font_cpu_srv,
            font_gpu_srv,
            font_staged_size: (0, 0),
            font_staged_hash: 0,
        }
    }

    pub unsafe fn update_texture_data(
        &mut self,
        device: &dx12::Device,
        allocator: &dx12_alloc::Allocator,
        texture: &egui::Texture,
    ) -> bool {
        debug_assert_eq!(texture.pixels.len(), texture.width * texture.height);

        let dimensions = (texture.width as u32, texture.height as u32);
        if dimensions != self.font_staged_size || texture.version != self.font_staged_hash {
            // Explicitly drop staged image so the pool's memory will be free to create the new
            // image
            self.font_staged_image = None;

            // Create the GPU image with the new dimensions
            self.create_staged_resources(allocator, dimensions);

            // Update the srv to point at the newly created image
            self.update_srv(device);

            // Update the metadata for determining when to re-upload the texture
            self.font_staged_size = dimensions;
            self.font_staged_hash = texture.version;

            // Map and write the texture data to our staging buffer
            let ptr = self
                .font_staging_resource
                .map(0, Some(0..0))
                .unwrap()
                .unwrap();
            ptr.as_ptr()
                .copy_from_nonoverlapping(texture.pixels.as_ptr(), texture.pixels.len());
            self.font_staging_resource.unmap(0, None);
            true
        } else {
            false
        }
    }

    pub unsafe fn record_texture_upload(&mut self, command_list: &mut dx12::GraphicsCommandList) {
        command_list.scoped_event(pix::Colour::GREEN, "Egui Texture Upload", |command_list| {
            let staged_resource = self
                .font_staged_image
                .as_ref()
                .unwrap()
                .get_resource()
                .unwrap();

            let dst = dx12::TextureCopyLocation::Subresource {
                resource: Some(staged_resource.clone()),
                subresource_index: 0,
            };
            let src = dx12::TextureCopyLocation::Placed {
                resource: Some(self.font_staging_resource.clone()),
                placed_footprint: dx12::PlacedSubresourceFootprint {
                    offset: 0,
                    footprint: dx12::SubresourceFootprint {
                        format: dxgi::Format::R8Unorm,
                        width: self.font_staged_size.0,
                        height: self.font_staged_size.1,
                        depth: 1,
                        row_pitch: self.font_staged_size.0,
                    },
                },
            };
            command_list.copy_texture_region(&dst, 0, 0, 0, &src, None);

            let barrier = dx12::ResourceBarrier::Transition {
                flags: Default::default(),
                resource: Some(staged_resource),
                subresource: 0,
                state_before: dx12::ResourceStates::COPY_DEST,
                state_after: dx12::ResourceStates::PIXEL_SHADER_RESOURCE,
            };
            command_list.resource_barrier(&[barrier]);
        });
    }

    /// Allocates the font texture on GPU memory
    fn create_staged_resources(
        &mut self,
        allocator: &dx12_alloc::Allocator,
        dimensions: (u32, u32),
    ) {
        let alloc_desc = dx12_alloc::AllocationDesc::builder()
            .heap_type(dx12::HeapType::Default)
            .pool(&self.font_staged_pool)
            .build();
        let resource_desc = dx12::ResourceDesc::builder()
            .dimension(dx12::ResourceDimension::Texture2D)
            .width(dimensions.0 as _)
            .height(dimensions.1 as _)
            .format(dxgi::Format::R8Unorm)
            .layout(dx12::TextureLayout::Unknown)
            .build();
        let allocation = allocator
            .create_resource(
                &alloc_desc,
                &resource_desc,
                dx12::ResourceStates::COPY_DEST,
                None,
            )
            .unwrap();
        allocation
            .get_resource()
            .unwrap()
            .set_name("egui::FontImage")
            .unwrap();

        self.font_staged_image = Some(allocation);
    }

    unsafe fn update_srv(&self, device: &dx12::Device) {
        let srv_desc = dx12::ShaderResourceViewDesc::Texture2D {
            format: dxgi::Format::R8Unorm,
            component_mapping: dx12::ComponentMapping::identity(),
            texture_2d: dx12::Tex2DSrv {
                most_detailed_mip: 0,
                mip_levels: 1,
                plane_slice: 0,
                resource_min_lod_clamp: 0.0,
            },
        };
        device.create_shader_resource_view(
            &self
                .font_staged_image
                .as_ref()
                .unwrap()
                .get_resource()
                .unwrap(),
            &srv_desc,
            self.font_cpu_srv,
        );
    }

    unsafe fn create_font_staging_allocation(
        allocator: &dx12_alloc::Allocator,
        dimensions: (u32, u32),
    ) -> dx12_alloc::Allocation {
        let size = dimensions.0 * dimensions.1;

        let alloc_desc = dx12_alloc::AllocationDesc::builder()
            .heap_type(dx12::HeapType::Upload)
            .build();
        let resource_desc = dx12::ResourceDesc::builder()
            .dimension(dx12::ResourceDimension::Buffer)
            .width(size as _)
            .build();
        let initial_resource_state = dx12::ResourceStates::GENERIC_READ;
        let allocation = allocator
            .create_resource(&alloc_desc, &resource_desc, initial_resource_state, None)
            .unwrap();
        allocation
    }

    unsafe fn create_staged_pool(
        allocator: &dx12_alloc::Allocator,
        dimensions: (u32, u32),
    ) -> dx12_alloc::Pool {
        let size = dimensions.0 * dimensions.1;

        let pool_desc = dx12_alloc::PoolDesc::builder()
            .heap_type(dx12::HeapType::Default)
            .heap_flags(dx12::HeapFlags::ALLOW_ONLY_NON_RT_DS_TEXTURES)
            .block_size(size as _)
            .min_block_count(1)
            .max_block_count(1)
            .build();

        let pool = allocator.create_pool(&pool_desc).unwrap();

        pool
    }

    pub fn vertex_buffer_size() -> usize {
        1024 * 1024 * 4
    }

    pub fn index_buffer_size() -> usize {
        1024 * 1024 * 2
    }
}
