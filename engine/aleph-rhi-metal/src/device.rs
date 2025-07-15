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

use std::any::TypeId;

use aleph_any::{AnyArc, AnyWeak, declare_interfaces};
use aleph_object_system::ArcedObject;
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::bump_cell::BlinkCell;
use aleph_rhi_impl_utils::object_counter::ObjectCounter;
use aleph_rhi_impl_utils::owned_desc::{OwnedBufferDesc, OwnedSamplerDesc, OwnedTextureDesc};
use crossbeam::queue::ArrayQueue;
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_metal::*;
use parking_lot::Mutex;

use crate::adapter::Adapter;
use crate::buffer::Buffer;
use crate::command_list::{CommandList, ListState};
use crate::context::Context;
use crate::descriptor_arena::DescriptorArena;
use crate::descriptor_pool::DescriptorPool;
use crate::descriptor_set_layout::DescriptorSetLayout;
use crate::fence::Fence;
use crate::pipeline::{ComputePipeline, GraphicsPipeline};
use crate::pipeline_layout::PipelineLayout;
use crate::queue::Queue;
use crate::sampler::Sampler;
use crate::semaphore::Semaphore;
use crate::texture::Texture;

pub struct Device {
    pub(crate) this: AnyWeak<Self>,
    pub(crate) context: AnyArc<Context>,
    pub(crate) adapter: AnyArc<Adapter>,
    pub(crate) device: Retained<ProtocolObject<dyn MTLDevice>>,
    pub(crate) general_queue: Option<AnyArc<Queue>>,
    pub(crate) compute_queue: Option<AnyArc<Queue>>,
    pub(crate) transfer_queue: Option<AnyArc<Queue>>,
    pub(crate) object_counter: ObjectCounter,
}

// Safety: Needed because of 'MTLDevice'
unsafe impl Send for Device {}
unsafe impl Sync for Device {}

declare_interfaces!(Device, [IDevice]);

impl IGetPlatformInterface for Device {
    unsafe fn __query_platform_interface(&self, _target: TypeId, _out: *mut ()) -> Option<()> {
        // TODO: Expose the device loader through an arc or something
        None
    }
}

impl IDevice for Device {
    // ========================================================================================== //
    // ========================================================================================== //

