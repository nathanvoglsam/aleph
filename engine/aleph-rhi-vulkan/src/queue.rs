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
use std::collections::VecDeque;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Weak};

use aleph_alloc::BVec;
use aleph_alloc::instrumentation::{IAllocationCategory, system};
use aleph_nstr::nstr;
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::{Rhi, RhiRoot, abort_on_unwind, try_clone_value_into_slot};
use ash::vk::{self, Handle};
use parking_lot::Mutex;

use crate::command_list::{CommandList, ListState};
use crate::device::{Device, FreeCommandList};
use crate::fence::Fence;
use crate::internal::allocation_callbacks::GLOBAL;
use crate::internal::error_mapper::map_error_class;
use crate::internal::queue_timing_query_manager::QueueTimingQueryManager;
use crate::internal::semaphore_pool::SemaphorePool;
use crate::internal::unwrap;
use crate::swap_image::{SwapImage, WorkSemaphore};

pub struct Queue {
    pub(crate) _this: Weak<Self>,
    pub(crate) _device: Weak<Device>,
    pub(crate) queue_type: QueueType,
    pub(crate) handle: vk::Queue,

    /// Optional, manages timing queries for the GPU profiling infrastructure.
    pub(crate) timing: Option<QueueTimingQueryManager>,

    /// Properties specific to the queue that need to be known by the implementation to report and
    /// work around.
    pub(crate) info: QueueInfo,

    /// A timeline semaphore that is used for tracking what submissions are in-flight on a GPU
    /// queue. This is used for [Queue::garbage_collect] to determine which submissions are complete
    /// without blocking on the GPU work.
    pub(crate) semaphore: vk::Semaphore,

    /// The index of the most recent submission to the queue.
    ///
    /// Used to track which submissions are in-flight, used in conjunction with
    /// [Queue::last_completed_index].
    pub(crate) last_submitted_index: AtomicU64,

    /// The index of the submission that is most recently confirmed to have completed. Used to track
    /// which submissions are in-flight, used in conjunction with [Queue::last_submitted_index].
    pub(crate) last_completed_index: AtomicU64,

    /// A ring-buffer that tracks all currently in flight queue submissions. This is used in
    /// conjunction with [IQueue::garbage_collect] to track when resources are no longer in use on
    /// the GPU timeline and are safe to destroy.
    pub(crate) in_flight: Mutex<VecDeque<QueueSubmission>>,
}

impl IGetPlatformInterface for Queue {
    unsafe fn __query_platform_interface(&self, target: TypeId, out: *mut ()) -> Option<()> {
        unsafe { try_clone_value_into_slot::<vk::Queue>(&self.handle, out, target) }
    }
}

impl Queue {
    #[inline]
    pub(crate) fn new(
        handle: vk::Queue,
        device: &Device,
        queue_type: QueueType,
        info: QueueInfo,
    ) -> Arc<Self> {
        let semaphore = unsafe {
            let mut info = vk::SemaphoreTypeCreateInfo::default()
                .initial_value(0)
                .semaphore_type(vk::SemaphoreType::TIMELINE);
            let info = vk::SemaphoreCreateInfo::default().push_next(&mut info);
            device.device.create_semaphore(&info, GLOBAL).unwrap()
        };
        if device.calibrated_timestamp.is_some() && info._timestamp_valid_bits == 0 {
            log::warn!(
                "Unable to profile queue '{queue_type}'. Timestamp queries are unsupported on this queue"
            );
        }
        let timing = if device.calibrated_timestamp.is_some() && info._timestamp_valid_bits > 0 {
            log::info!("Attempting to setup timing query manager for queue '{queue_type}'");
            let name = match queue_type {
                QueueType::General => nstr!("GPU General"),
                QueueType::Compute => nstr!("GPU Compute"),
                QueueType::Transfer => nstr!("GPU Transfer"),
            };
            match QueueTimingQueryManager::new(name, device) {
                Some(v) => Some(v),
                None => {
                    log::error!("Failed to setup timing query manager for queue '{queue_type}'");
                    None
                }
            }
        } else {
            None
        };
        Rhi::with(|| {
            Arc::new_cyclic(|v| Self {
                _this: v.clone(),
                _device: device._this.clone(),
                queue_type,
                handle,
                timing,
                info,
                semaphore,
                last_submitted_index: Default::default(),
                last_completed_index: Default::default(),
                in_flight: Default::default(),
            })
        })
    }

    pub(crate) fn next_submission_index(&self) -> u64 {
        // Get the state of last_submitted_index before and after we increment it
        let old_index = self.last_submitted_index.fetch_add(1, Ordering::Relaxed);
        let new_index = old_index + 1;

        // This performs an overflow check, if old_index is u64::MAX then the addition will have
        // caused an overflow and broken our monotonicity requirement.
        assert_ne!(old_index, u64::MAX, "last_submitted_index integer overflow");

        new_index
    }

