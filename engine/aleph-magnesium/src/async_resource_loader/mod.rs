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

pub mod buffer_upload_range;
pub mod loader_notify;
pub mod texture_upload_range;

use std::cell::{Cell, RefCell};

use aleph_alloc::BVec;
use aleph_alloc::instrumentation::system;
use aleph_any::AnyArc;
use thiserror::Error;

use crate::async_resource_loader::buffer_upload_range::BufferUploadRange;
use crate::async_resource_loader::texture_upload_range::TextureUploadRange;
use crate::internal::async_resource_loader::MgAsyncLdrSystem;
use crate::internal::async_resource_loader::buffer::BufferLoadState;
use crate::internal::async_resource_loader::queued_copy_manager::{
    QueuedCopyManager, SubmittedCopy,
};
use crate::internal::async_resource_loader::renderer_channel::{
    LoaderSender, LoaderToRendererMessage,
};
use crate::internal::async_resource_loader::request_states::RequestStates;
use crate::internal::async_resource_loader::submission_manager::SubmissionManager;
use crate::internal::async_resource_loader::texture::TextureLoadState;
use crate::internal::async_resource_loader::upload_memory_manager::UploadMemoryManager;
use crate::internal::buffer::make_standard_buffer_desc;
use crate::internal::handle::{Handle, make_handle_id};
use crate::internal::texture::make_standard_texture_desc;
use crate::resource::texture::simple::SimpleTextureDesc;

make_handle_id!(BufferLoad);
pub type BufferLoadHandle = Handle<BufferLoad>;

make_handle_id!(TextureLoad);
pub type TextureLoadHandle = Handle<TextureLoad>;

pub struct AsyncResourceLoader<C: Send + 'static> {
    /// Configurable options that control loader behavior.
    config: AsyncResourceLoaderConfig,

    /// The GPU device handle.
    device: AnyArc<dyn rhi::IDevice>,

    /// Handle to the copy queue that we will issue copy commands on.
    queue: AnyArc<dyn rhi::IQueue>,

    /// Fence that we use for synchronizing all copy submissions with both the CPU and the GPU.
    fence: rhi::FenceHandle,

    /// A property of the associated copy queue. Best explained by [Vulkan documentation].
    ///
    /// In simple terms this is the size of the smallest possible texture transfer operation we can
    /// submit on the queue.
    ///
    ///   [Vulkan documentation]: https://docs.vulkan.org/refpages/latest/refpages/source/VkQueueFamilyProperties.html#_description
    min_image_granularity: rhi::Extent3D,

    /// Internal manager that handles sub-allocation from a block of upload memory.
    upload_memory_manager: UploadMemoryManager,

    /// Internal manager that contains object pools that track state for live buffer and texture
    /// upload requests.
    request_states: RefCell<RequestStates<C>>,

    /// Internal manager that encapsulates the copy command queue that upload regions are pushed
    /// to before being flushed.
    queue_manager: QueuedCopyManager,

    /// Internal manager that tracks our outstanding command submissions and associated resources.
    submission_manager: SubmissionManager,

    /// Channel to pipe output upload messages into.
    loader_sender: LoaderSender<C>,
}

impl<C: Send + 'static> Drop for AsyncResourceLoader<C> {
    fn drop(&mut self) {
        // Wait on all outstanding submissions to complete on the copy queue and retire them. We
        // need to ensure all resources managed by the loader are not in use on the GPU queue before
        // we destroy them.
        //
        // A completed resource or two could theoretically make if off the sinking ship, so to
        // speak. Completed resources could still be sent to the renderer if one is still listening.
        //
        // We _don't_ flush any queued upload ranges because that would require issuing GPU commands
        // onto the queue. We take the stance that anything not in-flight on the GPU is canceled
        // when the loader is dropped.
        match self.wait_all_submissions() {
            Ok(_) => {}
            Err(RetireError::DeviceLost) => {
                // We can actually ignore device lost! We just wanted to make sure all the
                // resources were no longer in use on the GPU. If we hit device lost then that's
                // exactly what has happened. We shouldn't interact with the device after this so
                // we can just continue on
            }
            Err(RetireError::WaitFailure) => {
                // If we fail to retire the commands with this error code then we've failed to wait
                // on the device, but the device is strictly speaking still alive. We can't legally
                // drop the resources being held live by this loader, but we have no sane recovery
                // path. We can't wait on the device.
                //
                // The next best thing we can do is panic. This should just leak the handles which
                // does keep them alive, technically.
                panic!("Unrecoverable wait_all_submissions error");
            }
            Err(RetireError::RendererDisconnected) => {
                // A closed channel just means nobody's listening, it's perfectly reasonable to
                // ignore this and continue. We don't care if anyone's listening as we just need to
                // close all our in-flight work before destroying the loader.
            }
        }

        // Notify any outstanding request listeners that the requests were canceled. We don't care
        // if anyone is listening, but send the messages in case they are.
        for (_, r) in self.request_states.get_mut().buffers.drain() {
            let _ = self
                .loader_sender
                .try_send(LoaderToRendererMessage::Canceled { cookie: r.cookie });
        }

        // Notify any outstanding request listeners that the requests were canceled. We don't care
        // if anyone is listening, but send the messages in case they are.
        for (_, r) in self.request_states.get_mut().textures.drain() {
            let _ = self
                .loader_sender
                .try_send(LoaderToRendererMessage::Canceled { cookie: r.cookie });
        }
    }
}

