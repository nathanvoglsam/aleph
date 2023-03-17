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

use crate::device::Device;
use crate::internal::{try_clone_value_into_slot, unwrap};
use crossbeam::queue::ArrayQueue;
use erupt::{vk, ExtendableFrom};
use interfaces::any::{AnyArc, AnyWeak, IAny, TraitObject};
use interfaces::anyhow::anyhow;
use interfaces::gpu::*;
use parking_lot::Mutex;
use std::any::{Any, TypeId};
use std::ffi::CString;
use std::mem::transmute;
use std::ops::Deref;
use std::ptr;
use std::ptr::NonNull;
use std::sync::atomic::{AtomicU64, Ordering};

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

    /// Flags whether the user is allowed to query the IQueueDebug interface. Support is only
    /// enabled when a debug context is created.
    pub(crate) is_queue_debug_available: bool,

    /// Internal tracker used to mark the depth of the debug marker stack. Used to ensure that the
    /// user doesn't call 'end_event' without an associated 'begin_event'.
    pub(crate) debug_marker_depth: AtomicU64,

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
    fn __query_interface(&self, target: TypeId) -> Option<TraitObject> {
        unsafe {
            if target == TypeId::of::<dyn IQueue>() {
                return Some(transmute(self as &dyn IQueue));
            }
            if target == TypeId::of::<dyn IQueueDebug>() && self.is_queue_debug_available {
                return Some(transmute(self as &dyn IQueueDebug));
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
        try_clone_value_into_slot::<vk::Queue>(&self.handle, out, target)
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
        let is_queue_debug_available = device.context.instance_loader.enabled().ext_debug_utils;
        AnyArc::new_cyclic(|v| Self {
            _this: v.clone(),
            _device: device.this.clone(),
            queue_type,
            handle,
            info,
            submit_lock: Mutex::new(()),
            is_queue_debug_available,
            debug_marker_depth: Default::default(),
            semaphore: vk::Semaphore::null(),
            last_submitted_index: Default::default(),
            last_completed_index: Default::default(),
            in_flight: ArrayQueue::new(256),
        })
    }

    /// Inserts a Signal operation onto the ID3D12Queue that signals self.fence with the ID of the
    /// most recent submission. This is part of our queue work tracking and is needed to implement
    /// wait_idle as D3D12 doesn't provide a magic 'wait_idle' function like Vulkan does with
    /// vkQueueWaitIdle.
    ///
    /// This is intended to be used in queue submissions to flag self.fence to be signalled when
    /// the submission is complete.
    ///
    /// # Safety
    ///
    /// It is the caller's responsibility to ensure that 'last_submitted_index' is only ever
    /// monotonically increased. This calls ID3D12Queue::Signal with self.fence. An ID3D12Fence's
    /// counter must always be signalled monotonically.
    ///
    /// This function can't trigger the condition on its own, as it uses a fetch_add to acquire the
    /// signal value and never decrements the index, but requires the caller to ensure they never
    /// allow last_submitted_index to be decremented for the program to remain sound.
    pub(crate) fn next_submission_index(&self) -> u64 {
        // Get the state of last_submitted_index before and after we increment it
        let old_index = self.last_submitted_index.fetch_add(1, Ordering::Relaxed);
        let new_index = old_index + 1;

        // This performs an overflow check, if old_index is u64::MAX then the addition will have
        // caused an overflow and broken our monotonicity requirement.
        assert_ne!(old_index, u64::MAX, "last_submitted_index integer overflow");

        log::trace!("New Latest Submission Index: {}", new_index);

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
                    .device_loader
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
            }
        }
    }

    fn wait_idle(&self) {
        let device = self._device.upgrade().unwrap();

        unsafe {
            let _lock = self.submit_lock.lock();
            device.device_loader.queue_wait_idle(self.handle).unwrap();
        }
    }

    unsafe fn submit(&self, desc: &QueueSubmitDesc) -> Result<(), QueueSubmitError> {
        let device = self._device.upgrade().unwrap();

        // Translate the wait semaphore info
        let mut wait_semaphores = Vec::with_capacity(desc.wait_semaphores.len());
        let mut wait_values = Vec::with_capacity(desc.wait_semaphores.len());
        let mut wait_dst_stage_masks = Vec::with_capacity(desc.wait_semaphores.len());
        for semaphore in unwrap::semaphore_iter(desc.wait_semaphores) {
            wait_semaphores.push(semaphore.semaphore);
            wait_values.push(0);
            wait_dst_stage_masks.push(vk::PipelineStageFlags::ALL_COMMANDS);
        }

        // Translate the signal semaphore info.
        let mut signal_semaphores = Vec::with_capacity(desc.signal_semaphores.len() + 1);
        let mut signal_values = Vec::with_capacity(desc.signal_semaphores.len() + 1);
        for semaphore in unwrap::semaphore_iter(desc.signal_semaphores) {
            signal_semaphores.push(semaphore.semaphore);
            signal_values.push(0);
        }

        // We reserved space for one extra signal semaphore, which is our special timeline semaphore
        // that we use for tracking which submissions are still in flight on the queue. We add that
        // to the end of the signal list with the next computed submission index.
        let index = self.next_submission_index();
        signal_semaphores.push(self.semaphore);
        signal_values.push(index);

        let command_buffers: Vec<vk::CommandBuffer> = unwrap::command_list_iter(desc.command_lists)
            .map(|v| v.buffer)
            .collect();

        // Signal the fence, if one is provided, to let CPU know the submitted commands are
        // now fully retired.
        let fence = desc
            .fence
            .map(unwrap::fence)
            .map(|v| v.fence)
            .unwrap_or(vk::Fence::null());

        let mut timeline_info = vk::TimelineSemaphoreSubmitInfoBuilder::new()
            .wait_semaphore_values(&wait_values)
            .signal_semaphore_values(&signal_values);
        let info = vk::SubmitInfoBuilder::new()
            .wait_semaphores(&wait_semaphores)
            .wait_dst_stage_mask(&wait_dst_stage_masks)
            .signal_semaphores(&signal_semaphores)
            .command_buffers(&command_buffers)
            .extend_from(&mut timeline_info);

        {
            let _lock = self.submit_lock.lock();
            device
                .device_loader
                .queue_submit(self.handle, &[info], fence)
                .map_err(|v| anyhow!(v))?;
        }

        for _ in command_buffers {
            // TODO: we want to do some garbage collection for resources
            self.in_flight
                .push(QueueSubmission {
                    index,
                    items: Vec::new(),
                })
                .ok()
                .expect("Overflowed in-flight submission tracking buffer");
        }

        Ok(())
    }

    unsafe fn present(&self, desc: &QueuePresentDesc) -> Result<(), QueuePresentError> {
        let device = self._device.upgrade().unwrap();
        let swap_chain = unwrap::swap_chain(desc.swap_chain);

        // Checks if the queue supports present operations. While this could use a debug_assert
        // instead like other validation code, the cost of this check compared to the cost of the
        // present call is tiny.
        if !swap_chain.present_supported_on_queue(self.queue_type) {
            return Err(QueuePresentError::QueuePresentationNotSupported(
                self.queue_type,
            ));
        }

        let mut wait_semaphores = Vec::with_capacity(desc.wait_semaphores.len());
        for semaphore in unwrap::semaphore_iter(desc.wait_semaphores) {
            wait_semaphores.push(semaphore.semaphore);
        }

        unsafe {
            let mut swap_chain = swap_chain.inner.lock();

            assert!(
                swap_chain.acquired,
                "Can't present with a swap chain that isn't acquired"
            );

            let swapchains = [swap_chain.swap_chain];
            let image_indices = [desc.image_index];
            let info = vk::PresentInfoKHRBuilder::new()
                .wait_semaphores(&wait_semaphores)
                .swapchains(&swapchains)
                .image_indices(&image_indices);

            {
                let _lock = self.submit_lock.lock();
                device
                    .device_loader
                    .queue_present_khr(self.handle, &info)
                    .map_err(|v| anyhow!(v))?;
            }

            swap_chain.acquired = false;
        }

        let index = self.next_submission_index();
        let info = vk::SemaphoreSignalInfoBuilder::new()
            .semaphore(self.semaphore)
            .value(index);
        device
            .device_loader
            .signal_semaphore(&info)
            .map_err(|v| anyhow!(v))?;

        Ok(())
    }
}