    pub(crate) fn present_internal(
        &self,
        swap_image: &mut SwapImage,
        presented_from_drop: bool,
    ) -> Result<(), QueuePresentError> {
        // TODO: swap image is unconditionally consumed by this function, but if this function fails
        // there's no way to retry with the same image.
        abort_on_unwind(|| {
            let device = self._device.upgrade().unwrap();
            let loader = device.swapchain.as_ref().unwrap();

            // Checks if the queue supports present operations. While this could use a debug_assert
            // instead like other validation code, the cost of this check compared to the cost of
            // the present call is tiny.
            if !swap_image
                .swap_chain
                .present_supported_on_queue(self.queue_type)
            {
                return Err(QueuePresentError::QueuePresentationNotSupported(
                    self.queue_type,
                ));
            }

            let is_unused = swap_image.ready_semaphore.get_mut() != &0;
            if is_unused {
                // This exists for two reasons:
                // 1. To ensure that the swap image has a valid layout before presenting.
                // 2. To handle the case where the swap image was never used in a submission,
                //    preventing potential synchronization issues.
                //
                // The need for the layout transition is obvious: the swap image must be in a valid
                // layout before it can be presented.
                //
                // The second reason is more subtle: if the image was never used in a submission,
                // the 'ready semaphore' that was used with the 'acquire' operation will be unused
                // and so the present operation won't sync with the acquire.
                //
                // We can kill several birds with one stone by issuing a dummy layout transition for
                // the unused swap image, ensuring it is in a valid layout, and because we use the
                // RHI machinery to issue the work we ensure the ready semaphore is consumed and
                // we get a proper chain of synchronization from acquire -> work -> present.
                //
                // This also prevents the ready semaphore from being leaked, as without a submission
                // consuming the semaphore nothing will return it to the pool it came from.
                if presented_from_drop {
                    // However, we do need to recover if the swap image was unused and _dropped_.
                    //
                    // In this case we do issue the dummy layout transition.
                    //
                    // If we did this in general, it's possible we could issue the transition and
                    // then return from this function because the present operation itself fails.
                    // That would trash the image, and the client would have no way to recover.
                    //
                    // We only do this if the swap image was dropped because it's not legal for the
                    // client to access the image after dropping it.
                    unsafe {
                        self.issue_dummy_layout_transition_for_used_swap_image(swap_image)?;
                    }
                } else {
                    // Strictly speaking, if the swap image was never associated with a queue
                    // presentation then it's not legal for any GPU commands to have accessed it.
                    //
                    // This means nobody could have transitioned the image to the PresentSrc layout,
                    // meaning it's not legal for it to be presented.
                    //
                    // Rather than try and recover internally we just panic (which will abort)
                    // because the complexity of trying to recover from this situation is high.
                    panic!("It is illegal to present an unused swap image.");
                }
            } else {
                // If the swap image _has_, in fact, been used in a queue submission at some point
                // then we need to handle the 'presented_from_drop' case separately.
                //
                // We can't just sync with the image acquire operation, we need to synchronize with
                // all of the work that has been submitted with this image. The operation we need to
                // do is the same, a transition from undefined -> presentsrc to guarantee the image
                // is in the PresentSrc layout, but we need to synchronize differently.
                if presented_from_drop {
                    unsafe {
                        self.issue_dummy_layout_transition_for_used_swap_image(swap_image)?;
                    }
                }
            }

            let swap_chain = &swap_image.swap_chain;
            let swap_chain = swap_chain.inner.lock();

            let swapchains = [swap_chain.swap_chain];
            let image_indices = [swap_image.index];
            let mut wait_semaphores: BVec<_, RhiRoot> = BVec::new_in(system());
            wait_semaphores.extend(
                swap_image
                    .work_semaphores
                    .get_mut()
                    .iter()
                    .map(|v| v.handle),
            );
            let info = vk::PresentInfoKHR::default()
                .wait_semaphores(&wait_semaphores)
                .swapchains(&swapchains)
                .image_indices(&image_indices);

            let result = unsafe {
                let _lock = self.in_flight.lock();
                loader.queue_present(self.handle, &info)
            };

            match result {
                Ok(sub_optimal) => {
                    // The semaphores have been consumed, return them to the semaphore pool.
                    //
                    // We don't need to wait on the present operation like with a queue submission
                    // because the pool is specific to the image index. We implicitly sync with the
                    // present operation because the image acquire will never give us this same
                    // image index again until the present op is retied.
                    let _ = swap_image.texture.take();
                    let work_semaphores = swap_image.work_semaphores.get_mut().drain(..);
                    for semaphore in work_semaphores {
                        swap_image
                            .work_semaphore_pool
                            .push_to_recycle_queue(semaphore.handle);
                    }

                    if sub_optimal {
                        Err(QueuePresentError::SubOptimal)
                    } else {
                        Ok(())
                    }
                }
                Err(vk::Result::ERROR_DEVICE_LOST) => {
                    // Device lost means the semaphores are left in an undefined state. We will
                    // defer destruction to be managed by the queue via normal garbage collection.
                    let _ = swap_image.texture.take();
                    let work_semaphores = swap_image.work_semaphores.get_mut().drain(..);
                    defer_abandoned_semapore_destruction(&device, work_semaphores);
                    Err(QueuePresentError::DeviceLost)
                }
                Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => {
                    // The semaphores have been consumed, return them to the semaphore pool.
                    let _ = swap_image.texture.take();
                    let work_semaphores = swap_image.work_semaphores.get_mut().drain(..);
                    for semaphore in work_semaphores {
                        swap_image
                            .work_semaphore_pool
                            .push_to_recycle_queue(semaphore.handle);
                    }
                    Err(QueuePresentError::OutOfDate)
                }
                // We don't enable the extension for this error to be possible to return. If we do
                // enable it, we should do this.
                // Err(vk::Result::ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT) => {
                //     // The semaphores have been consumed, return them to the semaphore pool.
                //     let _ = swap_image.texture.take();
                //     let work_semaphores = swap_image.work_semaphores.get_mut().drain(..);
                //     for semaphore in work_semaphores {
                //         swap_image.semaphore_pool.push(semaphore.handle);
                //     }
                //     Err(QueuePresentError::Platform)
                // }
                Err(vk::Result::ERROR_SURFACE_LOST_KHR) => {
                    // The semaphores have been consumed, return them to the semaphore pool.
                    let _ = swap_image.texture.take();
                    let work_semaphores = swap_image.work_semaphores.get_mut().drain(..);
                    for semaphore in work_semaphores {
                        swap_image
                            .work_semaphore_pool
                            .push_to_recycle_queue(semaphore.handle);
                    }
                    Err(QueuePresentError::SurfaceLost)
                }
                Err(vk::Result::ERROR_OUT_OF_HOST_MEMORY) => {
                    // The semaphores have _not_ been consumed, so we need to destroy them. We defer
                    // destruction to the queue via normal garbage collection.
                    Err(QueuePresentError::Platform)
                }
                Err(vk::Result::ERROR_OUT_OF_DEVICE_MEMORY) => {
                    // The semaphores have _not_ been consumed, so we need to destroy them. We defer
                    // destruction to the queue via normal garbage collection.
                    Err(QueuePresentError::Platform)
                }
                Err(e) => abort_on_unwind(|| {
                    log::error!("Platform Error: {:#?}", e);
                    panic!("Unrecoverable presentation error")
                }),
            }
        })
    }

