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
use std::mem::transmute;
use std::ptr;
use std::ptr::NonNull;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use aleph_alloc::BVec;
use aleph_any::{AnyArc, AnyWeak, IAny, TraitObject, box_downcast};
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::{RhiSystem, try_clone_value_into_slot};
use ash::vk::{self, Handle};
use crossbeam::queue::ArrayQueue;
use parking_lot::Mutex;

use crate::command_list::{CommandList, ListState};
use crate::device::{Device, FreeCommandList};
use crate::fence::Fence;
use crate::internal::allocation_callbacks::GLOBAL;
use crate::internal::semaphore_pool::SemaphorePool;
use crate::internal::unwrap;
use crate::semaphore::Semaphore;
use crate::swap_image::SwapImage;

pub struct Queue {
    pub(crate) _this: AnyWeak<Self>,
    pub(crate) _device: AnyWeak<Device>,
    pub(crate) queue_type: QueueType,
    pub(crate) handle: vk::Queue,

    /// Properties specific to the queue that need to be known by the implementation to report and
    /// work around.
    pub(crate) info: QueueInfo,

    /// Lock used to serialize submissions to the command queue.
    pub(crate) submit_lock: Mutex<()>,

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
    pub(crate) in_flight: ArrayQueue<QueueSubmission>,
}