impl<C: Send + 'static> AsyncResourceLoader<C> {
    pub fn begin_buffer_load(
        &self,
        size: u64,
        cookie: C,
    ) -> Result<BufferLoadHandle, rhi::BufferCreateError> {
        let rhi_desc = make_standard_buffer_desc(size);
        let buffer = match self.device.create_buffer(&rhi_desc) {
            Ok(v) => v,
            Err(err) => {
                // 'send' only fails if the listener hangs up. If they hang up it doesn't matter
                // if we succeed, there's nobody to receive the message anyway.
                let _ = self
                    .loader_sender
                    .send(LoaderToRendererMessage::Failed { cookie });
                return Err(err);
            }
        };
        let load = BufferLoadState {
            buffer,
            bytes_allocated: 0,
            bytes_submitted: 0,
            bytes_needed: size,
            cookie,
        };
        let handle = self.request_states.borrow_mut().buffers.alloc(load);
        Ok(handle)
    }

    pub fn begin_texture_load<T: SimpleTextureDesc>(
        &self,
        desc: &T,
        cookie: C,
    ) -> Result<TextureLoadHandle, CreateTextureRequestError> {
        let rhi_desc = match make_standard_texture_desc(desc) {
            Some(v) => v,
            None => {
                // 'send' only fails if the listener hangs up. If they hang up it doesn't matter
                // if we succeed, there's nobody to receive the message anyway.
                let _ = self
                    .loader_sender
                    .send(LoaderToRendererMessage::Failed { cookie });
                return Err(CreateTextureRequestError::BadTextureDimensions);
            }
        };
        let texture = match self.device.create_texture(&rhi_desc) {
            Ok(v) => v,
            Err(err) => {
                // 'send' only fails if the listener hangs up. If they hang up it doesn't matter
                // if we succeed, there's nobody to receive the message anyway.
                let _ = self
                    .loader_sender
                    .send(LoaderToRendererMessage::Failed { cookie });
                return Err(CreateTextureRequestError::TextureCreateError(err));
            }
        };
        let load = TextureLoadState {
            texture,
            layout: desc.as_simple_layout(),
            levels: BVec::new_in(system()), // TODO: this
            cookie,
        };
        let handle = self.request_states.borrow_mut().textures.alloc(load);
        Ok(handle)
    }

    pub fn allocate_range_for_buffer_load(
        &self,
        handle: BufferLoadHandle,
        num_bytes: u64,
    ) -> Result<BufferUploadRange<'_, C>, AllocateRangeError> {
        assert_ne!(num_bytes, 0);

        // Clamp 'num_bytes' to the configured 'max_upload_range_size'. Then align the size to a
        // multiple of 4KiB. We align the size to avoid issuing tiny copy commands and maybe help
        // avoid poorly aligned copies.
        let num_bytes = u64::min(num_bytes, self.config.max_upload_range_size as u64);
        let num_bytes = num_bytes.next_multiple_of(4096);

        // Poll for, and retire, any completed submissions before we try and allocate anything.
        // Ideally this should release memory that we can use. This should limit the amount of times
        // we need to stall to satisfy an allocate request.
        //
        // If the renderer has disconnected we immediately bail. It's safe to do so here. If there's
        // no listener to send resources to there's no point letting the caller try and upload more
        // data.
        match self.retire_completed_submissions() {
            Ok(_) => {}
            Err(err) => {
                return match err {
                    RetireError::DeviceLost => Err(AllocateRangeError::DeviceLost),
                    RetireError::WaitFailure => Err(AllocateRangeError::WaitFailure),
                    RetireError::RendererDisconnected => {
                        Err(AllocateRangeError::RendererDisconnected)
                    }
                };
            }
        }

        let mut request_states = self.request_states.borrow_mut();

        let load = request_states
            .buffers
            .get_mut(handle)
            .ok_or(AllocateRangeError::LoadHandleInvalid)?;

        // If there is already an upload range outstanding for this load request then we return an
        // error. This prevents leaving holes in the middle of the resource in failure events.
        //
        // Without this it would be possible to have multiple outstanding upload ranges. If one is
        // dropped or canceled it may leave holes in the resource. Tracking these would be
        // substantially more complex. So we just don't let it happen.
        if load.has_outstanding_range() {
            return Err(AllocateRangeError::OutstandingRange);
        }

        // If we've already submitted all the bytes we need, we should error as the upload is
        // complete.
        if load.is_complete() {
            return Err(AllocateRangeError::UploadComplete);
        }

        // Clamp the size of our allocation to the number of bytes remaining to be uploaded.
        let remaining_bytes = load.remaining_bytes();
        let actual_bytes = u64::min(num_bytes, remaining_bytes);
        let actual_bytes = u64::min(actual_bytes, u32::MAX as u64);

        // Attempt to create a sub-allocated region from our upload-buffer
        let (allocation, data) = self
            .upload_memory_manager
            .allocate_upload_range(actual_bytes as u32)?;

        // Increment the 'allocated' count now that we've successfuly allocated an upload range for
        // the request. Doing this earlier would require fixing up the state on alloc failure.
        load.bytes_allocated += actual_bytes;

        let out = BufferUploadRange {
            loader: self,
            request: handle,
            data,
            allocation,
            dst_offset: load.bytes_submitted,
        };

        Ok(out)
    }

    pub fn allocate_range_for_texture_load(
        &self,
        handle: TextureLoadHandle,
        pitch_align: u32,
        num_bytes: u64,
    ) -> Result<TextureUploadRange<'_, C>, AllocateRangeError> {
        assert_ne!(num_bytes, 0);

        // Clamp 'num_bytes' to the configured 'max_upload_range_size'.
        //
        // We round up to multiples of 4096 to avoid tiny uploads. We shouldn't get poorly aligned
        // uploads like we do for buffers because we upload at row granularity into 512 byte aligned
        // upload ranges.
        let num_bytes = u64::min(num_bytes, self.config.max_upload_range_size as u64);
        let num_bytes = num_bytes.next_multiple_of(4096);

        // Poll for, and retire, any completed submissions before we try and allocate anything.
        // Ideally this should release memory that we can use. This should limit the amount of times
        // we need to stall to satisfy an allocate request.
        //
        // If the renderer has disconnected we immediately bail. It's safe to do so here. If there's
        // no listener to send resources to there's no point letting the caller try and upload more
        // data.
        match self.retire_completed_submissions() {
            Ok(_) => {}
            Err(RetireError::DeviceLost) => return Err(AllocateRangeError::DeviceLost),
            Err(RetireError::WaitFailure) => return Err(AllocateRangeError::WaitFailure),
            Err(RetireError::RendererDisconnected) => {
                return Err(AllocateRangeError::RendererDisconnected);
            }
        }

        let mut request_states = self.request_states.borrow_mut();
        let load = request_states
            .textures
            .get_mut(handle)
            .ok_or(AllocateRangeError::LoadHandleInvalid)?;

        // If there is already an upload range outstanding for this load request then we return an
        // error. This prevents leaving holes in the middle of the resource in failure events.
        //
        // Without this it would be possible to have multiple outstanding upload ranges. If one is
        // dropped or canceled it may leave holes in the resource. Tracking these would be
        // substantially more complex. So we just don't let it happen.
        if load.has_outstanding_range() {
            return Err(AllocateRangeError::OutstandingRange);
        }

        // If we've already submitted all the bytes we need, we should error as the upload is
        // complete.
        if load.is_complete() {
            return Err(AllocateRangeError::UploadComplete);
        }

        // Captures the 'needs_discard' flag early as we will muck up the state we use to detect
        // this flag while setting up the upload range.
        let needs_discard = load.needs_discard();

        // The load state tracks what levels need data. Build a list of levels we can provide data
        // for, using our provided 'num_bytes' as a soft cap for the amount of data we want to
        // upload in this block.
        let wanted = load.build_wanted_rows(
            &self.upload_memory_manager,
            pitch_align,
            num_bytes,
            self.min_image_granularity,
        );

        // If we failed to allocate upload ranges for _any_ mip level then we've hit an OOM case
        // where there's likely no space in the allocator. Return an explicit error for this so
        // the caller can know to stall on the loader to free up memory.
        if wanted.is_empty() {
            return Err(AllocateRangeError::NotEnoughUploadMemory);
        }

        let out = TextureUploadRange {
            loader: self,
            request: handle,
            needs_discard,
            wanted,
        };

        Ok(out)
    }

    pub fn maybe_flush(&self) -> Result<(), FlushError> {
        if self.queue_manager.queued_bytes.get() >= self.config.flush_threshold as u64 {
            // If we've queued over a certain threshold of bytes to be uploaded then we issue a
            // non-blocking upload flush on the loader. This will flush all the queued copy ranges
            // from the internal queue and fire off commands to the GPU to start the uploads to the
            // device resources.
            //
            // This means that in a hot "upload loop" where some thread is driving a resource loader
            // the thread never has to block on the GPU. This means we can always be issuing IO
            // calls and saturating disk IO.
            //
            // We want the GPU working in parallel with the CPU here, where the CPU is spending its
            // time "producing" data from disk, while the GPU is "consuming" data with copy commands
            // into GPU memory. The CPU should never need to wait on the GPU because GPU upload
            // throughput will almost always exceed disk throughput. The expectation is the GPU
            // will consume work faster than the CPU can produce it, so by the time the CPU would
            // ever try and wait on the GPU it should already be finished working.
            self.flush_submitted_uploads()
        } else {
            Ok(())
        }
    }

    pub fn retire_completed_submissions(&self) -> Result<(), RetireError> {
        let mut request_states = self.request_states.borrow_mut();
        let mut live = self.submission_manager.live.borrow_mut();

        // Poll all live submissions, removing and retiring any that we poll as complete.
        let mut is_device_lost = false;
        let mut maybe_retire_failed = Ok(());
        let mut i = 0;
        loop {
            // Loop exit condition. We can't use a 'for' loop as we modify the list while we iterate
            // it.
            if i >= live.len() {
                break;
            }

            // Poll the fence's completion status to determine if the submission is complete.

            let logical_value = match self.device.get_fence_signaled_value(&self.fence) {
                // Successful poll adopts the polled value
                Ok(value) => value,

                // Failure with device lost implies no more live GPU work, so map to u64::MAX and
                // flag the error. u64::MAX will always resolve as signaled
                Err(rhi::FencePollError::DeviceLost) => {
                    is_device_lost = true;
                    u64::MAX
                }

                // Any other failure is fatal, so we bail with 'wait failure'
                Err(_) => {
                    return Err(RetireError::WaitFailure);
                }
            };

            if logical_value >= live[i].signal_value {
                // We use swap_remove and skip incrementing the index to remove while iterating.
                let result = live.swap_remove(i).retire(
                    &mut request_states,
                    &self.upload_memory_manager,
                    &self.loader_sender,
                );

                // We mustn't exit if the renderer has disconnected. We must complete this function
                // completely to ensure the loader remains in a valid state. It's perfectly valid to
                // run the loader with nobody listening.
                if result.is_err() {
                    maybe_retire_failed = result;
                }
                continue;
            }

            // Only increment if we didn't remove a submission. 'i' will already point at the next
            // element to process if we removed one.
            i += 1;
        }

        // 'device lost' errors take precedence
        if is_device_lost {
            Err(RetireError::DeviceLost)
        } else {
            maybe_retire_failed
        }
    }

    pub fn wait_all_submissions(&self) -> Result<(), RetireError> {
        let mut live = self.submission_manager.live.borrow_mut();

        // Use 'wait_fences' to block on all the fences, rather than waiting on them individually.
        let mut fences: BVec<_, MgAsyncLdrSystem> = BVec::new_in(system());
        let mut values: BVec<_, MgAsyncLdrSystem> = BVec::new_in(system());
        for submission in live.iter() {
            fences.push(&self.fence);
            values.push(submission.signal_value);
        }

        let mut is_device_lost = false;
        match self.device.wait_fences(&fences, &values, true, u32::MAX) {
            Ok(rhi::FenceWaitResult::Complete) => {
                // Normal, successful wait
            }
            Err(rhi::FenceWaitError::DeviceLost) => {
                // Device lost also implies the work is complete, however we should signal that we
                // observed a device lost with an error. We can continue without immediately
                // returning.
                is_device_lost = true;
            }
            Ok(_) | Err(_) => {
                return Err(RetireError::WaitFailure);
            }
        }

        // If we've made it this far then all submissions are now complete, so we retire all of them
        // to release their resources.
        let mut retire_maybe_failed = Ok(());
        let mut request_states = self.request_states.borrow_mut();
        for submission in live.drain(..) {
            let result = submission.retire(
                &mut request_states,
                &self.upload_memory_manager,
                &self.loader_sender,
            );

            // We mustn't exit if the renderer has disconnected. We must complete this function
            // completely to ensure the loader remains in a valid state. It's perfectly valid to
            // run the loader with nobody listening.
            if result.is_err() {
                retire_maybe_failed = result;
            }
        }

        // 'device lost' takes precedence over any error the retire operation may have thrown.
        if is_device_lost {
            Err(RetireError::DeviceLost)
        } else {
            retire_maybe_failed
        }
    }

    pub fn flush_submitted_uploads(&self) -> Result<(), FlushError> {
        let mut list = self
            .device
            .create_command_list(&rhi::CommandListDesc {
                queue_type: rhi::QueueType::Transfer,
                name: None,
            })
            .inspect_err(|err| log::error!("{}", err))
            .map_err(|_| FlushError::CommandRecordingFailure)?;

        let request_states = self.request_states.borrow();
        let mut queue = self.queue_manager.queue.borrow_mut();

        // Create our submission bundle that tracks retired resources and completed uploads within
        // the submission so they can be released once the submission is observed complete on the
        // copy queue.
        let mut submission = self.submission_manager.new_submission();

        unsafe {
            let mut encoder = list
                .begin_transfer()
                .inspect_err(|err| log::error!("{}", err))
                .map_err(|_| FlushError::CommandRecordingFailure)?;

            let mut buffer_barriers: BVec<_, MgAsyncLdrSystem> = BVec::new_in(system());
            let mut texture_barriers: BVec<_, MgAsyncLdrSystem> = BVec::new_in(system());

            // Record copy commands for all the submitted upload ranges.
            for block in queue.iter() {
                match block {
                    SubmittedCopy::Buffer(block) => 'proccess: {
                        // It's possible to submit uploads and then cancel the request before
                        // flushing the upload commands. Our simple solution is to just drop the
                        // copy command if the request is no longer valid. This avoids us needing to
                        // flush stale entries from submitted queue eagerly.
                        let load = match request_states.buffers.get_ref(block.request) {
                            None => break 'proccess,
                            Some(v) => v,
                        };

                        // Record the copy command
                        encoder.copy_buffer_regions(
                            &self.upload_memory_manager.buffer,
                            &load.buffer,
                            &[block.region.clone()],
                        );

                        // If we just uploaded the last block...
                        if block.is_final {
                            // Prepare a barrier that performs the release half of a queue
                            // transition that transfers the resource to the general queue.
                            buffer_barriers.push(rhi::BufferBarrier {
                                buffer: Some(&load.buffer),
                                offset: 0,
                                size: u64::MAX,
                                before_sync: rhi::BarrierSync::COPY,
                                after_sync: Default::default(), // Ignored for queue transitions
                                before_access: rhi::BarrierAccess::COPY_WRITE,
                                after_access: Default::default(), // Ignored for queue transitions
                                queue_transition: Some(rhi::QueueTransition {
                                    before_queue: rhi::QueueType::Transfer,
                                    after_queue: rhi::QueueType::General,
                                }),
                            });

                            // And track the request as complete so we signal the renderer once
                            // we observe the submission as complete.
                            submission.completed_uploads.push(block.request.into());
                        }

                        // Extend the lifetime of the resource to live at least as long as this
                        // command submission by stashing a handle to it in the submission
                        // tracker.
                        //
                        // Without this it would be possible to cancel a request and immediately
                        // drop the resource even if it was in use on the GPU.
                        submission.live_resources.push(load.buffer.clone().into());

                        // Push the allocation into the submission bundle so that it will be freed
                        // once the loader observes that the submission is complete on the copy
                        // queue.
                        submission.retired_allocations.push(block.allocation);
                    }
                    SubmittedCopy::Texture(block) => 'process: {
                        // It's possible to submit uploads and then cancel the request before
                        // flushing the upload commands. Our simple solution is to just drop the
                        // copy command if the request is no longer valid. This avoids us needing to
                        // flush stale entries from submitted queue eagerly.
                        let load = match request_states.textures.get_ref(block.request) {
                            None => break 'process,
                            Some(v) => v,
                        };

                        // If this is the first time we're writing to the texture we need to do a
                        // discard operation to initialize the metadata and transition to the
                        // CopyDst layout.
                        if block.needs_discard {
                            let subresource_range = rhi::TextureSubResourceSet::all(
                                self.device.get_texture_desc(&load.texture),
                            );

                            // Issue singular barriers for resource initialization as batching them
                            // is not worth the effort as they're rare. The before scope is empty
                            // so they should be fine.
                            encoder.resource_barrier(
                                &[],
                                &[],
                                &[rhi::TextureBarrier {
                                    texture: Some(&load.texture),
                                    subresource_range,
                                    before_sync: rhi::BarrierSync::NONE,
                                    after_sync: rhi::BarrierSync::COPY,
                                    before_access: rhi::BarrierAccess::NONE,
                                    after_access: rhi::BarrierAccess::COPY_WRITE,
                                    before_layout: rhi::ImageLayout::Undefined,
                                    after_layout: rhi::ImageLayout::CopyDst,
                                    queue_transition: None,
                                }],
                            );
                        }

                        // Record the copy commands
                        encoder.copy_buffer_to_texture(
                            &self.upload_memory_manager.buffer,
                            &load.texture,
                            &block.regions,
                        );

                        // If we just uploaded the last block...
                        if block.is_final {
                            let subresource_range = rhi::TextureSubResourceSet::all(
                                self.device.get_texture_desc(&load.texture),
                            );

                            // TODO: We speculate we'll need 'shader read only' layout on the
                            //       general queue. This may not always be the case. I don't see a
                            //       good way to know the actual needed layout here. We could let
                            //       the render thread handle submitting the ownership transition?
                            //       Perhaps deferred recording to accumulate the real uses + render
                            //       thread kicks off this transition in a cmd buffer it makes
                            //       itself?
                            // Prepare a barrier that performs the release half of a queue
                            // transition that transfers the resource to the general queue.
                            texture_barriers.push(rhi::TextureBarrier {
                                texture: Some(&load.texture),
                                subresource_range,
                                before_sync: rhi::BarrierSync::COPY,
                                after_sync: Default::default(), // Ignored for queue transitions
                                before_access: rhi::BarrierAccess::COPY_WRITE,
                                after_access: Default::default(), // Ignored for queue transitions
                                before_layout: rhi::ImageLayout::CopyDst,
                                after_layout: rhi::ImageLayout::ShaderReadOnly,
                                queue_transition: Some(rhi::QueueTransition {
                                    before_queue: rhi::QueueType::Transfer,
                                    after_queue: rhi::QueueType::General,
                                }),
                            });

                            // And track the request as complete so we signal the renderer once
                            // we observe the submission as complete.
                            submission.completed_uploads.push(block.request.into());
                        }

                        // Extend the lifetime of the resource to live at least as long as this
                        // command submission by stashing a handle to it in the submission
                        // tracker.
                        //
                        // Without this it would be possible to cancel a request and immediately
                        // drop the resource even if it was in use on the GPU.
                        submission.live_resources.push(load.texture.clone().into());

                        // Push the allocations into the submission bundle so that they will be
                        // freed once the loader observes that the submission is complete on the
                        // copy queue.
                        submission
                            .retired_allocations
                            .extend(block.allocations.iter().copied());
                    }
                }
            }

            // Finally after all queued copies are recorded we issue all the barriers we have
            // deferred from the recording loop.
            encoder.resource_barrier(&[], &buffer_barriers, &texture_barriers);

            encoder
                .close()
                .inspect_err(|err| log::error!("{}", err))
                .map_err(|_| FlushError::CommandRecordingFailure)?;
        }

        unsafe {
            // Submit the encoded copy commands to the transfer queue
            self.queue
                .submit(&rhi::QueueSubmitDesc {
                    command_lists: &[Cell::new(Some(list))],
                    wait_fences: &[],
                    wait_values: &[],
                    signal_fences: &[&self.fence],
                    signal_values: &[submission.signal_value],
                    swap_image: None,
                })
                .inspect_err(|err| log::error!("{}", err))
                .map_err(|_| FlushError::CommandRecordingFailure)?;

            // Only after we've successfully submitted our copy commands do we clear the queue. This
            // prevents failures within the command recording loop from leaving the loader in an
            // inconsistent state.
            queue.clear();

            // Clear the 'queued_bytes' tracker now that we've flushed the queue of pending upload
            // work.
            self.queue_manager.queued_bytes.set(0);

            // And then add our submission metadata to our manager.
            //
            // We only issue the submissions after we've cleared all our failure points for the same
            // reason. Any failure in this function should leave the loader in a valid state.
            self.submission_manager.submit(submission);
        }

        // The very last thing we do is poll our in-flight submissions for completion and release
        // any resources we were holding alive for the GPU. We do this last to give the GPU as much
        // time as possible to complete the work before we try and poll.
        match self.retire_completed_submissions() {
            Ok(_) => Ok(()),
            Err(RetireError::DeviceLost) => Err(FlushError::DeviceLost),
            Err(RetireError::WaitFailure) => Err(FlushError::WaitFailure),
            Err(RetireError::RendererDisconnected) => Err(FlushError::RendererDisconnected),
        }
    }
}