    pub(crate) unsafe fn issue_dummy_layout_transition_for_used_swap_image(
        &self,
        swap_image: &mut SwapImage,
    ) -> Result<(), QueuePresentError> {
        abort_on_unwind(|| {
            let device = self._device.upgrade().unwrap();

            let desc = CommandListDesc {
                queue_type: self.queue_type,
                name: None,
            };
            let list = match device.create_command_list(&desc) {
                Ok(list) => list,
                Err(CommandListCreateError::NoSuchQueue(_)) => unreachable!(),
                Err(_) => panic!("Failed to create command list"),
            };
            let mut list = {
                if list.as_ref().type_id() == TypeId::of::<CommandList>() {
                    let ptr = Box::into_raw(list);
                    unsafe { Box::from_raw(ptr.cast::<CommandList>()) }
                } else {
                    panic!("Unknown ICommandList implementation")
                }
            };

            let texture = swap_image.texture();
            let barrier = TextureBarrier {
                texture: Some(texture),
                subresource_range: TextureSubResourceSet::all(device.get_texture_desc(texture)),
                before_sync: BarrierSync::ALL,
                after_sync: BarrierSync::ALL,
                before_access: BarrierAccess::NONE,
                after_access: BarrierAccess::NONE,
                before_layout: ImageLayout::Undefined,
                after_layout: ImageLayout::PresentSrc,
                queue_transition: None,
            };
            let name = nstr!("present-unused-image-transition");

            match self.queue_type {
                QueueType::General => {
                    let mut encoder = match list.begin_general() {
                        Ok(v) => v,
                        Err(_) => return Err(QueuePresentError::Platform),
                    };
                    unsafe {
                        let mut encoder = encoder.begin_transfer(name);
                        encoder.resource_barrier(&[], &[], &[barrier]);
                        drop(encoder);
                    }
                }
                QueueType::Compute => {
                    let mut encoder = match list.begin_compute() {
                        Ok(v) => v,
                        Err(_) => return Err(QueuePresentError::Platform),
                    };
                    unsafe {
                        let mut encoder = encoder.begin_transfer(name);
                        encoder.resource_barrier(&[], &[], &[barrier]);
                        drop(encoder);
                    }
                }
                QueueType::Transfer => {
                    let mut encoder = match list.begin_transfer() {
                        Ok(v) => v,
                        Err(_) => return Err(QueuePresentError::Platform),
                    };
                    unsafe {
                        let mut encoder = encoder.begin_transfer(name);
                        encoder.resource_barrier(&[], &[], &[barrier]);
                        drop(encoder);
                    }
                }
            }

            unsafe {
                // These semaphores are associated with queue submissions that _must_ be waited on
                // before presenting the swap image.
                //
                // Or, in this case, before we stomp it with an undefined -> present src layout
                // transition to make sure it's ready to present on the current queue.
                let mut work_semaphores = BVec::new_in(RhiRoot::default());
                work_semaphores.extend(swap_image.work_semaphores.get_mut().drain(..));

                let mut submission = QueueSubmission {
                    swap_ready_semaphore: vk::Semaphore::null(),
                    swap_work_semaphores: BVec::new_in(RhiRoot::default()),
                    swap_work_semaphore_pool: None,
                    abandoned_semaphores: BVec::new_in(RhiRoot::default()),
                    index: 0,
                    lists: BVec::new_in(RhiRoot::default()),
                };

                let mut manager = SubmissionManager::default();
                manager
                    .handle_command_lists_unwrapped([list].into_iter(), &mut submission)
                    .expect("Internal command submission failure");
                manager.handle_internal_semaphores(self, &mut submission);
                manager.handle_swap_semaphores(swap_image, self, &mut submission);

                // We want to consume and wait on all the work semaphores associated with this swap
                // image before submitting the queue. Normally these would've been waited on
                // directly by the present operation, but we're in a fallback path that needs to do
                // some work on a queue before issuing a present.
                //
                // Waiting on all the work semaphores ensures all accesses to the swap image are
                // retired before we do our fallback crimes.
                //
                // What should happen is that once this loop is complete, swap_image.work_semaphores
                // will contain only 1 semaphore, which was placed there by
                // 'handle_swap_semaphores'. We have _taken_ the old set of work semaphores and we
                // are adding them so the queue submission waits on them instead of the present
                // operation they were intended to wait on.
                for work_semaphore in work_semaphores.iter() {
                    manager.wait_on_binary_semaphore(work_semaphore.handle);
                }

                let mut timeline_info = manager.timeline_info();
                let info = manager.submit_info(&mut timeline_info);

                let mut in_flight = self.in_flight.lock();
                let submit_result =
                    device
                        .device
                        .queue_submit(self.handle, &[info], vk::Fence::null());

                let out = if let Err(err) = submit_result {
                    match err {
                        vk::Result::ERROR_DEVICE_LOST => {
                            // Make sure we don't leak the work semaphores if the device is lost.
                            submission
                                .abandoned_semaphores
                                .extend(work_semaphores.into_iter().map(|s| s.handle));
                            Err(QueuePresentError::DeviceLost)
                        }
                        _ => {
                            panic!("Fatal platform error: {:#?}", err);
                        }
                    }
                } else {
                    // The semaphores have been consumed, but we can't reuse them until the
                    // submission is complete. This defers the recycling of the semaphores until the
                    // submission has finished.
                    submission
                        .swap_work_semaphores
                        .extend(work_semaphores.into_iter().map(|s| s.handle));

                    Ok(())
                };

                Rhi::with(|| in_flight.push_back(submission));

                out
            }
        })
    }
}

