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

use crate::dx12::dxgi;
use crate::pix::RecordScopedEvent;
use crate::renderer::global::FontTexture;
use crate::renderer::GlobalObjects;
use crate::{dx12, pix};
use aleph_gpu_dx12::{IBufferExt, IDeviceExt, ITextureExt};
use interfaces::any::AnyArc;
use interfaces::gpu::{
    BarrierAccess, BarrierSync, BufferDesc, CpuAccessMode, Format, IBuffer, ICommandPool,
    IGeneralEncoder, ITexture, ImageLayout, ResourceStates, TextureBarrier, TextureDesc,
    TextureDimension, TextureSubResourceSet,
};
use std::ops::Deref;

pub struct PerFrameObjects {
    pub vtx_buffer: AnyArc<dyn IBuffer>,
    pub idx_buffer: AnyArc<dyn IBuffer>,

    pub command_allocator: AnyArc<dyn ICommandPool>,

    pub font_version: usize,
    pub font_staging_buffer: AnyArc<dyn IBuffer>,

    pub font_staged: Option<AnyArc<dyn ITexture>>,
    pub font_staged_size: (u32, u32),
    pub font_cpu_srv: dx12::CPUDescriptorHandle,
    pub font_gpu_srv: dx12::GPUDescriptorHandle,
}

impl PerFrameObjects {
    pub fn new(device: &dyn IDeviceExt, global: &GlobalObjects, index: usize) -> Self {
        let vtx_buffer = {
            let desc = BufferDesc {
                size: Self::vertex_buffer_size() as _,
                cpu_access: CpuAccessMode::Write,
                is_vertex_buffer: true,
                ..Default::default()
            };
            device.create_buffer(&desc).unwrap()
        };

        let idx_buffer = {
            let desc = BufferDesc {
                size: Self::index_buffer_size() as _,
                cpu_access: CpuAccessMode::Write,
                is_index_buffer: true,
                ..Default::default()
            };
            device.create_buffer(&desc).unwrap()
        };

        let font_staging_buffer = Self::create_font_staging_allocation(device, (4096, 4096));

        let size = device
            .get_raw_handle()
            .get_descriptor_handle_increment_size(dx12::DescriptorHeapType::CbvSrvUav);
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

        let command_allocator = device.create_command_pool().unwrap();
        command_allocator.set_name("egui::CommandAllocator");

        Self {
            vtx_buffer,
            idx_buffer,
            command_allocator,
            font_version: 0,
            font_staging_buffer,
            font_staged: None,
            font_staged_size: (0, 0),
            font_cpu_srv,
            font_gpu_srv,
        }
    }

    pub unsafe fn update_texture_data(&mut self, device: &dyn IDeviceExt, texture: &FontTexture) {
        // Check the data is correct
        assert_eq!(texture.bytes.len(), texture.width * texture.height);

        // Crunch our dimensions for d3d12
        let dimensions = (texture.width as u32, texture.height as u32);

        // Explicitly drop staged image so the pool's memory will be free to create the new
        // image
        self.font_staged = None;

        // Create the GPU image with the new dimensions
        self.create_staged_resources(device, dimensions);

        // Update the srv to point at the newly created image
        self.update_srv(&device.get_raw_handle());

        // Update the metadata for determining when to re-upload the texture
        self.font_version = texture.version;
        self.font_staged_size = dimensions;

        // Map and write the texture data to our staging buffer
        let resource = self
            .font_staging_buffer
            .query_interface::<dyn IBufferExt>()
            .unwrap()
            .get_raw_handle();
        let ptr = resource.map(0, Some(0..0)).unwrap().unwrap();
        ptr.as_ptr()
            .copy_from_nonoverlapping(texture.bytes.as_ptr(), texture.bytes.len());
        resource.unmap(0, None);
    }

    pub unsafe fn record_texture_upload(
        &mut self,
        command_list: &dx12::GraphicsCommandList,
        encoder: &mut dyn IGeneralEncoder,
    ) {
        command_list.scoped_event(pix::Colour::GREEN, "Egui Texture Upload", |command_list| {
            let staged_resource = self
                .font_staged
                .as_ref()
                .unwrap()
                .query_interface::<dyn ITextureExt>()
                .unwrap()
                .get_raw_handle();

            let dst = dx12::TextureCopyLocation::Subresource {
                resource: Some(staged_resource.clone()),
                subresource_index: 0,
            };
            let src = dx12::TextureCopyLocation::Placed {
                resource: Some(
                    self.font_staging_buffer
                        .query_interface::<dyn IBufferExt>()
                        .unwrap()
                        .get_raw_handle(),
                ),
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

            encoder.resource_barrier(
                &[],
                &[],
                &[TextureBarrier {
                    texture: self.font_staged.as_ref().unwrap().deref(),
                    subresource_range: TextureSubResourceSet {
                        base_mip_level: 0,
                        num_mip_levels: 1,
                        base_array_slice: 0,
                        num_array_slices: 1,
                    },
                    before_sync: BarrierSync::COPY,
                    after_sync: BarrierSync::ALL,
                    before_access: BarrierAccess::COPY_WRITE,
                    after_access: BarrierAccess::SHADER_SAMPLED_READ,
                    before_layout: ImageLayout::CopyDst,
                    after_layout: ImageLayout::ShaderReadOnlyOptimal,
                    split_barrier_mode: Default::default(),
                    queue_transition_mode: Default::default(),
                }],
            )
        });
    }

    /// Allocates the font texture on GPU memory
    fn create_staged_resources(&mut self, device: &dyn IDeviceExt, dimensions: (u32, u32)) {
        let image = device
            .create_texture(&TextureDesc {
                width: dimensions.0,
                height: dimensions.1,
                format: Format::R8Unorm,
                dimension: TextureDimension::Texture2D,
                initial_state: ResourceStates::COPY_DEST,
                array_size: 1,
                mip_levels: 1,
                sample_count: 1,
                sample_quality: 0,
                ..Default::default()
            })
            .unwrap();

        self.font_staged = Some(image);
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
                .font_staged
                .as_ref()
                .unwrap()
                .query_interface::<dyn ITextureExt>()
                .unwrap()
                .get_raw_handle(),
            &srv_desc,
            self.font_cpu_srv,
        );
    }

    fn create_font_staging_allocation(
        device: &dyn IDeviceExt,
        dimensions: (u32, u32),
    ) -> AnyArc<dyn IBuffer> {
        device
            .create_buffer(&BufferDesc {
                size: (dimensions.0 * dimensions.1) as u64,
                cpu_access: CpuAccessMode::Write,
                ..Default::default()
            })
            .unwrap()
    }

    pub fn vertex_buffer_size() -> usize {
        1024 * 1024 * 4
    }

    pub fn index_buffer_size() -> usize {
        1024 * 1024 * 2
    }
}