// Unwrapped declare_interfaces as we need to inject a custom condition for returning IQueueDebug
impl IAny for Queue {
    #[allow(bare_trait_objects)]
    fn __query_interface(&self, target: TypeId) -> Option<TraitObject<'_>> {
        unsafe {
            if target == TypeId::of::<dyn IQueue>() {
                return Some(transmute(self as &dyn IQueue));
            }
            if target == TypeId::of::<dyn IAny>() {
                return Some(transmute(self as &dyn IAny));
            }
        }
        unsafe {
            if target == TypeId::of::<Queue>() {
                Some(TraitObject {
                    data: NonNull::new_unchecked(self as *const _ as *mut ()),
                    vtable: ptr::null_mut(),
                    phantom: Default::default(),
                })
            } else {
                None
            }
        }
    }
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
    ) -> AnyArc<Self> {
        let semaphore = unsafe {
            let mut info = vk::SemaphoreTypeCreateInfo::default()
                .initial_value(0)
                .semaphore_type(vk::SemaphoreType::TIMELINE);
            let info = vk::SemaphoreCreateInfo::default().push_next(&mut info);
            device.device.create_semaphore(&info, GLOBAL).unwrap()
        };
        AnyArc::new_cyclic(|v| Self {
            _this: v.clone(),
            _device: device.this.clone(),
            queue_type,
            handle,
            info,
            submit_lock: Mutex::new(()),
            semaphore,
            last_submitted_index: Default::default(),
            last_completed_index: Default::default(),
            in_flight: ArrayQueue::new(256),
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
}

impl IQueue for Queue {
    fn upgrade(&self) -> AnyArc<dyn IQueue> {
        AnyArc::map::<dyn IQueue, _>(self._this.upgrade().unwrap(), |v| v)
    }

    fn strong_count(&self) -> usize {
        self._this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self._this.weak_count()
    }

    fn queue_properties(&self) -> QueueProperties {
        let v = self.info.min_image_transfer_granularity;
        let min_image_transfer_granularity = Extent3D::new(v.width, v.height, v.depth);

        QueueProperties {
            min_image_transfer_granularity,
        }
    }

    fn garbage_collect(&self) {
        let device = self._device.upgrade().unwrap();

        // Grab the index of the most recently completed command list on this queue and update
        // the queue's value
        //
        // Like in 'wait_idle' we need an atomic CAS loop to uphold monotonicity guarantees. There
        // is a window between the GetCompletedValue call and the atomic store for thread
        // preemption to allow another thread to write in a newer 'GetCompletedValue' with a
        // higher index before the initial thread gets a chance to write its lower index. Eventually
        // the initial thread will get execution back and overwrite the higher index with the lower
        // index it captured before being preempted.
        //
        // Atomics are 'fun'.
        let last_completed = loop {
            let old_last_completed = self.last_completed_index.load(Ordering::Relaxed);
            let new_last_completed = unsafe {
                device
                    .timeline_semaphore
                    .get_semaphore_counter_value(self.semaphore)
                    .unwrap()
            };
            match self.last_completed_index.compare_exchange(
                old_last_completed,
                new_last_completed,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => break new_last_completed,
                Err(_) => continue,
            }
        };

        // Capture the current length of the queue. We then pop N items off the queue and check
        // to see if it is complete based on comparing the list's index with the last completed
        // index. If the list is done we drop it to release any resources that it was keeping
        // alive.
        let num = self.in_flight.len();
        for _ in 0..num {
            // Check if the
            let v = self.in_flight.pop().unwrap();
            if v.index > last_completed {
                self.in_flight
                    .push(v)
                    .ok()
                    .expect("Overflowed in-flight command list tracking buffer");
            } else {
                // Now that we know the submission is complete we can return the swap semaphore
                // (if there is one) to the semaphore pool
                if !v.swap_ready_semaphore.is_null() {
                    let device = self._device.upgrade().unwrap();
                    device.swap_semaphore_pool.push(v.swap_ready_semaphore);
                }

                if let Some(pool) = v.swap_work_semaphore_pool.as_deref() {
                    assert!(!v.swap_work_semaphore.is_null());
                    pool.push(v.swap_work_semaphore);
                }

                // Grab the pool for the specific queue type. Don't want to get different
                // classes of command list mixed up!
                let pool_target = device
                    .command_list_pool
                    .get_pool_for_queue_type(self.queue_type);

                for mut list in v.lists.into_iter() {
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
                            log::warn!("CommandList free-object-pool overflowing!");
                            dropped.collect(&device);
                        }
                    }
                }
            }
        }
    }

    fn wait_idle(&self) {
        let device = self._device.upgrade().unwrap();

        unsafe {
            let _lock = self.submit_lock.lock();
            device.device.queue_wait_idle(self.handle).unwrap();
        }
    }

    unsafe fn submit(&self, desc: &QueueSubmitDesc) -> Result<(), QueueSubmitError> {
        let device = self._device.upgrade().unwrap();

        let mut manager = SubmissionManager::new(desc);
        let submission = manager.prepare_submission(self, desc)?;

        let mut timeline_info = manager.timeline_info();
        let info = manager.submit_info(&mut timeline_info);

        // Signal the fence, if one is provided, to let CPU know the submitted commands are
        // now fully retired.
        let fence = desc
            .fence
            .map(Fence::get)
            .map(|v| v.fence)
            .unwrap_or(vk::Fence::null());

        unsafe {
            let _lock = self.submit_lock.lock();
            device
                .device
                .queue_submit(self.handle, &[info], fence)
                .map_err(|v| log::error!("Platform Error: {:#?}", v))?;
        }

        // TODO: we want to do some garbage collection for resources
        self.in_flight
            .push(submission)
            .ok()
            .expect("Overflowed in-flight submission tracking buffer");

        Ok(())
    }

    unsafe fn present(&self, swap_image: AnyArc<dyn ISwapImage>) -> Result<(), QueuePresentError> {
        let device = self._device.upgrade().unwrap();
        let loader = device.swapchain.as_ref().unwrap();

        let mut swap_image = {
            let v = swap_image;
            unwrap::swap_image_owned(v)
        };
        let swap_image = AnyArc::get_mut(&mut swap_image).unwrap();

        let swap_chain = &swap_image.swap_chain;

        // Checks if the queue supports present operations. While this could use a debug_assert
        // instead like other validation code, the cost of this check compared to the cost of the
        // present call is tiny.
        if !swap_chain.present_supported_on_queue(self.queue_type) {
            return Err(QueuePresentError::QueuePresentationNotSupported(
                self.queue_type,
            ));
        }

        let result = unsafe {
            let swap_chain = swap_chain.inner.lock();

            let mut wait_semaphores = BVec::new_in(Default::default());
            std::mem::swap(
                Mutex::get_mut(&mut swap_image.work_semaphores),
                &mut wait_semaphores,
            );
            let swapchains = [swap_chain.swap_chain];
            let image_indices = [swap_image.index];
            let info = vk::PresentInfoKHR::default()
                .wait_semaphores(&wait_semaphores)
                .swapchains(&swapchains)
                .image_indices(&image_indices);

            {
                let _lock = self.submit_lock.lock();
                loader.queue_present(self.handle, &info)
            }
        };

        match result {
            Ok(false) => Ok(()),
            Ok(true) => Err(QueuePresentError::SubOptimal),
            Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => Err(QueuePresentError::OutOfDate),
            Err(vk::Result::ERROR_SURFACE_LOST_KHR) => Err(QueuePresentError::SurfaceLost),
            Err(e) => {
                // Coerce everything we don't explicitly handle to an error.
                log::error!("Platform Error: {:#?}", e);
                Err(QueuePresentError::Platform)
            }
        }
    }
}