fn defer_abandoned_semapore_destruction(
    device: &Device,
    work_semaphores: impl Iterator<Item = WorkSemaphore>,
) {
    for semaphore in work_semaphores {
        let queue = match device.get_queue_internal(semaphore.queue_type) {
            Some(v) => v,
            None => unreachable!(),
        };
        let mut in_flight = queue.in_flight.lock();
        let mut abandoned_semaphores = BVec::new_in(RhiRoot::default());
        abandoned_semaphores.push(semaphore.handle);
        let submission = QueueSubmission {
            swap_ready_semaphore: Default::default(),
            swap_work_semaphores: BVec::new_in(RhiRoot::default()),
            swap_work_semaphore_pool: None,
            abandoned_semaphores,
            index: semaphore.submission_index,
            lists: BVec::new_in(RhiRoot::default()),
        };
        Rhi::with(|| in_flight.push_back(submission));
    }
}

impl IQueue for Queue {
    fn upgrade(&self) -> Arc<dyn IQueue> {
        abort_on_unwind(|| self._this.upgrade().unwrap())
    }

    fn strong_count(&self) -> usize {
        self._this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self._this.weak_count()
    }

    fn get_queue_type(&self) -> QueueType {
        abort_on_unwind(|| self.queue_type)
    }

    fn queue_properties(&self) -> QueueProperties {
        abort_on_unwind(|| {
            let v = self.info.min_image_transfer_granularity;
            let min_image_transfer_granularity = Extent3D::new(v.width, v.height, v.depth);

            QueueProperties {
                min_image_transfer_granularity,
            }
        })
    }

