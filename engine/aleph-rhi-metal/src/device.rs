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
use std::ptr::NonNull;
use std::sync::Arc;
use std::time::Duration;

use aleph_any::{AnyArc, AnyWeak, declare_interfaces};
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::bump_cell::BlinkCell;
use aleph_rhi_impl_utils::object_counter::ObjectCounter;
use aleph_rhi_impl_utils::parameter_block_layout_visitor::{
    ParameterBlockLayoutVisitor, ParameterBlockLayoutVisitorElement,
};
use allocator_api2::vec::Vec as BVec;
use block2::RcBlock;
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_metal::*;
use parking_lot::{Condvar, Mutex};

use crate::adapter::Adapter;
use crate::binding_signature::BindingSignature;
use crate::buffer::Buffer;
use crate::command_list::CommandList;
use crate::context::Context;
use crate::descriptor_arena::{DescriptorArenaHeap, DescriptorArenaLinear};
use crate::descriptor_pool::DescriptorPool;
use crate::fence::Fence;
use crate::internal::image_view::ImageViewObject;
use crate::internal::parameter_block::ParameterBlock;
use crate::internal::unwrap;
use crate::parameter_block_layout::ParameterBlockLayout;
use crate::pipeline::{ComputePipeline, GraphicsPipeline};
use crate::queue::Queue;
use crate::sampler::Sampler;
use crate::semaphore::Semaphore;
use crate::texture::Texture;