impl<C: Send + 'static> AsyncResourceLoader<C> {
    pub(crate) fn new(
        device: AnyArc<dyn rhi::IDevice>,
        queue: AnyArc<dyn rhi::IQueue>,
        fence: rhi::FenceHandle,
        loader_sender: LoaderSender<C>,
        config: AsyncResourceLoaderConfig,
    ) -> Option<Self> {
        let min_image_granularity = queue.queue_properties().min_image_transfer_granularity;

        let upload_memory_manager =
            UploadMemoryManager::new(device.as_ref(), config.upload_block_size)?;

        let request_states = RefCell::new(RequestStates::new());

        let queue_manager = QueuedCopyManager::new();

        let submission_manager = SubmissionManager::new();

        Some(Self {
            config,
            device,
            queue,
            fence,
            min_image_granularity,
            upload_memory_manager,
            request_states,
            queue_manager,
            submission_manager,
            loader_sender,
        })
    }
}

#[derive(Error, Debug)]
pub enum CreateTextureRequestError {
    #[error("The set of non-zero texture dimensions is invalid. Texture is not 1D, 2D or 3D.")]
    BadTextureDimensions,

    /// This error is thrown when trying to create the RHI resource from the device. This
    /// is unrecoverable.
    #[error("Failed to create an RHI texture object.")]
    TextureCreateError(rhi::TextureCreateError),
}