    fn garbage_collect(&self) -> Result<(), QueueGarbageCollectError> {
        abort_on_unwind(|| {
            let device = self._device.upgrade().unwrap();

            // Grab the index of the most recently completed command list on this queue and update
            // the queue's value
            //
            // Like in 'wait_idle' we need an atomic CAS loop to uphold monotonicity guarantees.
            // There is a window between the GetCompletedValue call and the atomic store for thread
            // preemption to allow another thread to write in a newer 'GetCompletedValue' with a
            // higher index before the initial thread gets a chance to write its lower index.
            // Eventually the initial thread will get execution back and overwrite the higher index
            // with the lower index it captured before being preempted.
            //
            // Atomics are 'fun'.
            let last_completed = loop {
                let old_last_completed = self.last_completed_index.load(Ordering::Relaxed);
                let new_last_completed = unsafe {
                    device
                        .device
                        .get_semaphore_counter_value(self.semaphore)
                        .inspect_err(|v| log::error!("Platform Error: {:?}", v))
                        .map_err(map_error_class)
                };
                match new_last_completed {
                    // Got the value? Hit our CAS loop
                    Ok(v) => match self.last_completed_index.compare_exchange(
                        old_last_completed,
                        v,
                        Ordering::Relaxed,
                        Ordering::Relaxed,
                    ) {
                        Ok(_) => break v,
                        Err(_) => continue,
                    },

                    // Device lost? Everything is complete on the queue so we can use u64::MAX as
                    // our completed value.
                    Err(QueueGarbageCollectError::DeviceLost) => break u64::MAX,

                    // Any other error condition is unrecoverable so we return the error to the
                    // caller.
                    Err(e) => return Err(e),
                }
            };

            // Capture the current length of the queue. We then pop N items off the queue and check
            // to see if it is complete based on comparing the list's index with the last completed
            // index. If the list is done we drop it to release any resources that it was keeping
            // alive.
            let mut retired = Vec::new();
            Rhi::with(|| {
                let mut in_flight = self.in_flight.lock();
                for _ in 0..in_flight.len() {
                    let submission = abort_on_unwind(|| in_flight.pop_front().unwrap());

                    // If this submission is not yet complete, return it to the queue and check the
                    // next entry
                    if submission.index > last_completed {
                        in_flight.push_back(submission);
                        continue;
                    }

                    // Otherwise push the submission into the 'retired' set so we can process it
                    // outside the critical section.
                    retired.push(submission);
                }
            });

            for mut submission in retired {
                // Now that we know the submission is complete we can return the swap semaphore
                // (if there is one) to the semaphore pool
                if !submission.swap_ready_semaphore.is_null() {
                    let device = self._device.upgrade().unwrap();
                    device
                        .swap_semaphore_pool
                        .push_immediately(submission.swap_ready_semaphore);
                }

                if let Some(pool) = submission.swap_work_semaphore_pool.as_deref() {
                    for semaphore in submission.swap_work_semaphores {
                        pool.push_to_recycle_queue(semaphore);
                    }
                } else {
                    // If for some reason there is no pool for the swap work semaphores, we just
                    // destroy them instead.
                    for semaphore in submission.swap_work_semaphores {
                        unsafe {
                            device.device.destroy_semaphore(semaphore, GLOBAL);
                        }
                    }
                }

                if let Some(timing) = self.timing.as_ref() {
                    unsafe {
                        for list in submission.lists.iter_mut() {
                            for block in list.retired_timing_blocks.iter() {
                                block.block.notify_profiler(timing, block.count);
                            }
                            timing.return_blocks(
                                list.retired_timing_blocks.drain(..).map(|v| v.block),
                            );
                        }
                    }
                }

                // Grab the pool for the specific queue type. Don't want to get different
                // classes of command list mixed up!
                let pool_target = device
                    .command_list_pool
                    .get_pool_for_queue_type(self.queue_type);

                for mut list in submission.lists.into_iter() {
                    debug_assert_eq!(list.list_type, self.queue_type);

                    // Take the pool and buffer out of the CommandList object so they don't
                    // get dropped. We destroy the Box<CommandList> because it also contains
                    // a back reference to device.
                    //
                    // If we store it inside device then we create a reference cycle and we'll
                    // leak Device. Not great...
                    let list = FreeCommandList {
                        pool: std::mem::take(&mut list.pool),
                        buffer: std::mem::take(&mut list.buffer),
                        list_type: list.list_type,
                    };

                    // If we fill the list we just start destroying command lists rather than
                    // growing the pool.
                    if let Err(dropped) = pool_target.push(list) {
                        unsafe {
                            log::warn!("'command_list_pool' overflowing '{}'.", self.queue_type);
                            dropped.collect(&device);
                        }
                    }
                }

                for semaphore in submission.abandoned_semaphores {
                    unsafe {
                        device.device.destroy_semaphore(semaphore, GLOBAL);
                    }
                }
            }

            if let Some(timing) = self.timing.as_ref() {
                timing.issue_calibration(&device)
            }

            Ok(())
        })
    }