impl IQueueDebug for Queue {
    fn set_marker(&self, color: Color, message: &str) {
        let device = self._device.upgrade().unwrap();
        let _lock = self.submit_lock.lock();
        unsafe {
            if let Some(func) = device.device_loader.queue_insert_debug_utils_label_ext {
                let name = CString::new(message).unwrap();
                let color: [f32; 4] = color.into();
                let info = vk::DebugUtilsLabelEXTBuilder::new()
                    .label_name(&name)
                    .color(color);

                {
                    let _lock = self.submit_lock.lock();
                    (func)(self.handle, info.deref())
                }
            }
        }
    }

    fn begin_event(&self, color: Color, message: &str) {
        let device = self._device.upgrade().unwrap();
        let _lock = self.submit_lock.lock();
        unsafe {
            if let Some(func) = device.device_loader.queue_begin_debug_utils_label_ext {
                // Use a counter to track the event stack depth. Prevents mismatched
                // end_event+begin_event pairs.
                let previous_event_depth = self.debug_marker_depth.fetch_add(1, Ordering::Relaxed);
                assert_ne!(
                    previous_event_depth,
                    u64::MAX,
                    "Event Stack Depth overflow. How!!??!?"
                );

                let name = CString::new(message).unwrap();
                let color: [f32; 4] = color.into();
                let info = vk::DebugUtilsLabelEXTBuilder::new()
                    .label_name(&name)
                    .color(color);

                {
                    let _lock = self.submit_lock.lock();
                    (func)(self.handle, info.deref());
                }
            }
        }
    }

