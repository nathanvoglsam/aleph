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
use std::sync::atomic::{AtomicU64, Ordering};

use aleph_any::{AnyArc, AnyWeak, IAny, TraitObject};
use aleph_rhi_api::*;
use crossbeam::queue::ArrayQueue;
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_metal::*;
use parking_lot::Mutex;

use crate::command_list::CommandList;
use crate::device::Device;

pub struct Queue {
    pub(crate) _this: AnyWeak<Self>,
    pub(crate) _device: AnyWeak<Device>,
    pub(crate) queue_type: QueueType,

    pub(crate) objects: QueueObjects,

    /// Lock used to serialize submissions to the command queue.
    pub(crate) submit_lock: Mutex<()>,

    /// Flags whether the user is allowed to query the IQueueDebug interface. Support is only
    /// enabled when a debug context is created.
    pub(crate) is_queue_debug_available: bool,

    /// Internal tracker used to mark the depth of the debug marker stack. Used to ensure that the
    /// user doesn't call 'end_event' without an associated 'begin_event'.
    pub(crate) debug_marker_depth: AtomicU64,

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
    unsafe fn __query_platform_interface(&self, _target: TypeId, _out: *mut ()) -> Option<()> {
        todo!()
    }
}

impl Queue {
    #[inline]
    pub(crate) fn new(device: &Device, queue_type: QueueType) -> Option<AnyArc<Self>> {
        // TODO: should this be configurable? metal 4 will make this go away anyway I think?
        let queue = device.device.newCommandQueueWithMaxCommandBufferCount(64)?;

        let is_queue_debug_available = false;
        let out = AnyArc::new_cyclic(|v| Self {
            _this: v.clone(),
            _device: device.this.clone(),
            queue_type,
            objects: QueueObjects { queue },
            submit_lock: Mutex::new(()),
            is_queue_debug_available,
            debug_marker_depth: Default::default(),
            last_submitted_index: Default::default(),
            last_completed_index: Default::default(),
            in_flight: ArrayQueue::new(256),
        });

        Some(out)
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
        QueueProperties {
            min_image_transfer_granularity: Extent3D::default(),
        }
    }

    fn garbage_collect(&self) {
        todo!()
    }

    fn wait_idle(&self) {
        todo!()
    }

    unsafe fn submit(&self, desc: &QueueSubmitDesc) -> Result<(), QueueSubmitError> {
        todo!()
    }

    unsafe fn present(&self, desc: &QueuePresentDesc) -> Result<(), QueuePresentError> {
        todo!()
    }
}

impl IQueueDebug for Queue {
    fn set_marker(&self, _color: Color, _message: &aleph_nstr::NStr) {
        // TODO: this
    }

    fn begin_event(&self, _color: Color, _message: &aleph_nstr::NStr) {
        // TODO: this
    }

    fn end_event(&self) {
        // TODO: this
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
    pub queue: Retained<ProtocolObject<dyn MTLCommandQueue>>,
}

// Safety: Needed for 'MTLCommandQueue'
unsafe impl Send for Queue {}
unsafe impl Sync for Queue {}