    fn wait_idle(&self) -> Result<(), QueueWaitError> {
        abort_on_unwind(|| {
            let device = self._device.upgrade().unwrap();

            unsafe {
                let _lock = self.in_flight.lock();
                device
                    .device
                    .queue_wait_idle(self.handle)
                    .inspect_err(|v| log::error!("Platform Error: {:?}", v))
                    .map_err(map_error_class)
            }
        })
    }

    unsafe fn submit(&self, mut desc: QueueSubmitDesc) -> Result<(), QueueSubmitError> {
        abort_on_unwind(|| {
            let device = self._device.upgrade().unwrap();

            let mut manager = SubmissionManager::default();
            let submission = manager.prepare_submission(self, &mut desc)?;

            let mut timeline_info = manager.timeline_info();
            let info = manager.submit_info(&mut timeline_info);

            unsafe {
                let mut in_flight = self.in_flight.lock();
                let result = device
                    .device
                    .queue_submit(self.handle, &[info], vk::Fence::null());

                if let Err(err) = result {
                    return match err {
                        vk::Result::ERROR_DEVICE_LOST => Err(QueueSubmitError::DeviceLost),
                        _ => {
                            log::error!("Platform Error: {:#?}", err);
                            Err(QueueSubmitError::Platform)
                        }
                    };
                }

                Rhi::with(|| in_flight.push_back(submission));
            }

            Ok(())
        })
    }

    unsafe fn present(&self, swap_image: Arc<dyn ISwapImage>) -> Result<(), QueuePresentError> {
        abort_on_unwind(|| {
            let mut swap_image = {
                let v = swap_image;
                unwrap::swap_image_owned(v)
            };
            let swap_image = Arc::get_mut(&mut swap_image).unwrap();

            self.present_internal(swap_image, false)
        })
    }
}

#[derive(Clone)]
pub struct QueueInfo {
    pub family_index: u32,
    pub queue_index: u32,
    pub min_image_transfer_granularity: vk::Extent3D,
    pub _timestamp_valid_bits: u32,
    pub _sparse_binding: bool,
}

impl QueueInfo {
    pub fn new(family_index: u32, queue_index: u32, family: &vk::QueueFamilyProperties) -> Self {
        Self {
            family_index,
            queue_index,
            min_image_transfer_granularity: family.min_image_transfer_granularity,
            _timestamp_valid_bits: family.timestamp_valid_bits,
            _sparse_binding: family.queue_flags.contains(vk::QueueFlags::SPARSE_BINDING),
        }
    }
}

pub struct QueueSubmission {
    /// The (optional) swap image semaphore that was attached to this queue submission. This
    /// semaphore will be waited on by the queue submission, and will be signalled by an image
    /// acquire operation.
    pub swap_ready_semaphore: vk::Semaphore,

    /// The (optional) swap image semaphore that was attached to this queue submission. Once the
    /// semaphore is known to be signalled we canreturn it to the device's semaphore pool for reuse.
    ///
    /// This semaphore will be signalled when the queue submission is complete.
    pub swap_work_semaphores: BVec<vk::Semaphore, RhiRoot>,

    /// Handle to the semaphore pool we have to place the 'work' semaphores into. These will be
    /// waited by a present call which has some very inconvenient properties so they must be handled
    /// separately than the ready semaphores.
    pub swap_work_semaphore_pool: Option<Arc<SemaphorePool>>,

    /// These are internal _binary_ semaphores that have been abandoned because of error conditions
    /// leaving them without a way to be waited on. Abandoned semaphores that were signalled by the
    /// associated submission index are added here, and should be destroyed when the submission is
    /// retired.
    ///
    /// These can not be recycled.
    pub abandoned_semaphores: BVec<vk::Semaphore, RhiRoot>,

    /// The index of the queue submission. Used for tracking when the work has been retired
    pub index: u64,

    /// We separate the command lists in the submission into their own list so they can be easily
    /// filtered out and recycled later
    pub lists: BVec<Box<CommandList>, RhiRoot>,
}