#[derive(Error, Debug)]
pub enum AllocateRangeError {
    /// The caller tried to ask for more memory for a request that doesn't need any more memory.
    #[error("The caller asked for more memory for a completed request.")]
    UploadComplete,

    /// There is already an outstanding upload range active for the given request. Cancel or submit
    /// it before requesting another.
    #[error("There is already an outstanding upload range active for the given request.")]
    OutstandingRange,

    /// The internal upload pool is unable to service the allocation request. It is recommended to
    /// flush the loader's command queue and stall on the GPU to release memory, then try again.
    #[error("The internal upload pool is unable to service the allocation request.")]
    NotEnoughUploadMemory,

    /// There's no such request currently alive within the loader. Has the request been completed or
    /// canceled?
    #[error("There's no such request currently alive within the loader.")]
    LoadHandleInvalid,

    /// This error is thrown when the loader observes a device lost error when trying to wait on
    /// the internal fence. The device resource's lifetimes have ended so we can just destroy them.
    #[error("The GPU device has been lost.")]
    DeviceLost,

    /// This error is thrown when the loader fails to wait on the internal fence for any reason
    /// other than device lost. This is usually fatal and can't easily be cleaned up from because
    /// the device is strictly speaking still alive. You should probably just panic if you observe
    /// this error.
    #[error("The fence wait operation failed for a non-device-lost reason.")]
    WaitFailure,

