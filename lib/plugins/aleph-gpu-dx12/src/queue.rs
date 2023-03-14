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

use crate::internal::{try_clone_value_into_slot, unwrap};
use crossbeam::queue::ArrayQueue;
use interfaces::any::{AnyArc, AnyWeak, IAny, TraitObject};
use interfaces::anyhow::anyhow;
use interfaces::gpu::*;
use parking_lot::Mutex;
use pix::{begin_event_on_queue, end_event_on_queue, set_marker_on_queue};
use std::any::{Any, TypeId};
use std::mem::transmute;
use std::ptr;
use std::ptr::NonNull;
use std::sync::atomic::{AtomicU64, Ordering};
use windows::Win32::Graphics::Direct3D12::*;
use windows::Win32::Graphics::Dxgi::*;

pub struct Queue {
    pub(crate) this: AnyWeak<Self>,
    pub(crate) queue_type: QueueType,
    pub(crate) handle: ID3D12CommandQueue,

    /// Lock used to serialize submissions to the command queue.
    pub(crate) submit_lock: Mutex<()>,

    /// Flags whether the user is allowed to query the IQueueDebug interface. Support is only
    /// enabled when a debug context is created and the PIX library is linked.
    pub(crate) is_queue_debug_available: bool,

    /// Internal tracker used to mark the depth of the debug marker stack. Used to ensure that the
    /// user doesn't call 'end_event' without an associated 'begin_event'
    pub(crate) debug_marker_depth: AtomicU64,

    /// A special fence used specifically for tracking the work that is in-flight on this queue.
    /// This is signalled and waited using submission indices.
    pub(crate) fence: ID3D12Fence,

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
        try_clone_value_into_slot::<ID3D12CommandQueue>(&self.handle, out, target)
    }
}

impl Queue {
    #[inline]
    pub(crate) fn new(
        device: &ID3D12Device,
        queue_type: QueueType,
        debug: bool,
        handle: ID3D12CommandQueue,
    ) -> AnyArc<Self> {
        unsafe {
            AnyArc::new_cyclic(|v| Self {
                this: v.clone(),
                queue_type,
                handle,
                submit_lock: Mutex::new(()),
                is_queue_debug_available: debug,
                debug_marker_depth: Default::default(),
                fence: device.CreateFence(0, D3D12_FENCE_FLAG_NONE).unwrap(),
                last_submitted_index: Default::default(),
                last_completed_index: Default::default(),
                in_flight: ArrayQueue::new(256),
            })
        }
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
    pub(crate) unsafe fn record_submission_index_signal(&self) -> windows::core::Result<u64> {
        // Get the state of last_submitted_index before and after we increment it
        let old_index = self.last_submitted_index.fetch_add(1, Ordering::Relaxed);
        let new_index = old_index + 1;

        // This performs an overflow check, if old_index is u64::MAX then the addition will have
        // caused an overflow and broken our monotonicity requirement.
        assert_ne!(old_index, u64::MAX, "last_submitted_index integer overflow");

        // Signal new_index, new_index is the submission ID.
        self.handle.Signal(&self.fence, new_index)?;

        log::trace!("New Latest Submission Index: {}", new_index);

        Ok(new_index)
    }
}

impl IQueue for Queue {
    fn upgrade(&self) -> AnyArc<dyn IQueue> {
        AnyArc::map::<dyn IQueue, _>(self.this.upgrade().unwrap(), |v| v)
    }

    fn strong_count(&self) -> usize {
        self.this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self.this.weak_count()
    }

    fn queue_properties(&self) -> QueueProperties {
        QueueProperties {
            min_image_transfer_granularity: Extent3D::new(0, 0, 0),
        }
    }

    fn garbage_collect(&self) {
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
            let new_last_completed = unsafe { self.fence.GetCompletedValue() };
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
        unsafe {
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
            // underneath us we just try again until the thread 'wins' the race.
            //
            // This could theoretically cause wait_idle to loop indefinitely if the above race
            // condition occurs repeatedly, but in practice it will never happen once let alone
            // infinitely. But it is now impossible for 'wait_idle' to cause last_completed_index
            // to go backwards.
            loop {
                let last_completed = self.last_completed_index.load(Ordering::Relaxed);
                let last_submitted = self.last_submitted_index.load(Ordering::Relaxed);
                self.fence
                    .SetEventOnCompletion(last_submitted, None)
                    .unwrap();
                match self.last_completed_index.compare_exchange(
                    last_completed,
                    last_submitted,
                    Ordering::Relaxed,
                    Ordering::Relaxed,
                ) {
                    Ok(_) => break,
                    Err(_) => continue,
                }
            }
        }
    }