pub struct SubmissionManager {
    pub wait_semaphores: BVec<vk::Semaphore, RhiRoot>,
    pub wait_values: BVec<u64, RhiRoot>,
    pub wait_dst_stage_masks: BVec<vk::PipelineStageFlags, RhiRoot>,
    pub signal_semaphores: BVec<vk::Semaphore, RhiRoot>,
    pub signal_values: BVec<u64, RhiRoot>,
    pub command_buffers: BVec<vk::CommandBuffer, RhiRoot>,
}

impl<'a> Default for SubmissionManager {
    fn default() -> Self {
        Self {
            wait_semaphores: BVec::new_in(RhiRoot::default()),
            wait_values: BVec::new_in(RhiRoot::default()),
            wait_dst_stage_masks: BVec::new_in(RhiRoot::default()),
            signal_semaphores: BVec::new_in(RhiRoot::default()),
            signal_values: BVec::new_in(RhiRoot::default()),
            command_buffers: BVec::new_in(RhiRoot::default()),
        }
    }
}

impl SubmissionManager {
    pub fn prepare_submission(
        &mut self,
        queue: &Queue,
        desc: &mut QueueSubmitDesc,
    ) -> Result<QueueSubmission, QueueSubmitError> {
        assert_eq!(desc.wait_fences.len(), desc.wait_values.len());
        assert_eq!(desc.signal_fences.len(), desc.signal_values.len());

        // Reserve space for 1 semaphore for each caller provided 'wait_semaphore'. We add space for
        // one extra semaphore if the caller is attaching a swap image so that we can wait for the
        // image acquisition operation.
        let caller_wait_num = desc.wait_fences.len();
        let swap_wait_num = if desc.swap_image.is_some() { 1 } else { 0 };
        let wait_num = caller_wait_num + swap_wait_num;

        // Reserve space for 1 semaphore for each caller provided 'signal_semaphore'. We also add
        // space for an extra internal timeline semaphore that is used by the queue to track what
        // command lists have completed. A further entry is reserved if a swap image is attached to
        // signal a semaphore that the present operation will wait on.
        let caller_signal_num = desc.signal_fences.len();
        let internal_signal_num = 1;
        let swap_signal_num = if desc.swap_image.is_some() { 1 } else { 0 };
        let signal_num = caller_signal_num + internal_signal_num + swap_signal_num;

        self.wait_semaphores = BVec::with_capacity_in(wait_num, RhiRoot::default());
        self.wait_values = BVec::with_capacity_in(wait_num, RhiRoot::default());
        self.wait_dst_stage_masks = BVec::with_capacity_in(wait_num, RhiRoot::default());
        self.signal_semaphores = BVec::with_capacity_in(signal_num, RhiRoot::default());
        self.signal_values = BVec::with_capacity_in(signal_num, RhiRoot::default());
        self.command_buffers = BVec::with_capacity_in(desc.command_lists.len(), RhiRoot::default());

        let mut submission = QueueSubmission {
            swap_ready_semaphore: vk::Semaphore::null(),
            swap_work_semaphores: BVec::new_in(RhiRoot::default()),
            swap_work_semaphore_pool: None,
            abandoned_semaphores: BVec::new_in(RhiRoot::default()),
            index: 0,
            lists: BVec::with_capacity_in(desc.command_lists.len(), RhiRoot::default()),
        };

        self.handle_command_lists(desc, &mut submission)?;
        self.handle_api_fences(desc);
        self.handle_internal_semaphores(queue, &mut submission);
        if let Some(swap_image) = desc.swap_image.map(unwrap::swap_image) {
            self.handle_swap_semaphores(swap_image, queue, &mut submission);
        }

        Ok(submission)
    }