    fn end_event(&self) {
        let device = self._device.upgrade().unwrap();
        let _lock = self.submit_lock.lock();
        unsafe {
            if let Some(func) = device.device_loader.queue_end_debug_utils_label_ext {
                // Use a counter to track the event stack depth. Prevents mismatched
                // end_event+begin_event pairs.
                let previous_event_depth = self.debug_marker_depth.fetch_sub(1, Ordering::Relaxed);
                assert_ne!(
                    previous_event_depth, 0,
                    "Event Stack Depth underflow. end_event called without a matching begin_event"
                );

                {
                    let _lock = self.submit_lock.lock();
                    (func)(self.handle);
                }
            }
        }
    }
}

impl Drop for Queue {
    fn drop(&mut self) {
        let device = self._device.upgrade().unwrap();
        unsafe {
            device.device_loader.queue_wait_idle(self.handle).unwrap();
            device.device_loader.destroy_semaphore(self.semaphore, None);
        }
    }
}

pub struct QueueSubmission {
    /// The index of the queue submission. Used for tracking when the work has been retired
    pub index: u64,

    /// A list of times to be dropped
    pub items: Vec<Box<dyn Any + Send + Sync>>,
}

#[derive(Clone)]
pub struct QueueInfo {
    pub family_index: u32,
    pub min_image_transfer_granularity: vk::Extent3D,
    pub timestamp_valid_bits: u32,
    pub sparse_binding: bool,
}

impl QueueInfo {
    pub fn new(family_index: u32, family: &vk::QueueFamilyProperties) -> Self {
        Self {
            family_index,
            min_image_transfer_granularity: family.min_image_transfer_granularity,
            timestamp_valid_bits: family.timestamp_valid_bits,
            sparse_binding: family.queue_flags.contains(vk::QueueFlags::SPARSE_BINDING),
        }
    }
}
