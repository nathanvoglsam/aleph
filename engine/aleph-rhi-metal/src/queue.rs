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
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Weak};

use aleph_rhi_api::*;
use crossbeam::queue::SegQueue;
use objc2::rc::{Retained, autoreleasepool};
use objc2::runtime::ProtocolObject;
use objc2_foundation::ns_string;
use objc2_metal::*;
use parking_lot::{Mutex, MutexGuard};

use crate::command_list::{CommandList, ListState};
use crate::device::{Device, FreeCommandList};
use crate::fence::Fence;
use crate::internal::unwrap;

pub struct Queue {
    pub(crate) _this: Weak<Self>,
    pub(crate) _device: Weak<Device>,
    pub(crate) queue_type: QueueType,

    pub(crate) objects: QueueObjects,

    /// The index of the most recent submission to the queue.
    ///
    /// Used to track which submissions are in-flight, used in conjunction with
    /// [Queue::last_completed_index].
    pub(crate) last_submitted_index: AtomicU64,

    /// The index of the submission that is most recently confirmed to have completed. Used to track
    /// which submissions are in-flight, used in conjunction with [Queue::last_submitted_index].
    pub(crate) last_completed_index: AtomicU64,

    /// Lock used to serialize submissions to the command queue.
    pub(crate) submit_lock: Mutex<()>,

    /// A ring-buffer that tracks all currently in flight queue submissions. This is used in
    /// conjunction with [IQueue::garbage_collect] to track when resources are no longer in use on
    /// the GPU timeline and are safe to destroy.
    pub(crate) in_flight: SegQueue<QueueSubmission>,
}

impl IGetPlatformInterface for Queue {
    unsafe fn __query_platform_interface(&self, _target: TypeId, _out: *mut ()) -> Option<()> {
        None
    }
}

impl Queue {
    #[inline]
    pub(crate) fn new(device: &Device, queue_type: QueueType) -> Option<Arc<Self>> {
        let descriptor = MTL4CommandQueueDescriptor::new();
        match queue_type {
            QueueType::General => descriptor.setLabel(Some(ns_string!("GeneralQueue"))),
            QueueType::Compute => descriptor.setLabel(Some(ns_string!("ComputeQueue"))),
            QueueType::Transfer => descriptor.setLabel(Some(ns_string!("TransferQueue"))),
        }
        let queue = device
            .device
            .newMTL4CommandQueueWithDescriptor_error(&descriptor)
            .ok()?;

        let event = match device.device.newSharedEvent() {
            Some(v) => v,
            None => return None,
        };
        // event.setSignaledValue(0);

        // Add the allocator's residency sets to the pool
        for pool in device.allocator.as_ref().unwrap().pools() {
            queue.addResidencySet(&pool.info().heap_residency.lock().set);
            queue.addResidencySet(&pool.info().dedicated_residency_set.lock().set);
        }

        let out = Arc::new_cyclic(|v| Self {
            _this: v.clone(),
            _device: device.this.clone(),
            queue_type,
            objects: QueueObjects { queue, event },
            last_submitted_index: AtomicU64::new(0),
            last_completed_index: AtomicU64::new(0),
            submit_lock: Mutex::new(()),
            in_flight: SegQueue::new(),
        });

        Some(out)
    }
}

impl IQueue for Queue {
    fn upgrade(&self) -> Arc<dyn IQueue> {
        self._this.upgrade().unwrap()
    }

    fn strong_count(&self) -> usize {
        self._this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self._this.weak_count()
    }

    fn queue_properties(&self) -> QueueProperties {
        QueueProperties {
            min_image_transfer_granularity: Extent3D::default(),
        }
    }

    fn garbage_collect(&self) -> Result<(), QueueGarbageCollectError> {
        // Lock access to the queue to ensure nobody submits while we're running the GC cycle.
        let _lock = self.submit_lock.lock();
        autoreleasepool(|_| self.garbage_collect_internal())
    }

    fn wait_idle(&self) -> Result<(), QueueWaitError> {
        // Lock access to the queue to ensure nobody submits while we're waiting for all outstanding
        // work to complete
        let _lock = self.submit_lock.lock();
        autoreleasepool(|_| self.wait_idle_internal())
    }