#[derive(Clone)]
pub struct QueueInfo {
    pub family_index: u32,
    pub min_image_transfer_granularity: vk::Extent3D,
    pub _timestamp_valid_bits: u32,
    pub _sparse_binding: bool,
}

impl QueueInfo {
    pub fn new(family_index: u32, family: &vk::QueueFamilyProperties) -> Self {
        Self {
            family_index,
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
    pub swap_work_semaphore: vk::Semaphore,

    /// Handle to the semaphore pool we have to place the 'work' semaphores into. These will be
    /// waited by a present call which has some very inconvenient properties so they must be handled
    /// separately than the ready semaphores.
    pub swap_work_semaphore_pool: Option<Arc<SemaphorePool>>,

    /// The index of the queue submission. Used for tracking when the work has been retired
    pub index: u64,

    /// We separate the command lists in the submission into their own list so they can be easily
    /// filtered out and recycled later
    pub lists: BVec<Box<CommandList>, RhiSystem>,
}

pub struct SubmissionManager<'a> {
    pub wait_semaphores: BVec<vk::Semaphore, RhiSystem>,
    pub wait_values: BVec<u64, RhiSystem>,
    pub wait_dst_stage_masks: BVec<vk::PipelineStageFlags, RhiSystem>,
    pub signal_semaphores: BVec<vk::Semaphore, RhiSystem>,
    pub signal_values: BVec<u64, RhiSystem>,
    pub command_buffers: BVec<vk::CommandBuffer, RhiSystem>,
    pub swap_image: Option<&'a SwapImage>,
}

impl<'a> SubmissionManager<'a> {
    /// Constructs a new [`SubmissionSemaphoreManager`] with space in the internal lists reserved
    /// based on the given queue submission description
    pub fn new(desc: &'a QueueSubmitDesc<'a>) -> Self {
        // Reserve space for 1 semaphore for each caller provided 'wait_semaphore'. We add space for
        // one extra semaphore if the caller is attaching a swap image so that we can wait for the
        // image acquisition operation.
        let caller_wait_num = desc.wait_semaphores.len();
        let swap_wait_num = if desc.swap_image.is_some() { 1 } else { 0 };
        let wait_num = caller_wait_num + swap_wait_num;

        // Reserve space for 1 semaphore for each caller provided 'signal_semaphore'. We also add
        // space for an extra internal timeline semaphore that is used by the queue to track what
        // command lists have completed. A further entry is reserved if a swap image is attached to
        // signal a semaphore that the present operation will wait on.
        let caller_signal_num = desc.signal_semaphores.len();
        let internal_signal_num = 1;
        let swap_signal_num = if desc.swap_image.is_some() { 1 } else { 0 };
        let signal_num = caller_signal_num + internal_signal_num + swap_signal_num;

        let wait_semaphores = BVec::with_capacity_in(wait_num, RhiSystem::default());
        let wait_values = BVec::with_capacity_in(wait_num, RhiSystem::default());
        let wait_dst_stage_masks = BVec::with_capacity_in(wait_num, RhiSystem::default());
        let signal_semaphores = BVec::with_capacity_in(signal_num, RhiSystem::default());
        let signal_values = BVec::with_capacity_in(signal_num, RhiSystem::default());
        let command_buffers =
            BVec::with_capacity_in(desc.command_lists.len(), RhiSystem::default());
        let swap_image = desc.swap_image.map(unwrap::swap_image);

        Self {
            wait_semaphores,
            wait_values,
            wait_dst_stage_masks,
            signal_semaphores,
            signal_values,
            command_buffers,
            swap_image,
        }
    }