    fn upgrade(&self) -> AnyArc<dyn IDevice> {
        AnyArc::map::<dyn IDevice, _>(self.this.upgrade().unwrap(), |v| v)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn strong_count(&self) -> usize {
        self.this.strong_count()
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn weak_count(&self) -> usize {
        self.this.weak_count()
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn garbage_collect(&self) {
        if let Some(queue) = &self.general_queue {
            queue.garbage_collect();
        }
        if let Some(queue) = &self.compute_queue {
            queue.garbage_collect();
        }
        if let Some(queue) = &self.transfer_queue {
            queue.garbage_collect();
        }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn wait_idle(&self) {
        todo!()
    }

    // ========================================================================================== //
    // ========================================================================================== //

    #[aleph_profile::function]
    fn create_graphics_pipeline(
        &self,
        desc: &GraphicsPipelineDesc,
    ) -> Result<GraphicsPipelineHandle, PipelineCreateError> {
        DEVICE_BUMP.with(|bump_cell| {
            let bump = bump_cell.scope();

            let pipeline_layout = PipelineLayout::get_owned(desc.pipeline_layout);

            let out = GraphicsPipeline {
                _device: self.this.upgrade().unwrap(),
                _pipeline_layout: pipeline_layout,
                id: self.object_counter.next_graphics_pipeline(),
            };
            let out = ArcedObject::new_arc_opaque(out);
            unsafe { Ok(GraphicsPipelineHandle::new(out)) }
        })
    }

    // ========================================================================================== //
    // ========================================================================================== //

    #[aleph_profile::function]
    fn create_compute_pipeline(
        &self,
        desc: &ComputePipelineDesc,
    ) -> Result<ComputePipelineHandle, PipelineCreateError> {
        DEVICE_BUMP.with(|bump_cell| {
            let bump = bump_cell.scope();

            let pipeline_layout = PipelineLayout::get_owned(desc.pipeline_layout);

            let out = ComputePipeline {
                _device: self.this.upgrade().unwrap(),
                _pipeline_layout: pipeline_layout,
                id: self.object_counter.next_compute_pipeline(),
            };
            let out = ArcedObject::new_arc_opaque(out);
            unsafe { Ok(ComputePipelineHandle::new(out)) }
        })
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_descriptor_set_layout(
        &self,
        desc: &DescriptorSetLayoutDesc,
    ) -> Result<DescriptorSetLayoutHandle, DescriptorSetLayoutCreateError> {
        DEVICE_BUMP.with(|bump_cell| {
            let bump = bump_cell.scope();

            let mut samplers = Vec::new();
            for v in desc.items {
                if let Some(static_samplers) = v.static_samplers {
                    for sampler in static_samplers.iter().copied() {
                        let sampler = Sampler::get_owned(sampler);
                        samplers.push(sampler);
                    }
                }
            }

            let out = DescriptorSetLayout {
                _device: self.this.upgrade().unwrap(),
                _samplers: samplers,
                id: self.object_counter.next_set_layout(),
            };
            let out = ArcedObject::new_arc_opaque(out);
            unsafe { Ok(DescriptorSetLayoutHandle::new(out)) }
        })
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_descriptor_pool(
        &self,
        desc: &DescriptorPoolDesc,
    ) -> Result<Box<dyn IDescriptorPool>, DescriptorPoolCreateError> {
        DEVICE_BUMP.with(|bump_cell| {
            let bump = bump_cell.scope();

            let layout = DescriptorSetLayout::get_owned(desc.layout);

            let pool: Box<dyn IDescriptorPool> = Box::new(DescriptorPool {
                _device: self.this.upgrade().unwrap(),
                _layout: layout,
            });

            Ok(pool)
        })
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_descriptor_arena(
        &self,
        desc: &DescriptorArenaDesc,
    ) -> Result<Box<dyn IDescriptorArena>, DescriptorPoolCreateError> {
        DEVICE_BUMP.with(|bump_cell| {
            let bump = bump_cell.scope();

            let pool: Box<dyn IDescriptorArena> = Box::new(DescriptorArena {
                _device: self.this.upgrade().unwrap(),
            });

            Ok(pool)
        })
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_pipeline_layout(
        &self,
        desc: &PipelineLayoutDesc,
    ) -> Result<PipelineLayoutHandle, PipelineLayoutCreateError> {
        DEVICE_BUMP.with(|bump_cell| {
            let bump = bump_cell.scope();

            let out = PipelineLayout {
                _device: self.this.upgrade().unwrap(),
                id: self.object_counter.next_pipeline_layout(),
            };
            let out = ArcedObject::new_arc_opaque(out);
            unsafe { Ok(PipelineLayoutHandle::new(out)) }
        })
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_buffer(&self, desc: &BufferDesc) -> Result<BufferHandle, BufferCreateError> {
        let out = Buffer {
            _device: self.this.upgrade().unwrap(),
            id: self.object_counter.next_buffer(),
            map_state: Mutex::new(Default::default()),
            desc: OwnedBufferDesc::new(desc.clone()),
        };
        let out = ArcedObject::new_arc_opaque(out);
        unsafe { Ok(BufferHandle::new(out)) }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_texture(&self, desc: &TextureDesc) -> Result<TextureHandle, TextureCreateError> {
        DEVICE_BUMP.with(|bump_cell| {
            let bump = bump_cell.scope();

            let out = Texture {
                _device: self.this.upgrade().unwrap(),
                id: self.object_counter.next_texture(),
                is_owned: true,
                views: Default::default(),
                rtvs: Default::default(),
                dsvs: Default::default(),
                desc: OwnedTextureDesc::new(desc.clone()),
            };
            let out = ArcedObject::new_arc_opaque(out);
            unsafe { Ok(TextureHandle::new(out)) }
        })
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_sampler(&self, desc: &SamplerDesc) -> Result<SamplerHandle, SamplerCreateError> {
        DEVICE_BUMP.with(|bump_cell| {
            let bump = bump_cell.scope();

            let out = Sampler {
                _device: self.this.upgrade().unwrap(),
                id: self.object_counter.next_sampler(),
                desc: OwnedSamplerDesc::new(desc.clone()),
            };
            let out = ArcedObject::new_arc_opaque(out);
            unsafe { Ok(SamplerHandle::new(out)) }
        })
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_command_list(
        &self,
        desc: &CommandListDesc,
    ) -> Result<Box<dyn ICommandList>, CommandListCreateError> {
        DEVICE_BUMP.with(|bump_cell| {
            let bump = bump_cell.scope();

            let out: Box<dyn ICommandList> = Box::new(CommandList {
                _device: self.this.upgrade().unwrap(),
                list_type: desc.queue_type,
                state: ListState::Empty,
            });

            Ok(out)
        })
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_queue(&self, queue_type: QueueType) -> Option<AnyArc<dyn IQueue>> {
        match queue_type {
            QueueType::General => self
                .general_queue
                .clone()
                .map(|v| AnyArc::map::<dyn IQueue, _>(v, |v| v)),
            QueueType::Compute => self
                .compute_queue
                .clone()
                .map(|v| AnyArc::map::<dyn IQueue, _>(v, |v| v)),
            QueueType::Transfer => self
                .transfer_queue
                .clone()
                .map(|v| AnyArc::map::<dyn IQueue, _>(v, |v| v)),
        }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    unsafe fn update_descriptor_sets(&self, writes: &[DescriptorWriteDesc]) {
        todo!()
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_fence(&self, signalled: bool) -> Result<FenceHandle, FenceCreateError> {
        let fence = Fence {
            _device: self.this.upgrade().unwrap(),
        };
        let fence = ArcedObject::new_arc_opaque(fence);
        unsafe { Ok(FenceHandle::new(fence)) }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_semaphore(&self) -> Result<SemaphoreHandle, SemaphoreCreateError> {
        let semaphore = Semaphore {
            _device: self.this.upgrade().unwrap(),
        };
        let semaphore = ArcedObject::new_arc_opaque(semaphore);
        unsafe { Ok(SemaphoreHandle::new(semaphore)) }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn wait_fences(
        &self,
        fences: &[&FenceHandle],
        wait_all: bool,
        timeout: u32,
    ) -> FenceWaitResult {
        todo!()
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn poll_fence(&self, fence: &FenceHandle) -> bool {
        todo!()
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn reset_fences(&self, fences: &[&FenceHandle]) {
        todo!()
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_backend_api(&self) -> BackendAPI {
        BackendAPI::Metal
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_buffer_id(&self, buffer: &BufferHandle) -> std::num::NonZeroU64 {
        Buffer::get(buffer).get_buffer_id()
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_buffer_desc<'b>(&self, buffer: &'b BufferHandle) -> &'b BufferDesc<'b> {
        Buffer::get(buffer).desc()
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn map_buffer(&self, buffer: &BufferHandle) -> Result<std::ptr::NonNull<u8>, ResourceMapError> {
        Buffer::get(buffer).map_buffer(self)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn unmap_buffer(&self, buffer: &BufferHandle) -> Result<(), ResourceUnmapError> {
        Buffer::get(buffer).unmap_buffer(self)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn flush_buffer_range(&self, buffer: &BufferHandle, offset: u64, len: u64) {
        Buffer::get(buffer).flush_buffer_range(self, offset, len)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn invalidate_buffer_range(&self, buffer: &BufferHandle, offset: u64, len: u64) {
        Buffer::get(buffer).invalidate_buffer_range(self, offset, len)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_texture_id(&self, texture: &TextureHandle) -> std::num::NonZeroU64 {
        Texture::get(texture).get_id()
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_texture_desc<'b>(&self, texture: &'b TextureHandle) -> &'b TextureDesc<'b> {
        Texture::get(texture).desc()
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_texture_view(
        &self,
        texture: &TextureHandle,
        desc: &ImageViewDesc,
    ) -> Result<ImageView, ()> {
        Texture::get(texture).get_view(self, desc)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_texture_rtv(
        &self,
        texture: &TextureHandle,
        desc: &ImageViewDesc,
    ) -> Result<ImageView, ()> {
        Texture::get(texture).get_rtv(self, desc)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_texture_dsv(
        &self,
        texture: &TextureHandle,
        desc: &ImageViewDesc,
    ) -> Result<ImageView, ()> {
        Texture::get(texture).get_dsv(self, desc)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_sampler_id(&self, sampler: &SamplerHandle) -> std::num::NonZeroU64 {
        Sampler::get(sampler).id
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_sampler_desc<'b>(&self, sampler: &'b SamplerHandle) -> &'b SamplerDesc<'b> {
        Sampler::get(sampler).desc()
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_descriptor_set_layout_id(
        &self,
        set_layout: &DescriptorSetLayoutHandle,
    ) -> std::num::NonZeroU64 {
        DescriptorSetLayout::get(set_layout).id
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_pipeline_layout_id(
        &self,
        pipeline_layout: &PipelineLayoutHandle,
    ) -> std::num::NonZeroU64 {
        PipelineLayout::get(pipeline_layout).id
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_graphics_pipeline_id(&self, pipeline: &GraphicsPipelineHandle) -> std::num::NonZeroU64 {
        GraphicsPipeline::get(pipeline).id
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_compute_pipeline_id(&self, pipeline: &ComputePipelineHandle) -> std::num::NonZeroU64 {
        ComputePipeline::get(pipeline).id
    }
}

thread_local! {
    pub static DEVICE_BUMP: BlinkCell = BlinkCell::new();
}

pub struct FreeCommandList {
    pub list_type: QueueType,
}

impl FreeCommandList {
    pub unsafe fn collect(&self, device: &Device) {}
}

pub struct CommandListPool {
    pub general: ArrayQueue<FreeCommandList>,
    pub compute: ArrayQueue<FreeCommandList>,
    pub transfer: ArrayQueue<FreeCommandList>,
}

impl CommandListPool {
    pub fn new() -> Self {
        // We should only really ever need <num_lists_per_frame> * <frames_in_flight>
        Self {
            general: ArrayQueue::new(64),
            compute: ArrayQueue::new(32),
            transfer: ArrayQueue::new(32),
        }
    }

    pub fn get_for_queue_type(&self, queue_type: QueueType) -> Option<FreeCommandList> {
        match queue_type {
            QueueType::General => self.general.pop(),
            QueueType::Compute => self.compute.pop(),
            QueueType::Transfer => self.transfer.pop(),
        }
    }

    pub fn get_pool_for_queue_type(&self, queue_type: QueueType) -> &ArrayQueue<FreeCommandList> {
        match queue_type {
            QueueType::General => &self.general,
            QueueType::Compute => &self.compute,
            QueueType::Transfer => &self.transfer,
        }
    }

    pub unsafe fn collect(&self, device: &Device) {
        unsafe {
            while let Some(list) = self.general.pop() {
                list.collect(device);
            }

            while let Some(list) = self.compute.pop() {
                list.collect(device);
            }

            while let Some(list) = self.transfer.pop() {
                list.collect(device);
            }
        }
    }
}