    /// This error is thrown when trying to send messages to the renderer thread, but the renderer
    /// has disconnected from the channel. The caller is free to decide what to do, but typically
    /// you will want to tear down the loader.
    #[error("The renderer has disconnected from the channel.")]
    RendererDisconnected,
}

#[derive(Error, Debug)]
pub enum FlushError {
    /// This error is thrown when recording device commands into a command buffer fails for any
    /// reason. This could be a fail when creating the command buffer, a failure to submit, and any
    /// other RHI failure along that chain.
    ///
    /// There's really not much you can do in this case other than abort, but we leave that decision
    /// up to the caller.
    #[error("Failed to allocate memory")]
    CommandRecordingFailure,

    /// This error is thrown when the loader observes a device lost error when trying to wait on
    /// the internal fence. The device resource's lifetimes have ended so we can just destroy them.
    #[error("The GPU device has been lost.")]
    DeviceLost,

    /// This error is thrown when the loader fails to wait on the internal fence for any reason
    /// other than device lost. This is usually fatal and can't easily be cleaned up from because
    /// the device is strictly speaking still alive. You should probably just panic if you observe
    /// this error.
    #[error("The fence wait operation failed for a non-device-lost reason.")]
    WaitFailure,

    /// This error is thrown when trying to send messages to the renderer thread, but the renderer
    /// has disconnected from the channel. The caller is free to decide what to do, but typically
    /// you will want to tear down the loader.
    #[error("The renderer has disconnected from the channel.")]
    RendererDisconnected,
}