pub struct Device {
    pub(crate) this: AnyWeak<Self>,
    pub(crate) context: AnyArc<Context>,
    pub(crate) _adapter: AnyArc<Adapter>,
    pub(crate) device: Retained<ProtocolObject<dyn MTLDevice>>,
    pub(crate) listener: Retained<MTLSharedEventListener>,
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
        let _lock1 = self.general_queue.as_ref().map(|v| v.submit_lock());
        let _lock2 = self.compute_queue.as_ref().map(|v| v.submit_lock());
        let _lock3 = self.transfer_queue.as_ref().map(|v| v.submit_lock());
        if let Some(queue) = &self.general_queue {
            queue.garbage_collect_internal();
        }
        if let Some(queue) = &self.compute_queue {
            queue.garbage_collect_internal();
        }
        if let Some(queue) = &self.transfer_queue {
            queue.garbage_collect_internal();
        }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn wait_idle(&self) {
        let _lock1 = self.general_queue.as_ref().map(|v| v.submit_lock());
        let _lock2 = self.compute_queue.as_ref().map(|v| v.submit_lock());
        let _lock3 = self.transfer_queue.as_ref().map(|v| v.submit_lock());
        if let Some(queue) = &self.general_queue {
            queue.wait_idle_internal();
        }
        if let Some(queue) = &self.compute_queue {
            queue.wait_idle_internal();
        }
        if let Some(queue) = &self.transfer_queue {
            queue.wait_idle_internal();
        }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_parameter_block_layout(
        &self,
        desc: &ParameterBlockDesc,
    ) -> Result<AnyArc<dyn IParameterBlockLayout>, ParameterBlockLayoutCreateError> {
        ParameterBlockLayout::create(self, desc)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_binding_signature(
        &self,
        desc: &BindingSignatureDesc,
    ) -> Result<AnyArc<dyn IBindingSignature>, BindingSignatureCreateError> {
        BindingSignature::create(self, desc)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    #[aleph_profile::function]
    fn create_graphics_pipeline(
        &self,
        desc: &GraphicsPipelineDesc,
    ) -> Result<GraphicsPipelineHandle, PipelineCreateError> {
        GraphicsPipeline::create(self, desc)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    #[aleph_profile::function]
    fn create_compute_pipeline(
        &self,
        desc: &ComputePipelineDesc,
    ) -> Result<ComputePipelineHandle, PipelineCreateError> {
        ComputePipeline::create(self, desc)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_descriptor_pool(
        &self,
        desc: &DescriptorPoolDesc,
    ) -> Result<Box<dyn IDescriptorPool>, DescriptorPoolCreateError> {
        DescriptorPool::create(self, desc)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_descriptor_arena(
        &self,
        desc: &DescriptorArenaDesc,
    ) -> Result<Box<dyn IDescriptorArena>, DescriptorPoolCreateError> {
        match desc.arena_type {
            DescriptorArenaType::Linear => DescriptorArenaLinear::create(self, desc),
            DescriptorArenaType::Heap => DescriptorArenaHeap::create(self, desc),
        }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_buffer(&self, desc: &BufferDesc) -> Result<BufferHandle, BufferCreateError> {
        Buffer::create(self, desc)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_texture(&self, desc: &TextureDesc) -> Result<TextureHandle, TextureCreateError> {
        Texture::create(self, desc)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_sampler(&self, desc: &SamplerDesc) -> Result<SamplerHandle, SamplerCreateError> {
        Sampler::create(self, desc)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_command_list(
        &self,
        desc: &CommandListDesc,
    ) -> Result<Box<dyn ICommandList>, CommandListCreateError> {
        CommandList::create(self, desc)
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

    unsafe fn update_parameter_block(
        &self,
        layout: &dyn IParameterBlockLayout,
        block: ParameterBlockHandle,
        base: u32,
        writes: &[ParameterWrite],
    ) {
        let layout = unwrap::parameter_block_layout(layout);
        let block = unsafe { block.into_raw::<ParameterBlock>().as_mut() };
        let cpu_handle = block.cpu_addr.unwrap();

        let mut update_use_sets =
            |write_group: &ParameterBlockLayoutVisitorElement,
             src: &ProtocolObject<dyn MTLResource>| unsafe {
                if write_group.ty.is_uav() {
                    let base = layout.compiled.use_write_bases[write_group.binding as usize]
                        + write_group.element as usize;
                    block.writes.as_mut()[base] = NonNull::from(src).as_ptr();
                } else {
                    let base = layout.compiled.use_read_bases[write_group.binding as usize]
                        + write_group.element as usize;
                    block.reads.as_mut()[base] = NonNull::from(src).as_ptr();
                }
            };

        let visitor =
            ParameterBlockLayoutVisitor::new(layout.desc.get(), base as u64, writes).unwrap();
        for write_group in visitor {
            for (i, write) in write_group.writes.iter().enumerate() {
                let i = i + write_group.index as usize;
                match write {
                    ParameterWrite::Sampler(v) => unsafe {
                        let sampler = Sampler::get(v.sampler);
                        let id = sampler.objects.sampler.gpuResourceID().to_raw();
                        cpu_handle.add(i).write(id);
                    },
                    ParameterWrite::Texture(v) => unsafe {
                        let src = v.image_view.into_raw::<ImageViewObject>().as_ref();
                        let id = src.texture.gpuResourceID().to_raw();
                        cpu_handle.add(i).write(id);
                        update_use_sets(&write_group, src.texture.as_ref());
                    },
                    ParameterWrite::Buffer(v) => unsafe {
                        let src = Buffer::get(v.buffer);
                        let addr = src.objects.buffer.gpuAddress() + v.offset;
                        cpu_handle.add(i).write(addr);
                        update_use_sets(&write_group, src.objects.buffer.as_ref());
                    },
                    ParameterWrite::TextureBuffer(_) => unimplemented!(),
                }
            }
        }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_fence(&self, signalled: bool) -> Result<FenceHandle, FenceCreateError> {
        Fence::create(self, signalled)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_semaphore(&self) -> Result<SemaphoreHandle, SemaphoreCreateError> {
        Semaphore::create(self)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn wait_fences(
        &self,
        fences: &[&FenceHandle],
        wait_all: bool,
        timeout: u32,
    ) -> FenceWaitResult {
        match fences {
            // The single fence case can just call a wait function directly.
            [fence] => {
                let fence = Fence::get(fence);
                let result = unsafe {
                    fence
                        .objects
                        .event
                        .waitUntilSignaledValue_timeoutMS(fence.get_wait_value(), timeout as u64)
                };
                if result {
                    FenceWaitResult::Complete
                } else {
                    FenceWaitResult::Timeout
                }
            }
            // The multi-fence case requires some work of our own to group the wait into a
            // single operation. There's no 'wait multiple' available so we need to use another
            // sync primitive to get the behavior we want.
            _ => DEVICE_BUMP.with(|bump| -> FenceWaitResult {
                let bump = bump.scope();

                let mut inner_fences = BVec::with_capacity_in(fences.len(), bump.allocator());
                inner_fences.extend(fences.iter().map(|v| Fence::get(v)));

                // We do a speculative poll of the fences to see if we can exit without having
                // to run through any of the
                if wait_all {
                    // For the 'wait all' case we do a pre-check to see if all the fences are
                    // already signalled. If they are we can early exit without allocating any
                    // sync objects.
                    'unsignalled_check: {
                        for fence in &inner_fences {
                            if !fence.poll_signalled() {
                                // If we find an unsignalled fence then we bail from the outer
                                // block. This prevents us from hitting the 'return' statement
                                // below.
                                break 'unsignalled_check;
                            }
                        }
                        // If we escape the loop and don't find any unsignalled fences then
                        // we can immediately return as the wait conditions are complete.
                        return FenceWaitResult::Complete;
                    }
                } else {
                    // For the 'wait any' case we do a pre-check to see if any of the fences
                    // are already signalled. This avoids creating our sync objects for no
                    // reason.
                    for fence in &inner_fences {
                        if fence.poll_signalled() {
                            // If we find _any_ signalled fence in this case we can immediately
                            // return as the wait operaiton is complete.
                            return FenceWaitResult::Complete;
                        }
                    }
                }

                // If we reach this point we have, at minimum, polled that the wait condition
                // has not yet been met. If the timeout is set to 0 then we can immediately
                // exit and avoid all of the machinery below. The caller has, after all, asked
                // to 'wait' for 0ms.
                if timeout == 0 {
                    return FenceWaitResult::Timeout;
                }

                // Construct our condvar that will be used to block the thread that called
                // IDevice::wait_fences. We adjust the count to wait for based on the 'wait_all'
                // flag. 'wait_all = true' requires all fences to signal and sets the count to
                // 'fences.len()'. 'wait_all = false' only requires a single fence to signal so
                // we set the count to 1.
                let fence_num =
                    isize::try_from(fences.len()).expect("Waiting on too many fences. How???????");
                let wait_count = if wait_all { fence_num } else { 1 };
                let pair = Arc::new((Mutex::new(wait_count), Condvar::new()));

                // This is our notify closure. This will be sent off into the aether of Metal
                // and/or Apple's dispatch queue. We update each event underlying our fences to
                // call our notify function once it becomes signalled.
                let notify_pair = pair.clone();
                let notify_block = RcBlock::new(
                    move |_event: NonNull<ProtocolObject<dyn MTLSharedEvent>>, _value: u64| {
                        // This code relies on 'notifyListener' calling the closure even if the
                        // fence is _already_ signalled when attached to the MTLSharedEvent. If it
                        // doesn't then we may deadlock waiting on a signal that will never come.
                        let (lock, cvar) = notify_pair.as_ref();
                        let mut waiting = lock.lock();
                        *waiting -= 1;
                        cvar.notify_one();
                    },
                );

                // Add a listener to every fence in the set that will notify and ultimately
                // unblock our waiting thread once all the fences have been signalled.
                for fence in inner_fences {
                    unsafe {
                        // TODO: we need to
                        // 1) Test that this _drops_ the block once the notification has been called
                        //    so that we don't leak the Arc
                        // 2) Test that this calls the block even if the event is already
                        //    signalled.
                        let block = RcBlock::into_raw(notify_block.copy());
                        fence.objects.event.notifyListener_atValue_block(
                            &self.listener,
                            fence.get_wait_value(),
                            block,
                        );
                    }
                }

                // Finally, we wait for the fences to be signalled. This is where we will stall
                // the thread waiting for the condition to complete.
                let (lock, cvar) = pair.as_ref();
                let mut waiting = lock.lock();
                let result = cvar.wait_while_for(
                    &mut waiting,
                    |v| *v > 0,
                    Duration::from_millis(timeout as u64),
                );

                if result.timed_out() {
                    FenceWaitResult::Timeout
                } else {
                    FenceWaitResult::Complete
                }
            }),
        }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn poll_fence(&self, fence: &FenceHandle) -> bool {
        let fence = Fence::get(fence);
        fence.poll_signalled()
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn reset_fences(&self, _fences: &[&FenceHandle]) {
        // Fence reset is a no-op on metal as a fence is always ready to use. It uses a monotonic
        // counter to keep the signals and waits correct.
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
        Buffer::get(buffer).map_buffer()
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn unmap_buffer(&self, buffer: &BufferHandle) -> Result<(), ResourceUnmapError> {
        Buffer::get(buffer).unmap_buffer()
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn flush_buffer_range(&self, buffer: &BufferHandle, offset: u64, len: u64) {
        Buffer::get(buffer).flush_buffer_range(offset, len)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn invalidate_buffer_range(&self, buffer: &BufferHandle, offset: u64, len: u64) {
        Buffer::get(buffer).invalidate_buffer_range(offset, len)
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
        Texture::get(texture).get_view(desc)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_texture_rtv(
        &self,
        texture: &TextureHandle,
        desc: &ImageViewDesc,
    ) -> Result<ImageView, ()> {
        Texture::get(texture).get_rtv(desc)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_texture_dsv(
        &self,
        texture: &TextureHandle,
        desc: &ImageViewDesc,
    ) -> Result<ImageView, ()> {
        Texture::get(texture).get_dsv(desc)
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

    fn get_graphics_pipeline_id(&self, pipeline: &GraphicsPipelineHandle) -> std::num::NonZeroU64 {
        GraphicsPipeline::get(pipeline).id
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_compute_pipeline_id(&self, pipeline: &ComputePipelineHandle) -> std::num::NonZeroU64 {
        ComputePipeline::get(pipeline).id
    }
}

impl Device {
    pub fn get_queue_internal(&self, queue_type: QueueType) -> Option<&Queue> {
        match queue_type {
            QueueType::General => self.general_queue.as_deref(),
            QueueType::Compute => self.compute_queue.as_deref(),
            QueueType::Transfer => self.transfer_queue.as_deref(),
        }
    }
}

thread_local! {
    pub static DEVICE_BUMP: BlinkCell = BlinkCell::new();
}