    pub fn prepare_submission(
        &mut self,
        queue: &Queue,
        desc: &QueueSubmitDesc,
    ) -> Result<QueueSubmission, QueueSubmitError> {
        let mut submission = QueueSubmission {
            swap_ready_semaphore: vk::Semaphore::null(),
            swap_work_semaphore: vk::Semaphore::null(),
            swap_work_semaphore_pool: None,
            index: 0,
            lists: BVec::with_capacity_in(desc.command_lists.len(), RhiSystem::default()),
        };

        self.handle_command_lists(desc, &mut submission)?;
        self.handle_api_semaphores(desc);
        self.handle_internal_semaphores(queue, &mut submission);
        self.handle_swap_semaphores(&mut submission);

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
    fn handle_command_lists(
        &mut self,
        desc: &QueueSubmitDesc,
        submission: &mut QueueSubmission,
    ) -> Result<(), QueueSubmitError> {
        for list in desc.command_lists {
            let list = list.take().unwrap();
            let list = box_downcast::<_, CommandList>(list).ok().unwrap();
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

    /// This function will add all API level semaphores that are provided from the caller. These
    /// are handled differently from the internal semaphores that are used for synchronizing inside
    /// the API implementation.
    fn handle_api_semaphores(&mut self, desc: &QueueSubmitDesc) {
        for semaphore in desc.wait_semaphores {
            let semaphore = Semaphore::get(semaphore);
            self.wait_on_binary_semaphore(semaphore.semaphore);
        }

        for semaphore in desc.signal_semaphores {
            let semaphore = Semaphore::get(semaphore);
            self.signal_binary_semaphore(semaphore.semaphore);
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
    fn handle_swap_semaphores(&mut self, submission: &mut QueueSubmission) {
        if let Some(swap_image) = self.swap_image {
            let device = &swap_image.swap_chain.device.device;

            let mut wait_semaphores = swap_image.work_semaphores.lock();

            // We take the ready semaphore from the swap image, leaving a null semaphore in its
            // place. This way only the first submission on a queue will wait on the semaphore and
            // we don't end up waiting multiple times, which is not correct.
            submission.swap_ready_semaphore = swap_image.take_ready_semaphore();
            if !submission.swap_ready_semaphore.is_null() {
                self.wait_on_binary_semaphore(submission.swap_ready_semaphore);
            }

            submission.swap_work_semaphore = unsafe { swap_image.semaphore_pool.get(device) };
            self.signal_binary_semaphore(submission.swap_work_semaphore);
            wait_semaphores.push(submission.swap_work_semaphore);

            submission.swap_work_semaphore_pool = Some(swap_image.semaphore_pool.clone());
        }
    }

    fn wait_on_binary_semaphore(&mut self, semaphore: vk::Semaphore) {
        self.wait_semaphores.push(semaphore);
        self.wait_values.push(0);
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