    unsafe fn submit(&self, desc: &QueueSubmitDesc) -> Result<(), QueueSubmitError> {
        autoreleasepool(|_| {
            let _lock = self.submit_lock.lock();

            assert!(
                !desc.command_lists.is_empty(),
                "Can't call IQueue::submit with zero command buffers!"
            );

            // Flush any changes to the residency sets maintained by our resource allocator.
            //
            // This is the best safe point to commit the changes. All new resources will be in these
            // sets by the time a submit will take place. Any resource that was removed from the
            // set since last submit can't be used on the GPU because it's unsound to destroy it on
            // the CPU while in use on the GPU.
            //
            // We're also safe from another thread adding/removing resources mid submit.
            // - New resources can't be used without another call to submit
            // - Destroyed resources can't be in use in the submission because that would invoke
            //   user after free bugs. You would have to submit work using that resource without
            //   waiting for it to complete.
            let device = self._device.upgrade().unwrap();
            let allocator = device.allocator.as_ref().unwrap();
            assert!(
                allocator.pools().len() <= 14,
                "Can't register more than 28 residency sets from the allocator"
            );
            for pool in allocator.pools() {
                aleph_profile::scope_named!("MTLResidencySet::commit");
                pool.info().heap_residency.lock().commit_if_dirty();
                pool.info().dedicated_residency_set.lock().commit_if_dirty();
            }

            let lists: Vec<_> = desc
                .command_lists
                .iter()
                .map(|list| {
                    let list = list.take().unwrap();
                    if list.as_ref().type_id() == TypeId::of::<CommandList>() {
                        let ptr = Box::into_raw(list);
                        unsafe { Box::from_raw(ptr.cast::<CommandList>()) }
                    } else {
                        panic!("Unknown ICommandList implementation")
                    }
                })
                .collect();

            // First we need to have this work wait for the drawable to be safe to use.
            if let Some(swap_image) = desc.swap_image {
                let swap_image = unwrap::swap_image(swap_image);
                self.objects
                    .queue
                    .waitForDrawable(swap_image.objects.drawable.as_ref());
            }

            // And then we need to wait on all the fences the caller has requested to wait on too.
            if !desc.wait_fences.is_empty() {
                let iter = desc
                    .wait_fences
                    .iter()
                    .copied()
                    .zip(desc.wait_values.iter().copied());
                for (fence, value) in iter {
                    let fence = Fence::get(fence);
                    self.objects
                        .queue
                        .waitForEvent_value(fence.objects.event.as_ref(), value);
                }
            }

            let mut submission_bundle = Vec::new();
            for list in &lists {
                assert_eq!(list.list_type, self.queue_type);
                assert_eq!(list.state, ListState::Closed);

                let handle: &ProtocolObject<dyn MTL4CommandBuffer> = list.objects.list.as_ref();
                submission_bundle.push(NonNull::from_ref(handle));
            }
            unsafe {
                let bundle_ptr = submission_bundle.as_mut_ptr();
                let bundle_ptr = NonNull::new(bundle_ptr).unwrap();
                self.objects
                    .queue
                    .commit_count(bundle_ptr, submission_bundle.len());
            }

            // And then after we've committed our buffers we should signal all the fences the caller
            // asked to be signaled.
            let iter = desc
                .signal_fences
                .iter()
                .copied()
                .zip(desc.signal_values.iter().copied());
            for (fence, value) in iter {
                let fence = Fence::get(fence);
                self.objects
                    .queue
                    .signalEvent_value(fence.objects.event.as_ref(), value);
            }

            let index = self.record_submission_index_signal();

            self.in_flight.push(QueueSubmission { index, lists });

            Ok(())
        })
    }

    unsafe fn present(&self, swap_image: Arc<dyn ISwapImage>) -> Result<(), QueuePresentError> {
        autoreleasepool(|_| {
            let _lock = self.submit_lock.lock();

            let mut swap_image = unwrap::swap_image_owned(swap_image);
            let swap_image = Arc::get_mut(&mut swap_image).unwrap();

            // We consume the swap image here. Once we enter this function it is no longer legal to
            // encode commands for the swap image. All the commands that access the image must also
            // have been submitted by now. All work referencing the image is on the queue, so we can
            // tell the queue to signal the drawable here.
            self.objects
                .queue
                .signalDrawable(swap_image.objects.drawable.as_ref());
            swap_image.objects.drawable.present();

            Ok(())
        })
    }
}

impl Queue {
    pub fn wait_idle_internal(&self) -> Result<(), QueueWaitError> {
        // This function may run in parallel with submissions and wait_idle/garbage_collect
        // calls from any number of other threads. A naive implementation could allow a data
        // race to break the monotonicity property of last_completed_index and
        // last_submitted_index.
        //
        // It is possible for a thread to be preempted immediately after the atomic load of
        // 'last_submitted_index', then another submission occurs and another thread calls and
        // completes a 'wait_idle' before the original thread gets control back. In this case
        // the original thread will wait for an older 'last_submitted' value and then update
        // the 'last_submitted_index' value with an outdated submission index. This would allow
        // last_submitted_index to decrease, which breaks the monotonicity property.
        //
        // This would likely never happen in a real program as you would need to have a thread
        // get preempted for enough time for both the CPU to submit another command list on
        // another thread and then perform a full pipeline flush on the queue. It would also
        // need to preempt exactly between the atomic loads and SetEventOnCompletion. The odds
        // are astronomically small, but we can avoid the problem.
        //
        // By using a compare_exchange we can check if 'last_completed_index' is still the value
        // we captured before 'SetEventOnCompletion'. If another thread has changed this from
        // underneath us, we just try again until the thread 'wins' the race.
        //
        // This could theoretically cause wait_idle to loop indefinitely if the above race
        // condition occurs repeatedly, but in practice it will never happen once let alone
        // infinitely. But it is now impossible for 'wait_idle' to cause last_completed_index
        // to go backwards.
        loop {
            let last_completed = self.last_completed_index.load(Ordering::Relaxed);
            let last_submitted = self.last_submitted_index.load(Ordering::Relaxed);
            let result = self
                .objects
                .event
                .waitUntilSignaledValue_timeoutMS(last_submitted, u64::MAX);
            assert!(result);
            match self.last_completed_index.compare_exchange(
                last_completed,
                last_submitted,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => break Ok(()),
                Err(_) => continue,
            }
        }
    }