    pub fn timeline_info(&self) -> vk::TimelineSemaphoreSubmitInfo<'_> {
        vk::TimelineSemaphoreSubmitInfo::default()
            .wait_semaphore_values(&self.wait_values)
            .signal_semaphore_values(&self.signal_values)
    }

    pub fn submit_info<'b>(
        &'b self,
        timeline_info: &'b mut vk::TimelineSemaphoreSubmitInfo,
    ) -> vk::SubmitInfo<'b> {
        vk::SubmitInfo::default()
            .wait_semaphores(&self.wait_semaphores)
            .wait_dst_stage_mask(&self.wait_dst_stage_masks)
            .signal_semaphores(&self.signal_semaphores)
            .command_buffers(&self.command_buffers)
            .push_next(timeline_info)
    }

    /// Handles adding all the command lists provided in the submission desc into both the
    /// submission tracker object as well as the internal 'vk::CommandBuffer' list that gets passed
    /// to the vk api call.
    fn handle_command_lists<'b>(
        &mut self,
        desc: &mut QueueSubmitDesc,
        submission: &mut QueueSubmission,
    ) -> Result<(), QueueSubmitError> {
        let lists = desc.command_lists.iter_mut().map(|list| {
            let list = list.take().unwrap();
            let list = {
                if list.as_ref().type_id() == TypeId::of::<CommandList>() {
                    let ptr = Box::into_raw(list);
                    unsafe { Box::from_raw(ptr.cast::<CommandList>()) }
                } else {
                    panic!("Unknown ICommandList implementation")
                }
            };
            list
        });
        self.handle_command_lists_unwrapped(lists, submission)
    }

    /// Handles adding all the command lists provided in the submission desc into both the
    /// submission tracker object as well as the internal 'vk::CommandBuffer' list that gets passed
    /// to the vk api call.
    fn handle_command_lists_unwrapped<'b>(
        &mut self,
        lists: impl Iterator<Item = Box<CommandList>>,
        submission: &mut QueueSubmission,
    ) -> Result<(), QueueSubmitError> {
        for list in lists {
            self.command_buffers.push(list.buffer);
            submission.lists.push(list);
        }

        // Make sure all the lists are in the correct state for submission
        for list in submission.lists.iter() {
            if list.state != ListState::Closed {
                return Err(QueueSubmitError::InvalidCommandListState);
            }
        }

        Ok(())
    }

    /// This function will add all API level fences that are provided from the caller. These
    /// are handled differently from the internal semaphores that are used for synchronizing inside
    /// the API implementation.
    fn handle_api_fences(&mut self, desc: &QueueSubmitDesc) {
        let iter = desc
            .wait_fences
            .iter()
            .zip(desc.wait_values.iter().copied());
        for (fence, value) in iter {
            let fence = Fence::get(fence);
            self.wait_on_timeline_semaphore(fence.semaphore, value);
        }

        let iter = desc
            .signal_fences
            .iter()
            .zip(desc.signal_values.iter().copied());
        for (fence, value) in iter {
            let fence = Fence::get(fence);
            self.signal_timeline_semaphore(fence.semaphore, value);
        }
    }

    /// This function will handle the queue's internal semaphores. This is, specifically, a timeline
    /// semaphore that is used to track which submissions are complete on the CPU. This is always
    /// inserted into every submission and is used by the 'garbage_collect' calls to know what GPU
    /// work has completed.
    fn handle_internal_semaphores(&mut self, queue: &Queue, submission: &mut QueueSubmission) {
        submission.index = queue.next_submission_index();
        self.signal_timeline_semaphore(queue.semaphore, submission.index);
    }

    /// This function handles any semaphores the swap chain system may need to insert. These are not
    /// always inserted as not all submissions will be associated with a swap image.
    fn handle_swap_semaphores(
        &mut self,
        swap_image: &SwapImage,
        queue: &Queue,
        submission: &mut QueueSubmission,
    ) {
        let device = &swap_image.swap_chain.device.device;

        let mut wait_semaphores = swap_image.work_semaphores.lock();

        // We take the ready semaphore from the swap image, leaving a null semaphore in its
        // place. This way only the first submission on a queue will wait on the semaphore and
        // we don't end up waiting multiple times, which is not correct.
        submission.swap_ready_semaphore = swap_image.take_ready_semaphore();
        if !submission.swap_ready_semaphore.is_null() {
            self.wait_on_binary_semaphore(submission.swap_ready_semaphore);
        }

        let swap_work_semaphore = unsafe { swap_image.work_semaphore_pool.get(device) };
        self.signal_binary_semaphore(swap_work_semaphore);
        wait_semaphores.push(WorkSemaphore {
            handle: swap_work_semaphore,
            queue_type: queue.queue_type,
            submission_index: submission.index,
        });
    }

    fn wait_on_binary_semaphore(&mut self, semaphore: vk::Semaphore) {
        self.wait_semaphores.push(semaphore);
        self.wait_values.push(0);
        self.wait_dst_stage_masks
            .push(vk::PipelineStageFlags::ALL_COMMANDS);
    }

    fn wait_on_timeline_semaphore(&mut self, semaphore: vk::Semaphore, value: u64) {
        self.wait_semaphores.push(semaphore);
        self.wait_values.push(value);
        self.wait_dst_stage_masks
            .push(vk::PipelineStageFlags::ALL_COMMANDS);
    }

    fn signal_binary_semaphore(&mut self, semaphore: vk::Semaphore) {
        self.signal_semaphores.push(semaphore);
        self.signal_values.push(0);
    }

    fn signal_timeline_semaphore(&mut self, semaphore: vk::Semaphore, value: u64) {
        self.signal_semaphores.push(semaphore);
        self.signal_values.push(value);
    }
}