#[derive(Error, Debug)]
pub enum RetireError {
    /// This error is thrown when the loader observes a device lost error when trying to wait on
    /// the internal fence. The device resource's lifetimes have ended so we can just destroy them.
    #[error("The GPU device has been lost.")]
    DeviceLost,

    /// This error is thrown when the loader fails to wait on the internal fence for any reason
    /// other than device lost. This is usually fatal and can't easily be cleaned up from because
    /// the device is strictly speaking still alive. You should probably just panic if you observe
    /// this error.
    #[error("The fence wait operation failed for a non-device-lost reason.")]
    WaitFailure,

    /// This error is thrown when trying to send messages to the renderer thread, but the renderer
    /// has disconnected from the channel. The caller is free to decide what to do, but typically
    /// you will want to tear down the loader.
    #[error("The renderer has disconnected from the channel.")]
    RendererDisconnected,
}

/// Configuration options that control various knobs for tuning the performance and memory usage
/// of an [`AsyncResourceLoader`]
pub struct AsyncResourceLoaderConfig {
    /// The size of the internal managed upload block, in bytes.
    ///
    /// This allows controling how much memory is permanently reserved by the upload block.
    ///
    /// The default is: 128MiB
    pub upload_block_size: u32,

    /// The maximum size of an upload range, in bytes. Allocation request size parameters are
    /// clamped to this value.
    ///
    /// Generally should be configured with 'upload_block_size'. The max allocation size should be
    /// constrained so that one single upload range can't consume the majority of the upload block.
    /// It's recommended to make this half or less of the upload block size.
    ///
    /// The default is: 48MiB
    pub max_upload_range_size: u32,

    /// How many bytes can be submitted to the loader before the loader will internally force a
    /// flush of the submitted ranges and begin uploading.
    ///
    /// Controls how eager the loader is to flush work to the GPU copy queue. There's some fixed
    /// overhead for each submission so this should not be made too small. Setting this too large
    /// may introduce avoidable latency as it may delay when upload work is launched.
    ///
    /// The default is: 64MiB
    pub flush_threshold: u32,
}

impl Default for AsyncResourceLoaderConfig {
    fn default() -> Self {
        Self {
            upload_block_size: 128 * 1024 * 1024,
            max_upload_range_size: 48 * 1024 * 1024,
            flush_threshold: 64 * 1024 * 1024,
        }
    }
}

const fn assert_send<T: Send>() {}
const _: () = assert_send::<AsyncResourceLoader<()>>();