    pub fn garbage_collect_internal(&self) -> Result<(), QueueGarbageCollectError> {
        let device = self._device.upgrade().unwrap();

        // Grab the index of the most recently completed command list on this queue and update
        // the queue's value
        //
        // Like in 'wait_idle' we need an atomic CAS loop to uphold monotonicity guarantees. There
        // is a window between GetCompletedValue and the atomic store for thread preemption to allow
        // another thread to write in a newer 'GetCompletedValue' with a higher index before the
        // initial thread gets a chance to write its lower index. Eventually the initial thread will
        // get execution back and overwrite the higher index with the lower index it captured before
        // being preempted.
        //
        // Atomics are 'fun'.
        let last_completed = loop {
            let old_last_completed = self.last_completed_index.load(Ordering::Relaxed);
            let new_last_completed = self.objects.event.signaledValue();
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
            let v = self.in_flight.pop().unwrap();
            if v.index > last_completed {
                self.in_flight.push(v);
            } else {
                // If the submission is complete we recycle the command lists
                //
                // Grab the pool for the specific queue type. Don't want to get different
                // classes of command list mixed up!
                let pool_target = device
                    .command_list_pool
                    .get_pool_for_queue_type(self.queue_type);

                for list in v.lists.into_iter() {
                    debug_assert_eq!(list.list_type, self.queue_type);

                    // Take the pool and buffer out of the CommandList object so they don't
                    // get dropped. We destroy the Box<CommandList> because it also contains
                    // a back reference to device.
                    //
                    // If we store it inside device then we create a reference cycle and leak
                    // Device. Not great...
                    let list = FreeCommandList {
                        allocator: list.objects.allocator,
                        list: list.objects.list,
                        argument_table: list.objects.argument_table,
                        list_type: list.list_type,
                    };

                    // If we fill the list we just start destroying command lists rather than
                    // growing the pool.
                    if pool_target.push(list).is_err() {
                        log::warn!("CommandList free-object-pool overflowing!");
                    }
                }
            }
        }
        Ok(())
    }

    /// Inserts a Signal operation onto the ID3D12Queue that signals 'self.fence' with the ID of the
    /// most recent submission. This is part of our queue work tracking and is needed to implement
    /// wait_idle as D3D12 doesn't provide a magic 'wait_idle' function like Vulkan does with
    /// vkQueueWaitIdle.
    ///
    /// This is intended to be used in queue submissions to flag 'self.fence' to be signaled when
    /// the submission is complete.
    ///
    /// # Safety
    ///
    /// It is the caller's responsibility to ensure that 'last_submitted_index' is only ever
    /// monotonically increased. This calls ID3D12Queue::Signal with self.fence. An ID3D12Fence's
    /// counter must always be signaled monotonically.
    ///
    /// This function can't trigger the condition on its own, as it uses a fetch_add to acquire the
    /// signal value and never decrements the index, but requires the caller to ensure they never
    /// allow last_submitted_index to be decremented for the program to remain sound.
    pub(crate) fn record_submission_index_signal(&self) -> u64 {
        // Get the state of last_submitted_index before and after we increment it
        let old_index = self.last_submitted_index.fetch_add(1, Ordering::Relaxed);
        let new_index = old_index + 1;

        // This performs an overflow check, if old_index is u64::MAX then the addition will have
        // caused an overflow and broken our monotonicity requirement.
        assert_ne!(old_index, u64::MAX, "last_submitted_index integer overflow");

        self.objects
            .queue
            .signalEvent_value(self.objects.event.as_ref(), new_index);

        new_index
    }

    pub fn submit_lock(&self) -> MutexGuard<'_, ()> {
        self.submit_lock.lock()
    }
}

pub struct QueueSubmission {
    /// The index of the queue submission. Used for tracking when the work has been retired
    pub index: u64,

    /// We separate the command lists in the submission into their own list so they can be easily
    /// filtered out and recycled later
    pub lists: Vec<Box<CommandList>>,
}

/// Wrapper over the MTL objects to limit scope of our 'unsafe impl Send+Sync'
pub struct QueueObjects {
    pub queue: Retained<ProtocolObject<dyn MTL4CommandQueue>>,
    pub event: Retained<ProtocolObject<dyn MTLSharedEvent>>,
}

// Safety: Needed for 'MTLCommandQueue'
unsafe impl Send for Queue {}
unsafe impl Sync for Queue {}