    unsafe fn submit(&self, desc: &QueueSubmitDesc) -> Result<(), QueueSubmitError> {
        // Grab the submit lock to prevent concurrent submits. I'm not sure if d3d12 allows
        // concurrent submits from multiple threads but vulkan doesn't so I'll assume d3d12 doesn't
        // either.
        let _lock = self.submit_lock.lock();

        // 'Wait' on all the wait_semaphores in a loop, as we're emulating vulkan like semaphore
        // objects that predicate a submission
        for semaphore in unwrap::semaphore_iter(desc.wait_semaphores) {
            semaphore
                .wait_on_queue(&self.handle)
                .map_err(|v| anyhow!(v))?;
        }

        let handles: Vec<Option<ID3D12CommandList>> = unwrap::command_list_iter(desc.command_lists)
            .map(|v| Some(v.list.clone().into()))
            .collect();

        self.handle.ExecuteCommandLists(&handles);

        // 'Signal' all the 'signal_semaphores' in a loop, as we're emulating vulkan like
        // semaphore objects.
        for semaphore in unwrap::semaphore_iter(desc.signal_semaphores) {
            semaphore
                .signal_on_queue(&self.handle)
                .map_err(|v| anyhow!(v))?;
        }

        // Signal the fence, if one is provided, to let CPU know the submitted commands are
        // now fully retired.
        if let Some(fence) = desc.fence.map(unwrap::fence) {
            fence
                .signal_on_queue(&self.handle)
                .map_err(|v| anyhow!(v))?;
        }

        // Safety: We simply never, ever decrement last_submitted_index. Ever. It's impossible for
        // it to be decremented.
        let index = self
            .record_submission_index_signal()
            .map_err(|v| anyhow!(v))?;

        for handle in handles {
            let _handle = handle.unwrap();
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
        let swap_chain = unwrap::swap_chain(desc.swap_chain);

        // Checks if the queue supports present operations. While this could use a debug_assert
        // instead like other validation code, the cost of this check compared to the cost of the
        // present call is tiny.
        if !swap_chain.present_supported_on_queue(self.queue_type) {
            return Err(QueuePresentError::QueuePresentationNotSupported(
                self.queue_type,
            ));
        }

        // Grab the submit lock to prevent concurrent submits. I'm not sure if d3d12 allows
        // concurrent submits from multiple threads but vulkan doesn't so I'll assume d3d12 doesn't
        // either.
        let _lock = self.submit_lock.lock();

        for semaphore in unwrap::semaphore_iter(desc.wait_semaphores) {
            semaphore
                .wait_on_queue(&self.handle)
                .map_err(|v| anyhow!(v))?;
        }

        let presentation_params = DXGI_PRESENT_PARAMETERS {
            DirtyRectsCount: 0,
            pDirtyRects: std::ptr::null_mut(),
            pScrollRect: std::ptr::null_mut(),
            pScrollOffset: std::ptr::null_mut(),
        };
        swap_chain
            .swap_chain
            .Present1(0, 0, &presentation_params)
            .ok()
            .map_err(|v| anyhow!(v))?;

        if swap_chain
            .acquired
            .compare_exchange(true, false, Ordering::SeqCst, Ordering::SeqCst)
            .is_err()
        {
            panic!("Attempted to present an image without having first acquired one");
        }

        // Safety: We simply never, ever decrement last_submitted_index. Ever. It's impossible for
        // it to be decremented.
        let _index = self
            .record_submission_index_signal()
            .map_err(|v| anyhow!(v))?;

        Ok(())
    }
}

impl IQueueDebug for Queue {
    fn set_marker(&self, color: Color, message: &str) {
        let _lock = self.submit_lock.lock();
        unsafe {
            set_marker_on_queue(&self.handle, color.0.into(), message);
        }
    }

    fn begin_event(&self, color: Color, message: &str) {
        let _lock = self.submit_lock.lock();
        unsafe {
            // Use a counter to track the event stack depth. Prevents mismatched
            // end_event+begin_event pairs.
            let previous_event_depth = self.debug_marker_depth.fetch_add(1, Ordering::Relaxed);
            assert_ne!(
                previous_event_depth,
                u64::MAX,
                "Event Stack Depth overflow. How!!??!?"
            );
            begin_event_on_queue(&self.handle, color.0.into(), message);
        }
    }

    fn end_event(&self) {
        let _lock = self.submit_lock.lock();
        unsafe {
            // Use a counter to track the event stack depth. Prevents mismatched
            // end_event+begin_event pairs.
            let previous_event_depth = self.debug_marker_depth.fetch_sub(1, Ordering::Relaxed);
            assert_ne!(
                previous_event_depth, 0,
                "Event Stack Depth underflow. end_event called without a matching begin_event"
            );
            end_event_on_queue(&self.handle);
        }
    }
}

pub struct QueueSubmission {
    /// The index of the queue submission. Used for tracking when the work has been retired
    pub index: u64,

    /// A list of times to be dropped
    pub items: Vec<Box<dyn Any + Send + Sync>>,
}
